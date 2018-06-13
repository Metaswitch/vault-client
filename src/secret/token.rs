//! The `Token` is a `Secret` used only internally to contact Vault. It must be periodically
//! renewed.

use std::cmp;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use futures::Future;
use pinboard::NonEmptyPinboard;
use tokio_core::reactor::Remote;
use tokio_timer;
use vault_api;

use errors::*;
use MAX_LIFETIME;
use secret::{keep_secret_up_to_date, Secret};
use VaultApi;

/// [Tokens](https://www.vaultproject.io/docs/concepts/tokens.html) are used by Vault clients to
/// authenticate themselves against a Vault server. This supports only renewable tokens (i.e.
/// created with `-period="..."`).
#[derive(Debug, Clone)]
pub struct Token {
    /// The actual token that gets sent to Vault.
    token: String,

    /// The time to the next renewal. This should normally be the same as `renew_period`, except if
    /// the token was created with a non-standard period.
    next_renewal: Duration,

    /// The periodicity with which to renew the token.
    renew_period: Duration,

    /// The token is renewed with this lifetime. This means that, should we lose contact with Vault,
    /// we have at least `lifetime - replace_period` to restore contact before the token expires.
    lifetime: Duration,
}

impl Token {
    /// Create a new token.
    pub fn try_new<S: Into<String>>(
        token: S,
        lifetime: Duration,
        renew_period: Duration,
    ) -> Result<Self> {
        // We support lifetimes of up to a year.
        ensure!(
            MAX_LIFETIME > lifetime.as_secs(),
            "Lifetime ({:?}) exceeds maximum ({}s)",
            lifetime,
            MAX_LIFETIME
        );

        // We must renew prior to expiry.
        ensure!(
            lifetime > renew_period,
            "Lifetime of {:?} will expire prior to renewal at {:?}",
            lifetime,
            renew_period
        );

        Ok(Token {
            token: token.into(),
            // Start by forcing a token renewal.
            next_renewal: Duration::new(0, 0),
            renew_period,
            lifetime,
        })
    }

    /// Read the actual value of the token. Be careful as to where this is recorded - it may be a
    /// security leak to write the token value in a log.
    pub fn get_token_str(&self) -> &String {
        &self.token
    }

    /// Helper function to keep a single `Token` updated. The `Token` returned can then be used to
    /// update other `Secret`s.
    pub fn keep_updated<V: 'static + VaultApi + Send + Sync>(
        self,
        client: Arc<V>,
        remote: &Remote,
        cache_path: PathBuf,
    ) -> Arc<NonEmptyPinboard<Self>> {
        // Start keeping the token alive
        let token = Arc::new(NonEmptyPinboard::new(self));

        let timer = tokio_timer::wheel()
            .max_timeout(Duration::from_secs(MAX_LIFETIME))
            .build();

        remote.spawn({
            let token = token.clone();
            let remote = remote.clone();

            move |_| {
                keep_secret_up_to_date(&remote, token.clone(), client, timer, &cache_path, token)
            }
        });

        token
    }
}

impl Secret for Token {
    /// `Token`s are special. We _renew_, rather than _replacing_. This means that the token never
    /// changes, so we don't need to cache it.
    fn get_new<T: Into<String>, V: VaultApi>(
        self,
        client: &V,
        old_token: T,
    ) -> Box<Future<Item = Self, Error = Error> + Send> {
        client
            .renew_own_token(
                old_token.into(),
                vault_api::models::RenewSelfParameters { increment: None },
                &vault_api::Context::new(),
            )
            .then(move |result| match result {
                Ok(vault_api::RenewOwnTokenResponse::Success(
                    vault_api::models::AuthResponse {
                        auth:
                            Some(vault_api::models::AuthResponseAuth {
                                lease_duration: ttl,
                                ..
                            }),
                        ..
                    },
                )) => {
                    // `renew_period` should normally be fine. However, to protect against
                    // misconfiguration, we ensure that we have renewed once 1/3 of the total expiry
                    // time has elapsed.
                    let replace_after =
                        cmp::min(self.renew_period, Duration::from_secs((ttl as u64) / 3));

                    // Renewal means that the token is unchanged. The time to the next renewal is
                    // the only thing that may differ.
                    Ok(Token {
                        next_renewal: replace_after,
                        ..self
                    })
                }
                Ok(resp) => bail!(
                    "Unexpected token replacement response from Vault: {:?}",
                    resp
                ),
                Err(err) => {
                    Err(err).chain_err(|| "Failed to replace token, request to Vault failed")
                }
            })
            .log(|m: &Token| debug!("Got new token: {:?}", m))
    }

    fn get_time_to_replace(&self) -> &Duration {
        &self.next_renewal
    }
}
