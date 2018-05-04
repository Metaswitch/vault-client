//! A cache is stored on disk. This allows us to persist secrets over restart of the process, which
//! allows the client library to provide service without contacting Vault for a limited period of
//! time.

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use atomicwrites::{AllowOverwrite, AtomicFile};
use serde_json;

use errors::*;
use secret::pki::{CaChain, X509};

/// All secrets that the client library has ever issued will be stored in the cache. Each secret is
/// responsible for keeping its own entry in the cache up to date.
#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Cache {
    /// PKI backend: Certificates
    pub certificates: HashMap<String, X509>,

    /// PKI backend: CA certificate chain, against which the certificates are verified.
    pub ca_certificate_chain: Option<CaChain>,
}

impl Cache {
    /// Loads the current cache. If there is no cache at the path indicated, will return an empty
    /// `Cache`.
    pub fn load(path: &Path) -> Result<Self> {
        if let Ok(file) = File::open(path) {
            serde_json::from_reader(file).chain_err(|| "Unable to parse cache on disk")
        } else {
            Ok(Default::default())
        }
    }

    /// Update the cache with new values. This is atomic.
    pub fn update<F: Fn(&mut Self)>(path: &Path, updater: F) -> Result<()> {
        debug!("Updating cache on disk");

        AtomicFile::new(path, AllowOverwrite)
            .write(|f| {
                let cache_string = {
                    let mut cache = Cache::load(path).unwrap_or_default();
                    updater(&mut cache);
                    serde_json::to_string(&cache).expect("Impossible to fail to serialize cache")
                };

                f.write_all(cache_string.as_bytes())
                    .chain_err(|| "Failed to write cache")
            })
            .chain_err(|| "Failed to update contents of on-disk cache")
            .log(|m| debug!("Successfully updated cache on disk: {:?}", m))
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use std::time::Duration;

    use self::tempfile::NamedTempFile;

    use Cache;
    use secret::Secret;
    use secret::pki::X509;

    fn new_certificate<S: Into<String>>(common_name: S) -> X509 {
        X509 {
            common_name: common_name.into(),
            certificate: "certificate".to_string(),
            issuing_ca: "issuing_ca".to_string(),
            ca_chain: None,
            private_key: "private_key".to_string(),
            private_key_type: "private_key_type".to_string(),
            serial_number: "serial_number".to_string(),
            replace_after: Duration::from_secs(100),
            lifetime: Duration::from_secs(200),
        }
    }

    #[test]
    fn insert() {
        let cache_file = NamedTempFile::new().unwrap();
        let certificate = new_certificate("foo");

        // Write the cache
        Cache::update(cache_file.path(), |cache| certificate.update_cache(cache)).unwrap();

        // `bar` not in cache
        assert!(
            !Cache::load(cache_file.path())
                .unwrap()
                .certificates
                .contains_key("bar"),
            "Unexpected key 'bar' in cache"
        );

        // Add `bar` to cache
        let new_certificate = new_certificate("bar");
        Cache::update(cache_file.path(), |cache| {
            new_certificate.update_cache(cache)
        }).unwrap();
        assert!(
            Cache::load(cache_file.path())
                .unwrap()
                .certificates
                .contains_key("foo"),
            "'foo' not in cache"
        );
        assert!(
            Cache::load(cache_file.path())
                .unwrap()
                .certificates
                .contains_key("bar"),
            "'bar' not in cache"
        );
    }

    #[test]
    fn read_write_symmetry() {
        let cache_file = NamedTempFile::new().unwrap();
        let certificate = new_certificate("foo");

        // Write the cache
        Cache::update(cache_file.path(), |cache| certificate.update_cache(cache)).unwrap();

        // Read and verify that it gives the same certificate.
        assert_eq!(
            Cache::load(cache_file.path()).unwrap().certificates["foo"],
            certificate,
            "Certificate after read/write cycle does not match original"
        );
    }

    #[test]
    fn idempotency() {
        let cache_file = NamedTempFile::new().unwrap();
        let certificate = new_certificate("foo");

        // Write the cache
        Cache::update(cache_file.path(), |cache| certificate.update_cache(cache)).unwrap();

        // Read and verify that it gives the same certificate.
        let old_cache = Cache::load(cache_file.path()).unwrap();
        assert_eq!(
            old_cache.certificates["foo"], certificate,
            "Failed to write expected certificate"
        );

        // Update the cache again.
        Cache::update(cache_file.path(), |cache| certificate.update_cache(cache)).unwrap();

        // Check that it's still the same
        assert_eq!(
            Cache::load(cache_file.path()).unwrap(),
            old_cache,
            "Cache update not idempotent"
        );
    }
}
