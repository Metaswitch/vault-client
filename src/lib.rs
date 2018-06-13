//! This is a thick wrapper around the [Vault](https://www.vaultproject.io/docs/index.html) HTTPS
//! API. Tested against Vault 0.7.2. See the API docs or examples for usage.
#![cfg_attr(feature = "cargo-clippy", warn(missing_docs_in_private_items))]
#![warn(missing_docs)]

extern crate atomicwrites;
#[macro_use]
extern crate error_chain;
extern crate futures;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate pinboard;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_timer;
extern crate url;
extern crate vault_api;

use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::{future, Future};
use pinboard::Pinboard;
use tokio_core::reactor::Remote;
use url::Url;
use vault_api::Api as VaultApi;

use cache::Cache;
pub use errors::*;
use registry::Registry;
pub use secret::token::Token;
pub use secret::pki::{CaChain, X509, X509Builder};

mod cache;
mod errors;
mod registry;
mod secret;

/// 1 year
///
/// We're using `tokio_timer` as our timer. We need to set its maximum lifetime (the default is
/// far too low), which will set the maximum lifetime any secret is permitted to have. One year
/// should be more than sufficient.
///
/// Note that this will cause a failure on platforms where `usize` is any narrower than 32 bits
/// since `tokio_timer` tracks the number of time increments (of 100 ms) in a `usize`.
pub static MAX_LIFETIME: u64 = 60 * 60 * 24 * 365;

/// 10 minutes
///
/// We cache Vault's CA certificate. Within the TTL (10 minutes), we just respond from the cache,
/// rather than asking Vault. Otherwise, we respond from the cache only if Vault is unresponsive.
static CA_CERTIFICATE_CACHE_TTL: u64 = 60 * 10;

/// A [Vault](https://www.vaultproject.io/docs/index.html) client. It exposes methods to allow for
/// simple management of its own token and X509 certificates.
pub struct Client<V: VaultApi> {
    /// The registry used to keep [X509] secrets up to date.
    x509_registry: Arc<Registry<V, X509>>,

    /// Maintain a cache of the CA certificate chain.
    ca_certificate_chain_cache: Arc<Pinboard<(CaChain, Instant)>>,

    /// The lifetime with which new certificates will be created.
    certificate_lifetime: Duration,

    /// The periodicity with which certificates will be created. This must be less than the
    /// lifetime.
    certificate_replacement: Duration,
}

impl<V: 'static + VaultApi + Send + Sync> Client<V> {
    /// The main constructor for [`Client`]. Others call into this.
    fn try_new_with_client<T: Into<Token>>(
        initial_token: T,
        cache_path: &Path,
        remote: Remote,
        client: Arc<V>,
        certificate_replacement: Duration,
        certificate_lifetime: Duration,
    ) -> Result<Self> {
        // Start keeping the token alive
        let token = initial_token.into().keep_updated(
            client.clone(),
            remote.clone(),
            cache_path.to_path_buf(),
        );

        // Registry for X509 certificates
        let x509_registry =
            Arc::new(
                Registry::new(client, remote, cache_path.to_path_buf(), token)
                    .load_cache()?
            );

        // Load the CA certificate chain
        let ca_certificate_chain_cache = load_ca_certificate_chain(cache_path)?;

        Ok(Client {
            x509_registry,
            ca_certificate_chain_cache,
            certificate_lifetime,
            certificate_replacement,
        })
    }

    /// Get a new X509 certificate with the specified common name.
    pub fn get_certificate<S: Into<String>>(
        &self,
        common_name: S,
    ) -> Box<Future<Item = X509, Error = Error> + Send> {
        let common_name = common_name.into();

        future::result(self.x509_registry
            // Get the certificate if previously registered.
            .get(&common_name))
            .and_then({
                let x509_registry = self.x509_registry.clone();
                let certificate_lifetime = self.certificate_lifetime;
                let certificate_replacement = self.certificate_replacement;

                move |secret| {
                    secret
                        .map(|s| -> Box<Future<Item=X509, Error=Error> + Send> {
                            Box::new(future::result(Ok(s)))
                        })
                        .unwrap_or_else(|| {
                            // The certificate hasn't yet been registered, so do so now.
                            x509_registry.register(common_name.to_string(),
                                                   X509Builder::new(common_name,
                                                                    certificate_replacement,
                                                                    certificate_lifetime))
                    })
                }
            })
            .log(|m| debug!("Got X.509 certificate: {:?}", m))
    }

    /// Get the root CA certificate. Uses caching logic of CA certificate chain. See
    /// `get_ca_certificate_chain` for details.
    pub fn get_root_certificate(&self) -> Box<Future<Item = String, Error = Error> + Send> {
        Box::new(self.get_ca_certificate_chain().map(CaChain::root))
    }

    /// Get the issuing CA certificate. Uses caching logic of CA certificate chain. See
    /// `get_ca_certificate_chain` for details.
    pub fn get_ca_certificate(&self) -> Box<Future<Item = String, Error = Error> + Send> {
        Box::new(self.get_ca_certificate_chain().map(CaChain::leaf))
    }

    /// Get the CA certificate chain, using the cached value if it's sufficiently recent. On
    /// failure, use the cached version if available, regardless of age.
    pub fn get_ca_certificate_chain(&self) -> Box<Future<Item = CaChain, Error = Error> + Send> {
        let ca_certificate_chain_cache = self.ca_certificate_chain_cache.clone();

        let cached_certificate_chain = match ca_certificate_chain_cache.read() {
            Some((ref chain, t)) if t.elapsed() < Duration::from_secs(CA_CERTIFICATE_CACHE_TTL) => {
                return Box::new(future::ok(chain.clone()));
            }
            Some((chain, _)) => Some(chain),
            None => None,
        };

        self.x509_registry
            .get_ca_certificate_chain()
            .then(move |ca| match ca {
                Ok(chain) => {
                    ca_certificate_chain_cache.set((chain.clone(), Instant::now()));
                    Ok(chain)
                }
                e @ Err(_) => if let Some(chain) = cached_certificate_chain {
                    warn!("Using cached CA certificate chain: {:?}", e);
                    Ok(chain)
                } else {
                    error!("Failed to get CA certificate chain and none are cached");
                    e
                },
            })
            .log(|m| debug!("Got CA certificate: {:?}", m))
    }
}

/// Load the cached CA certificate. If the cache doesn't exist or doesn't contain a CA certificate,
/// return the default value instead (an empty `Pinboard`).
fn load_ca_certificate_chain(cache_path: &Path) -> Result<Arc<Pinboard<(CaChain, Instant)>>> {
    Ok(Arc::new(
        Cache::load(cache_path)?
            .ca_certificate_chain
            .map(|c| Pinboard::new((c, Instant::now())))
            .unwrap_or_default(),
    ))
}

/// A `Client` already filled in with the default Vault connection implementation.
pub type VaultClient = Client<vault_api::client::Client>;

impl VaultClient {
    /// Initialise a new `VaultClient` using a periodic token.
    pub fn try_new<T: Into<Token>>(
        vault_address: &Url,
        vault_certificate: &Path,
        initial_token: T,
        new_cache_location: &Path,
        remote: Remote,
        certificate_replacement: Duration,
        certificate_lifetime: Duration,
    ) -> Result<Self> {
        // Create vault client
        let client = Arc::new(vault_api::client::Client::try_new_https(
            vault_address.to_owned(),
            vault_certificate,
        ).chain_err(|| "Failed to create Vault HTTPS client")?);

        Client::try_new_with_client(
            initial_token,
            new_cache_location,
            remote,
            client,
            certificate_replacement,
            certificate_lifetime,
        )
    }
}
