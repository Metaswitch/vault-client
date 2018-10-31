#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "/v1";
pub const API_VERSION: &'static str = "0.7.2";

#[derive(Debug, PartialEq)]
pub enum RevokeLeaseResponse {
    /// Success
    Success
}

#[derive(Debug, PartialEq)]
pub enum GenerateCertResponse {
    /// Success
    Success
    (models::GenerateCertificateResponse)
}

#[derive(Debug, PartialEq)]
pub enum ReadCertResponse {
    /// Success
    Success
    (models::CertificateResponse)
}

#[derive(Debug, PartialEq)]
pub enum CreateOrphanTokenResponse {
    /// Success
    Success
    (models::AuthResponse)
}

#[derive(Debug, PartialEq)]
pub enum CreateTokenResponse {
    /// Success
    Success
    (models::AuthResponse)
}

#[derive(Debug, PartialEq)]
pub enum LogInWithTLSCertificateResponse {
    /// Success
    Success
    (models::AuthResponse)
}

#[derive(Debug, PartialEq)]
pub enum RenewOwnTokenResponse {
    /// Success
    Success
    (models::AuthResponse)
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Revoke lease
    async fn revoke_lease(
        &self,
        x_vault_token: String,
        body: models::RevokeLeaseParameters,
        context: &C) -> Result<RevokeLeaseResponse, ApiError>;

    /// Generate certificate
    async fn generate_cert(
        &self,
        x_vault_token: String,
        mount: String,
        name: String,
        body: models::GenerateCertificateParameters,
        context: &C) -> Result<GenerateCertResponse, ApiError>;

    /// Read certificate
    async fn read_cert(
        &self,
        mount: String,
        serial: String,
        context: &C) -> Result<ReadCertResponse, ApiError>;

    /// Create an orphan token
    async fn create_orphan_token(
        &self,
        x_vault_token: String,
        body: models::CreateTokenParameters,
        context: &C) -> Result<CreateOrphanTokenResponse, ApiError>;

    /// Create token
    async fn create_token(
        &self,
        x_vault_token: String,
        body: models::CreateTokenParameters,
        context: &C) -> Result<CreateTokenResponse, ApiError>;

    /// Log in
    async fn log_in_with_tls_certificate(
        &self,
        body: Option<models::AuthCertLoginParameters>,
        context: &C) -> Result<LogInWithTLSCertificateResponse, ApiError>;

    /// Renew own token
    async fn renew_own_token(
        &self,
        x_vault_token: String,
        body: models::RenewSelfParameters,
        context: &C) -> Result<RenewOwnTokenResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Revoke lease
    async fn revoke_lease(
        &self,
        x_vault_token: String,
        body: models::RevokeLeaseParameters,
        ) -> Result<RevokeLeaseResponse, ApiError>;

    /// Generate certificate
    async fn generate_cert(
        &self,
        x_vault_token: String,
        mount: String,
        name: String,
        body: models::GenerateCertificateParameters,
        ) -> Result<GenerateCertResponse, ApiError>;

    /// Read certificate
    async fn read_cert(
        &self,
        mount: String,
        serial: String,
        ) -> Result<ReadCertResponse, ApiError>;

    /// Create an orphan token
    async fn create_orphan_token(
        &self,
        x_vault_token: String,
        body: models::CreateTokenParameters,
        ) -> Result<CreateOrphanTokenResponse, ApiError>;

    /// Create token
    async fn create_token(
        &self,
        x_vault_token: String,
        body: models::CreateTokenParameters,
        ) -> Result<CreateTokenResponse, ApiError>;

    /// Log in
    async fn log_in_with_tls_certificate(
        &self,
        body: Option<models::AuthCertLoginParameters>,
        ) -> Result<LogInWithTLSCertificateResponse, ApiError>;

    /// Renew own token
    async fn renew_own_token(
        &self,
        x_vault_token: String,
        body: models::RenewSelfParameters,
        ) -> Result<RenewOwnTokenResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Revoke lease
    async fn revoke_lease(
        &self,
        x_vault_token: String,
        body: models::RevokeLeaseParameters,
        ) -> Result<RevokeLeaseResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().revoke_lease(x_vault_token, body, &context).await
    }

    /// Generate certificate
    async fn generate_cert(
        &self,
        x_vault_token: String,
        mount: String,
        name: String,
        body: models::GenerateCertificateParameters,
        ) -> Result<GenerateCertResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().generate_cert(x_vault_token, mount, name, body, &context).await
    }

    /// Read certificate
    async fn read_cert(
        &self,
        mount: String,
        serial: String,
        ) -> Result<ReadCertResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().read_cert(mount, serial, &context).await
    }

    /// Create an orphan token
    async fn create_orphan_token(
        &self,
        x_vault_token: String,
        body: models::CreateTokenParameters,
        ) -> Result<CreateOrphanTokenResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_orphan_token(x_vault_token, body, &context).await
    }

    /// Create token
    async fn create_token(
        &self,
        x_vault_token: String,
        body: models::CreateTokenParameters,
        ) -> Result<CreateTokenResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_token(x_vault_token, body, &context).await
    }

    /// Log in
    async fn log_in_with_tls_certificate(
        &self,
        body: Option<models::AuthCertLoginParameters>,
        ) -> Result<LogInWithTLSCertificateResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().log_in_with_tls_certificate(body, &context).await
    }

    /// Renew own token
    async fn renew_own_token(
        &self,
        x_vault_token: String,
        body: models::RenewSelfParameters,
        ) -> Result<RenewOwnTokenResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().renew_own_token(x_vault_token, body, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
