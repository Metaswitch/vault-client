//! Server implementation of vault_api.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;

use std::collections::HashMap;

use swagger;

use vault_api::{Api, ApiError, Context,
                      SysLeasesRevokePutResponse,
                      GenerateCertResponse,
                      ReadCertResponse,
                      CreateOrphanTokenResponse,
                      CreateTokenResponse,
                      LogInWithTLSCertificateResponse,
                      RenewOwnTokenResponse
};
use vault_api::models;

#[derive(Copy, Clone)]
pub struct Server;

impl Api for Server {

    /// Revoke lease
    fn sys_leases_revoke_put(&self, x_vault_token: String, body: models::RevokeLeaseParameters, context: &Context) -> Box<Future<Item=SysLeasesRevokePutResponse, Error=ApiError> + Send> {
        let context = context.clone();
        println!("sys_leases_revoke_put(\"{}\", {:?}) - X-Span-ID: {:?}", x_vault_token, body, context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Generate certificate
    fn generate_cert(&self, x_vault_token: String, mount: String, name: String, body: models::GenerateCertificateParameters, context: &Context) -> Box<Future<Item=GenerateCertResponse, Error=ApiError> + Send> {
        let context = context.clone();
        println!("generate_cert(\"{}\", \"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", x_vault_token, mount, name, body, context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Read certificate
    fn read_cert(&self, mount: String, serial: String, context: &Context) -> Box<Future<Item=ReadCertResponse, Error=ApiError> + Send> {
        let context = context.clone();
        println!("read_cert(\"{}\", \"{}\") - X-Span-ID: {:?}", mount, serial, context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Create an orphan token
    fn create_orphan_token(&self, x_vault_token: String, body: models::CreateTokenParameters, context: &Context) -> Box<Future<Item=CreateOrphanTokenResponse, Error=ApiError> + Send> {
        let context = context.clone();
        println!("create_orphan_token(\"{}\", {:?}) - X-Span-ID: {:?}", x_vault_token, body, context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Create token
    fn create_token(&self, x_vault_token: String, body: models::CreateTokenParameters, context: &Context) -> Box<Future<Item=CreateTokenResponse, Error=ApiError> + Send> {
        let context = context.clone();
        println!("create_token(\"{}\", {:?}) - X-Span-ID: {:?}", x_vault_token, body, context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Log in
    fn log_in_with_tls_certificate(&self, body: Option<models::AuthCertLoginParameters>, context: &Context) -> Box<Future<Item=LogInWithTLSCertificateResponse, Error=ApiError> + Send> {
        let context = context.clone();
        println!("log_in_with_tls_certificate({:?}) - X-Span-ID: {:?}", body, context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Renew own token
    fn renew_own_token(&self, x_vault_token: String, body: models::RenewSelfParameters, context: &Context) -> Box<Future<Item=RenewOwnTokenResponse, Error=ApiError> + Send> {
        let context = context.clone();
        println!("renew_own_token(\"{}\", {:?}) - X-Span-ID: {:?}", x_vault_token, body, context.x_span_id.unwrap_or(String::from("<none>")).clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
