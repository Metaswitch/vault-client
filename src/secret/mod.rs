//! The [secret backends](https://www.vaultproject.io/docs/secrets/index.html) supported by the
//! client library.
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use futures::Future;
use pinboard::NonEmptyPinboard;
use tokio_core::reactor::Remote;
use tokio_timer::Timer;
use vault_api::Api as VaultApi;

use self::token::Token;
use cache::Cache;
use errors::*;

pub mod pki;
pub mod token;

/// This is an interface for secrets that need to be updated in order to stay valid. Note that said
/// updates may not need to follow any strict periodicity.
pub trait Secret: Debug + Send + Sync + Clone {
    /// Ask the Vault client for a new version of this secret, valid for a longer period.
    fn get_new<T: Into<String>, V: VaultApi>(
        self,
        client: &V,
        token: T,
    ) -> Box<Future<Item = Self, Error = Error> + Send>;

    /// Callers must use this function to work out when `Secret` next needs to be updated.
    fn get_time_to_replace(&self) -> &Duration;

    /// If this secret is stored in the cache, implement this function to update it accordingly when
    /// a new value is obtained.
    fn update_cache(&self, _cache: &mut Cache) {}
}

/// Constructor for `Secrets`. This allows clients of this crate to supply some of the config
/// without needing to worry about e.g. the `VaultApi` client.
pub trait SecretBuilder<S: Secret>: Send {
    /// Construct a `Secret`
    fn build<T: Into<String>, V: VaultApi>(
        self,
        client: Arc<V>,
        token: T,
    ) -> Box<Future<Item = S, Error = Error> + Send>;
}

/// Run though one round of keeping a secret up to date. Wait for a period, then update the
/// secret and kick off another run of this function.
pub fn keep_secret_up_to_date<V, S>(
    remote: &Remote,
    secret: Arc<NonEmptyPinboard<S>>,
    client: Arc<V>,
    timer: Timer,
    cache_path: &PathBuf,
    token: Arc<NonEmptyPinboard<Token>>,
) -> Box<Future<Item = (), Error = ()> + Send>
where
    V: 'static + VaultApi + Send + Sync,
    S: 'static + Secret,
{
    let cache_path = cache_path.to_owned();

    Box::new(
        secret
            .read()
            .get_new(&*client, token.read().get_token_str().to_owned())
            .log(|m| info!("Updated secret: {:?}", m))
            .and_then({
                let cache_path = cache_path.clone();

                move |new_secret| {
                    Cache::update(&cache_path, |cache| new_secret.update_cache(cache))?;
                    Ok(new_secret)
                }
            })
            .map({
                let secret = secret.clone();

                move |new_secret| {
                    // Got the new secret. Overwrite the old one. Note that there are no guarantees here
                    // that we are not accidentally overwriting someone else's changes, as we are not using
                    // locks.
                    secret.set(new_secret.to_owned());
                    new_secret
                }
            })
            .and_then({
                let timer = timer.clone();

                move |new_secret| {
                    let duration = new_secret.get_time_to_replace();

                    // Sleep for the duration of the secret
                    debug!("Sleeping for {:?}", duration);
                    timer
                        .sleep(*duration)
                        .then(|e| e.chain_err(|| "Secret timer error"))
                }
            })
            .and_then({
                let remote = remote.to_owned();
                let timer = timer.clone();
                let secret = secret.clone();
                let client = client.clone();
                let cache_path = cache_path.clone();
                let token = token.clone();

                move |_| {
                    // Spawn a new task which does this all again
                    remote.spawn(move |handle| {
                        keep_secret_up_to_date(
                            handle.remote(),
                            secret,
                            client,
                            timer,
                            &cache_path,
                            token,
                        )
                    });
                    Ok(())
                }
            })
            .or_else({
                let remote = remote.to_owned();

                move |err| {
                    // On failure, retry every 10 minutes.
                    error!("Failure - retrying in 10 minutes: {:?}", err);

                    timer
                        .sleep(Duration::from_secs(60 * 10))
                        .then(|e| e.chain_err(|| "Retry timer error"))
                        .then({
                            move |_| {
                                // Spawn a new task which does this all again
                                remote.spawn(move |handle| {
                                    keep_secret_up_to_date(
                                        handle.remote(),
                                        secret,
                                        client,
                                        timer,
                                        &cache_path,
                                        token,
                                    )
                                });
                                Ok(())
                            }
                        })
                }
            }),
    )
}
