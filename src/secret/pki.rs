//! Support for the PKI backend. This produces [X509] certificates. The single corresponding CA
//! certificate is handled elsewhere
//!
//! The [`X509`] certificate is a [`Secret`] most commonly used for securing TLS communications. It
//! must be periodically replaced.

use std::sync::Arc;
use std::time::Duration;

use futures::Future;
use openssl;
use vault_api;

use Cache;
use errors::*;
use MAX_LIFETIME;
use secret::{Secret, SecretBuilder};
use VaultApi;

/// Name of the PKI backend used by this registry. This is hard-coded for now as only one PKI
/// registry is supported.
pub static PKI_BACKEND_NAME: &str = "pki";

/// Name of the role used by this registry. This is hard-coded.
static ROLE_NAME: &str = "metaswitch";

/// Constructor for `X509`
pub struct X509Builder {
    /// The common name used for this certificate.
    common_name: String,

    /// The amount of time before this token needs to be replaced.
    replace_after: Duration,

    /// The lifetime we should request for any replacements.
    lifetime: Duration,
}

impl X509Builder {
    /// Create an `X509` certificate from scratch.
    pub fn new(common_name: String, replace_after: Duration, lifetime: Duration) -> Self {
        X509Builder {
            common_name,
            replace_after,
            lifetime,
        }
    }
}

impl SecretBuilder<X509> for X509Builder {
    fn build<T: Into<String>, V: VaultApi>(
        self,
        client: Arc<V>,
        token: T,
    ) -> Box<Future<Item = X509, Error = Error> + Send> {
        X509::try_new(
            self.common_name,
            self.replace_after,
            self.lifetime,
            &*client,
            token.into(),
        )
    }
}

/// X509 certificates, as obtained by use of the
/// [PKI secret backend](https://www.vaultproject.io/docs/secrets/pki/index.html). These are most
/// commonly used to secure TLS transports, such as HTTPS.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct X509 {
    /// The common name used for this certificate.
    pub common_name: String,

    /// The actual certificate.
    pub certificate: String,

    /// CA that issued this certificate.
    pub issuing_ca: String,

    /// Chain of certificates from the issued one to the CA.
    pub ca_chain: Option<Vec<String>>,

    /// Private key for the certificate.
    pub private_key: String,

    /// Type of the private key
    pub private_key_type: String,

    /// Serial number of this certificate
    pub serial_number: String,

    /// The amount of time before this token needs to be replaced.
    pub replace_after: Duration,

    /// The total lifetime that we request for our certificates.
    pub lifetime: Duration,
}

impl X509 {
    /// Helper function to convert a [`vault_api::models::GenerateCertificateResponseData`] into an
    /// [`X509`].
    fn new_from_vault(
        resp: vault_api::models::GenerateCertificateResponseData,
        common_name: String,
        replace_after: Duration,
        lifetime: Duration,
    ) -> Self {
        X509 {
            common_name: common_name,
            certificate: resp.certificate,
            issuing_ca: resp.issuing_ca,
            ca_chain: resp.ca_chain,
            private_key: resp.private_key,
            private_key_type: resp.private_key_type,
            serial_number: resp.serial_number,
            replace_after: replace_after,
            lifetime: lifetime,
        }
    }

    /// Directly build an [`X509`] certificate.
    fn try_new<V: VaultApi>(
        common_name: String,
        replacement_period: Duration,
        lifetime: Duration,
        client: &V,
        token: String,
    ) -> Box<Future<Item = Self, Error = Error> + Send> {
        let cert_params = vault_api::models::GenerateCertificateParameters {
            ttl: Some(lifetime.as_secs().to_string() + "s"),
            ..vault_api::models::GenerateCertificateParameters::new(common_name.clone())
        };

        client
            .generate_cert(
                token,
                PKI_BACKEND_NAME.to_string(),
                ROLE_NAME.to_string(),
                cert_params,
                &vault_api::Context::new(),
            )
            .then(move |result| match result {
                Ok(vault_api::GenerateCertResponse::Success(
                    vault_api::models::GenerateCertificateResponse {
                        data: Some(certificate_response),
                        ..
                    },
                )) => {
                    // We cannot track lifetimes longer than `MAX_LIFETIME`
                    ensure!(
                        lifetime.as_secs() < MAX_LIFETIME,
                        "Excessive ttl: {:?}",
                        lifetime
                    );

                    // We must replace prior to expiry.
                    ensure!(
                        lifetime > replacement_period,
                        "Lifetime of {:?} will expire prior to replacement at {:?}",
                        lifetime,
                        replacement_period
                    );

                    Ok(X509::new_from_vault(
                        certificate_response,
                        common_name,
                        replacement_period,
                        lifetime,
                    ))
                }
                Ok(resp) => bail!(
                    "Unexpected certificate generation response from Vault: {:?}",
                    resp
                ),
                Err(err) => {
                    Err(err).chain_err(|| "Failed to replace certificate, request to Vault failed")
                }
            })
            .log(|m| debug!("Created new certificate: {:?}", m))
    }
}

impl Secret for X509 {
    fn get_new<T: Into<String>, V: VaultApi>(
        self,
        client: &V,
        token: T,
    ) -> Box<Future<Item = Self, Error = Error> + Send> {
        X509::try_new(
            self.common_name.clone(),
            self.replace_after,
            self.lifetime,
            client,
            token.into(),
        ).log(|m| debug!("Got new certificate: {:?}", m))
    }

    fn get_time_to_replace(&self) -> &Duration {
        &self.replace_after
    }

    fn update_cache(&self, cache: &mut Cache) {
        cache
            .certificates
            .insert(self.common_name.clone(), self.clone());
    }
}

/// The type of a non-empty CA chain. This is useful for validating PEM data.
///
/// Note that we don't store these as `X509` objects, because those don't implement `Send`.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct CaChain {
    /// The first element of the CA chain
    root: String,
    /// The rest of the CA chain, from furthest from the root to the closest
    rest: Vec<String>,
}

impl CaChain {
    /// Try to create a `CaChain` from PEM-formatted bytes.
    ///
    /// This also validates the PEM.
    pub fn try_from_pem(pem: &str) -> Result<Self> {
        // First, convert to X509 - this also does validation.
        let mut stack = openssl::x509::X509::stack_from_pem(pem.as_bytes())
            .chain_err(|| "CA chain was not valid PEM")?;
        let root_x509 = stack.pop().ok_or_else(|| "CA chain was empty")?;

        /// Helper function to convert X509 to PEM
        fn x509_to_pem(x509: openssl::x509::X509) -> Result<String> {
            let pem_bytes = x509.to_pem()
                .chain_err(|| "Failed to convert CA chain into bytes")?;
            String::from_utf8(pem_bytes).chain_err(|| "CA chain was not valid UTF8")
        }

        // Next, convert each X509 back to PEM strings.
        let root_pem = x509_to_pem(root_x509)?;
        let rest_pem = stack
            .into_iter()
            .map(x509_to_pem)
            .collect::<Result<Vec<String>>>()?;

        Ok(CaChain {
            root: root_pem,
            rest: rest_pem,
        })
    }

    /// Get the certificate furthest from the root certificate. Usually, this is the issuer's CA
    /// certificate.
    pub fn leaf(self) -> String {
        // Get the top of the `rest`, or else get the root certificate.
        self.rest.into_iter().next().unwrap_or(self.root)
    }

    /// Get the root CA certificate.
    pub fn root(self) -> String {
        self.root
    }
}
