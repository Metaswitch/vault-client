//! The [`Registry`] handles keeping the secrets entrusted to it up to date.
//!
//! See [`test::test_run`] for an example of use.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::Future;
use pinboard::NonEmptyPinboard;
use tokio_core::reactor::Remote;
use tokio_timer;
use vault_api;

use errors::*;
use secret::pki::{CaChain, X509, PKI_BACKEND_NAME};
use secret::token::Token;
use secret::{keep_secret_up_to_date, Secret, SecretBuilder};
use Cache;
use {VaultApi, MAX_LIFETIME};

/// The registry itself.
///
/// Keeps track of all the registered secrets of this type.
///
/// The secrets themselves are [`NonEmptyPinboard`]s, so
/// can be written and read at any time without locks, with the following tradeoffs:
/// - multiple writers might miss each others writes, sacrificing consistency
/// - multiple readers might not see all writes, and readers might get slightly out-of date values
/// - each read event causes a clone
///
/// These is fine for us, since once the secret is in place, the only writer is the task making
/// sure it's renewed.
pub(crate) struct Registry<V: VaultApi, S: Secret + 'static> {
    /// The client used to talk to Vault. This is used when we need to renew the secrets.
    client: Arc<V>,

    /// A Tokio reactor remote. The registry uses this to add new secret-handling futures to the
    /// reactor.
    remote: Remote,

    /// Vault credentials used to obtain other secrets. This is kept alive by something else.
    token: Arc<NonEmptyPinboard<Token>>,

    /// The secrets store by this registry. These are kept alive by this registry.
    secrets: Arc<Mutex<HashMap<String, Arc<NonEmptyPinboard<S>>>>>,

    /// The location of the cache where all values are stored on disk.
    cache_path: PathBuf,
}

impl<V: 'static + VaultApi + Send + Sync, S: Secret + 'static> Registry<V, S> {
    /// Create a new `Registry`. The `Remote`'s `Core` must be run for the `Registry` to operate.
    pub fn new(
        client: Arc<V>,
        remote: Remote,
        cache_path: PathBuf,
        initial_token: Arc<NonEmptyPinboard<Token>>,
    ) -> Self {
        Registry {
            client,
            remote,
            token: initial_token,
            secrets: Arc::default(),
            cache_path,
        }
    }

    /// Attempt to get a secret previously `register`ed with the `Registry`.
    pub fn get(&self, key: &str) -> Result<Option<S>> {
        let key = key.to_string();
        let secrets = self.secrets.lock().unwrap();
        Ok(secrets.get(&key).map(|s| s.read()))
    }

    /// Register a secret with the `Registry`. The secret will then be kept up to date (as long as
    /// the core is running).
    pub fn register<N, B>(
        &self,
        secret_name: N,
        secret_builder: B,
    ) -> Box<Future<Item = S, Error = Error> + Send>
    where
        B: 'static + SecretBuilder<S>,
        N: Into<String>,
    {
        let client = self.client.clone();
        let secret_name = secret_name.into();
        let token = self.token.read().get_token_str().clone();

        Box::new(secret_builder.build(client, token).map({
            let timer = tokio_timer::wheel()
                .max_timeout(Duration::from_secs(MAX_LIFETIME))
                .build();

            let cache_path = self.cache_path.clone();
            let client = self.client.clone();
            let remote = self.remote.clone();
            let secret_name = secret_name.clone();
            let token = self.token.clone();
            let secret_store = self.secrets.clone();

            move |secret| {
                // Get the secret storage (`NonEmptyPinboard`) for this secret name,
                // creating it if necessary. If we created it, we should spin up a new task
                // to keep it up to date.
                let mut secret_store = secret_store.lock().unwrap();
                let secret = secret_store
                    .entry(secret_name)
                    .or_insert_with(|| {
                        let new_secret = Arc::new(NonEmptyPinboard::new(secret.clone()));
                        remote.spawn({
                            let secret = new_secret.clone();
                            move |handle| {
                                keep_secret_up_to_date(
                                    handle.remote(),
                                    secret,
                                    client,
                                    timer,
                                    &cache_path,
                                    token,
                                )
                            }
                        });
                        new_secret
                    })
                    .read();
                debug!("Registered {:?}", secret);
                secret
            }
        }))
    }
}

