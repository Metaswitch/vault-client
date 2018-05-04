use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod leases_api;
pub use self::leases_api::{ LeasesApi, LeasesApiClient };
mod pki_backend_api;
pub use self::pki_backend_api::{ PkiBackendApi, PkiBackendApiClient };
mod token_backend_api;
pub use self::token_backend_api::{ TokenBackendApi, TokenBackendApiClient };

pub mod configuration;
pub mod client;
