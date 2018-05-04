#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate futures;
extern crate chrono;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, Context, ContextWrapper};


#[derive(Debug, PartialEq)]
pub enum SysLeasesRevokePutResponse {
    Success ,
}

#[derive(Debug, PartialEq)]
pub enum GenerateCertResponse {
    Success ( models::GenerateCertificateResponse ) ,
}

#[derive(Debug, PartialEq)]
pub enum ReadCertResponse {
    Success ( models::CertificateResponse ) ,
}

#[derive(Debug, PartialEq)]
pub enum CreateOrphanTokenResponse {
    Success ( models::AuthResponse ) ,
}

#[derive(Debug, PartialEq)]
pub enum CreateTokenResponse {
    Success ( models::AuthResponse ) ,
}

#[derive(Debug, PartialEq)]
pub enum LogInWithTLSCertificateResponse {
    Success ( models::AuthResponse ) ,
}

#[derive(Debug, PartialEq)]
pub enum RenewOwnTokenResponse {
    Success ( models::AuthResponse ) ,
}


/// API
pub trait Api {

    /// Revoke lease
    fn sys_leases_revoke_put(&self, x_vault_token: String, body: models::RevokeLeaseParameters, context: &Context) -> Box<Future<Item=SysLeasesRevokePutResponse, Error=ApiError> + Send>;

    /// Generate certificate
    fn generate_cert(&self, x_vault_token: String, mount: String, name: String, body: models::GenerateCertificateParameters, context: &Context) -> Box<Future<Item=GenerateCertResponse, Error=ApiError> + Send>;

    /// Read certificate
    fn read_cert(&self, mount: String, serial: String, context: &Context) -> Box<Future<Item=ReadCertResponse, Error=ApiError> + Send>;

    /// Create an orphan token
    fn create_orphan_token(&self, x_vault_token: String, body: models::CreateTokenParameters, context: &Context) -> Box<Future<Item=CreateOrphanTokenResponse, Error=ApiError> + Send>;

    /// Create token
    fn create_token(&self, x_vault_token: String, body: models::CreateTokenParameters, context: &Context) -> Box<Future<Item=CreateTokenResponse, Error=ApiError> + Send>;

    /// Log in
    fn log_in_with_tls_certificate(&self, body: Option<models::AuthCertLoginParameters>, context: &Context) -> Box<Future<Item=LogInWithTLSCertificateResponse, Error=ApiError> + Send>;

    /// Renew own token
    fn renew_own_token(&self, x_vault_token: String, body: models::RenewSelfParameters, context: &Context) -> Box<Future<Item=RenewOwnTokenResponse, Error=ApiError> + Send>;

}

/// API without a `Context`
pub trait ApiNoContext {

    /// Revoke lease
    fn sys_leases_revoke_put(&self, x_vault_token: String, body: models::RevokeLeaseParameters) -> Box<Future<Item=SysLeasesRevokePutResponse, Error=ApiError> + Send>;

    /// Generate certificate
    fn generate_cert(&self, x_vault_token: String, mount: String, name: String, body: models::GenerateCertificateParameters) -> Box<Future<Item=GenerateCertResponse, Error=ApiError> + Send>;

    /// Read certificate
    fn read_cert(&self, mount: String, serial: String) -> Box<Future<Item=ReadCertResponse, Error=ApiError> + Send>;

    /// Create an orphan token
    fn create_orphan_token(&self, x_vault_token: String, body: models::CreateTokenParameters) -> Box<Future<Item=CreateOrphanTokenResponse, Error=ApiError> + Send>;

    /// Create token
    fn create_token(&self, x_vault_token: String, body: models::CreateTokenParameters) -> Box<Future<Item=CreateTokenResponse, Error=ApiError> + Send>;

    /// Log in
    fn log_in_with_tls_certificate(&self, body: Option<models::AuthCertLoginParameters>) -> Box<Future<Item=LogInWithTLSCertificateResponse, Error=ApiError> + Send>;

    /// Renew own token
    fn renew_own_token(&self, x_vault_token: String, body: models::RenewSelfParameters) -> Box<Future<Item=RenewOwnTokenResponse, Error=ApiError> + Send>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: Context) -> ContextWrapper<'a, Self>;
}

impl<'a, T: Api + Sized> ContextWrapperExt<'a> for T {
    fn with_context(self: &'a T, context: Context) -> ContextWrapper<'a, T> {
         ContextWrapper::<T>::new(self, context)
    }
}

impl<'a, T: Api> ApiNoContext for ContextWrapper<'a, T> {

    /// Revoke lease
    fn sys_leases_revoke_put(&self, x_vault_token: String, body: models::RevokeLeaseParameters) -> Box<Future<Item=SysLeasesRevokePutResponse, Error=ApiError> + Send> {
        self.api().sys_leases_revoke_put(x_vault_token, body, &self.context())
    }

    /// Generate certificate
    fn generate_cert(&self, x_vault_token: String, mount: String, name: String, body: models::GenerateCertificateParameters) -> Box<Future<Item=GenerateCertResponse, Error=ApiError> + Send> {
        self.api().generate_cert(x_vault_token, mount, name, body, &self.context())
    }

    /// Read certificate
    fn read_cert(&self, mount: String, serial: String) -> Box<Future<Item=ReadCertResponse, Error=ApiError> + Send> {
        self.api().read_cert(mount, serial, &self.context())
    }

    /// Create an orphan token
    fn create_orphan_token(&self, x_vault_token: String, body: models::CreateTokenParameters) -> Box<Future<Item=CreateOrphanTokenResponse, Error=ApiError> + Send> {
        self.api().create_orphan_token(x_vault_token, body, &self.context())
    }

    /// Create token
    fn create_token(&self, x_vault_token: String, body: models::CreateTokenParameters) -> Box<Future<Item=CreateTokenResponse, Error=ApiError> + Send> {
        self.api().create_token(x_vault_token, body, &self.context())
    }

    /// Log in
    fn log_in_with_tls_certificate(&self, body: Option<models::AuthCertLoginParameters>) -> Box<Future<Item=LogInWithTLSCertificateResponse, Error=ApiError> + Send> {
        self.api().log_in_with_tls_certificate(body, &self.context())
    }

    /// Renew own token
    fn renew_own_token(&self, x_vault_token: String, body: models::RenewSelfParameters) -> Box<Future<Item=RenewOwnTokenResponse, Error=ApiError> + Send> {
        self.api().renew_own_token(x_vault_token, body, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::router;

pub mod models;
