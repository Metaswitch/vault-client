//! A simple binary that will just connect to Vault and keep its token up to date.

extern crate futures;
extern crate tokio_core;
extern crate url;
extern crate vault_client;

use vault_client::*;

use std::env;
use std::path::Path;
use std::time::Duration;
use std::sync::Arc;

use futures::Future;
use tokio_core::reactor::Core;
use url::Url;

fn main() {
    let mut core = Core::new().unwrap();

    let token = Token::try_new(
        env::var("VAULT_TOKEN").unwrap(),
        Duration::from_secs(10),
        Duration::from_secs(2),
    ).unwrap();
    let vault_url = Url::parse(&env::var("VAULT_ADDR").unwrap()).unwrap();
    let vault_cert = Path::new("test/certificates/vault.crt");

    // The vault client can use a cache file to persist the token and secrets over service restart.
    // This will pick up any old (non-expired) secrets, and ensure that any new ones are written to
    // the file. It should be accessible only to root.
    let cache_file = Path::new("/tmp/vault-agent");

    // At a minimum, we need the Vault token, IP and X.509 certificate.
    let vault_client = Arc::new(
        Client::try_new(
            &vault_url,
            vault_cert,
            token,
            cache_file,
            core.remote(),
            Duration::from_secs(60),
            Duration::from_secs(360),
        ).unwrap(),
    );

    // Get a new certificate
    let vault_client_clone = vault_client.clone();
    std::thread::spawn(move || loop {
        let result = vault_client_clone.get_certificate("localhost").wait();

        println!("{:?}", result);

        std::thread::sleep(Duration::from_secs(10));
    });

    core.run(futures::empty::<(), ()>()).unwrap();
}
