//! Main library entry point for vault_api implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use openssl::ssl::SslAcceptorBuilder;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use vault_api::models;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        vault_api::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set cerificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = Arc::new(ssl.build());
            let mut tcp_listener = TcpListener::bind(&addr).await.unwrap();
            let mut incoming = tcp_listener.incoming();

            while let (Some(tcp), rest) = incoming.into_future().await {
                if let Ok(tcp) = tcp {
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);
                    let tls_acceptor = Arc::clone(&tls_acceptor);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::accept(&*tls_acceptor, tcp).await.map_err(|_| ())?;

                        let service = service.await.map_err(|_| ())?;

                        Http::new().serve_connection(tls, service).await.map_err(|_| ())
                    });
                }

                incoming = rest;
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use vault_api::{
    Api,
    RevokeLeaseResponse,
    GenerateCertResponse,
    ReadCertResponse,
    CreateOrphanTokenResponse,
    CreateTokenResponse,
    LogInWithTLSCertificateResponse,
    RenewOwnTokenResponse,
};
use vault_api::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Revoke lease
    async fn revoke_lease(
        &self,
        x_vault_token: String,
        body: models::RevokeLeaseParameters,
        context: &C) -> Result<RevokeLeaseResponse, ApiError>
    {
        let context = context.clone();
        info!("revoke_lease(\"{}\", {:?}) - X-Span-ID: {:?}", x_vault_token, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Generate certificate
    async fn generate_cert(
        &self,
        x_vault_token: String,
        mount: String,
        name: String,
        body: models::GenerateCertificateParameters,
        context: &C) -> Result<GenerateCertResponse, ApiError>
    {
        let context = context.clone();
        info!("generate_cert(\"{}\", \"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", x_vault_token, mount, name, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Read certificate
    async fn read_cert(
        &self,
        mount: String,
        serial: String,
        context: &C) -> Result<ReadCertResponse, ApiError>
    {
        let context = context.clone();
        info!("read_cert(\"{}\", \"{}\") - X-Span-ID: {:?}", mount, serial, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Create an orphan token
    async fn create_orphan_token(
        &self,
        x_vault_token: String,
        body: models::CreateTokenParameters,
        context: &C) -> Result<CreateOrphanTokenResponse, ApiError>
    {
        let context = context.clone();
        info!("create_orphan_token(\"{}\", {:?}) - X-Span-ID: {:?}", x_vault_token, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Create token
    async fn create_token(
        &self,
        x_vault_token: String,
        body: models::CreateTokenParameters,
        context: &C) -> Result<CreateTokenResponse, ApiError>
    {
        let context = context.clone();
        info!("create_token(\"{}\", {:?}) - X-Span-ID: {:?}", x_vault_token, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Log in
    async fn log_in_with_tls_certificate(
        &self,
        body: Option<models::AuthCertLoginParameters>,
        context: &C) -> Result<LogInWithTLSCertificateResponse, ApiError>
    {
        let context = context.clone();
        info!("log_in_with_tls_certificate({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Renew own token
    async fn renew_own_token(
        &self,
        x_vault_token: String,
        body: models::RenewSelfParameters,
        context: &C) -> Result<RenewOwnTokenResponse, ApiError>
    {
        let context = context.clone();
        info!("renew_own_token(\"{}\", {:?}) - X-Span-ID: {:?}", x_vault_token, body, context.get().0.clone());
        Err("Generic failuare".into())
    }

}