impl<V: 'static + VaultApi + Send + Sync> Registry<V, X509> {
    /// Load X.509 certificates from the cache, and ensure that they are added to the registry.
    /// Certificates already in the registry will **not** be overridden by certificates with the
    /// same common name in the cache (which are ignored).
    pub fn load_cache(self) -> Result<Self> {
        let cache_path = self.cache_path.clone();
        let client = self.client.clone();
        let remote = self.remote.clone();
        let token = self.token.clone();

        {
            let mut secret_store = self.secrets.lock().unwrap();
            let cache = Cache::load(&cache_path)?;
            let timer = tokio_timer::wheel()
                .max_timeout(Duration::from_secs(MAX_LIFETIME))
                .build();

            for (certificate_name, certificate) in cache.certificates {
                let cache_path = cache_path.clone();
                let client = client.clone();
                let remote = remote.clone();
                let timer = timer.clone();
                let token = token.clone();

                secret_store
                    .entry(certificate_name)
                    .or_insert_with(move || {
                        let secret_to_insert = Arc::new(NonEmptyPinboard::new(certificate));
                        remote.spawn({
                            let secret_to_update = secret_to_insert.clone();
                            move |handle| {
                                keep_secret_up_to_date(
                                    handle.remote(),
                                    secret_to_update,
                                    client,
                                    timer,
                                    &cache_path,
                                    token,
                                )
                            }
                        });
                        secret_to_insert
                    });
            }
        }

        Ok(self)
    }

    /// Get the current CA certificate chain
    pub fn get_ca_certificate_chain(&self) -> Box<Future<Item = CaChain, Error = Error> + Send> {
        let cache_path = self.cache_path.clone();

        // Future to get the CA chain from the `ca_chain` endpoint. This usually works, but may fail
        // (with an empty CA chain) if Vault is using a self-signed root CA certificate.
        let from_ca_chain = self.client
            .read_cert(
                PKI_BACKEND_NAME.to_string(),
                "ca_chain".to_string(),
                &vault_api::Context::new(),
            )
            .then(|x| x.chain_err(|| "Did not receive a valid response for the CA chain"))
            .and_then(move |response| match response {
                vault_api::ReadCertResponse::Success(body) => {
                    let ca_certificate_chain = body.data
                        .ok_or("No data found in Vault response")?
                        .certificate;

                    // Validate it. We do this by converting into `CaChain`.
                    CaChain::try_from_pem(&ca_certificate_chain)
                }
            });

        // Future to get the CA chain from the `ca` endpoint. This is used as a backup in case the
        // above was empty or invalid.
        let from_ca = self.client
            .read_cert(
                PKI_BACKEND_NAME.to_string(),
                "ca".to_string(),
                &vault_api::Context::new(),
            )
            .then(|x| x.chain_err(|| "Did not receive a valid response for the CA certificate"))
            .and_then(move |response| match response {
                vault_api::ReadCertResponse::Success(body) => {
                    let ca_certificate = body.data
                        .ok_or("No data found in Vault response")?
                        .certificate;

                    // Validate it. We do this by converting into `CaChain`.
                    CaChain::try_from_pem(&ca_certificate)
                }
            });

        Box::new(
            from_ca_chain
                .or_else(|e| {
                    info!(
                        "Trying to get CA chain using backup endpoint. Reason: {}",
                        e
                    );
                    from_ca
                })
                .and_then(move |ca_chain| {
                    // Cache the new CA certificate chain
                    Cache::update(&cache_path, |cache| {
                        cache.ca_certificate_chain = Some(ca_chain.clone());
                    })?;

                    Ok(ca_chain)
                })
                .map_err(|e| e.chain_err(|| "Failed to get CA certificate chain from Vault")),
        )
    }
}

#[cfg(test)]
mod tests {}
