#![allow(unused_extern_crates)]
extern crate chrono;
extern crate hyper_openssl;
extern crate url;

use self::hyper_openssl::openssl;
use self::hyper_openssl::HttpsConnector;
use self::url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET, QUERY_ENCODE_SET};
use futures;
use futures::{future, stream};
use futures::{Future, Stream};
use http::header;
use http::header::{HeaderName, HeaderValue};
use hyper;
use hyper::client::connect::Connect;
use hyper::client::HttpConnector;
use hyper::Request;
use mime::Mime;
use serde::de::Deserialize;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io::{Error, Read};
use std::path::Path;
use std::str;
use std::sync::Arc;

use mimetypes;

use serde_json;

#[allow(unused_imports)]
use std::collections::{BTreeMap, HashMap};
#[allow(unused_imports)]
use swagger;

use swagger::{ApiError, Context, XSpanId};

use models;
use {
    Api, CreateOrphanTokenResponse, CreateTokenResponse, GenerateCertResponse,
    LogInWithTLSCertificateResponse, ReadCertResponse, RenewOwnTokenResponse,
    SysLeasesRevokePutResponse,
};

const REQUEST_X_VAULT_TOKEN: &'static str = "X-Vault-Token";

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path(
    url: &url::Url,
    correct_scheme: Option<&'static str>,
) -> Result<String, ClientInitError> {
    // First convert to Url, since a base path is a subset of Url.
    let scheme = url.scheme();

    // Check the scheme if necessary
    if let Some(correct_scheme) = correct_scheme {
        if scheme != correct_scheme {
            return Err(ClientInitError::InvalidScheme);
        }
    }

    let host = url.host().ok_or_else(|| ClientInitError::MissingHost)?;
    let port = url.port().map(|x| format!(":{}", x)).unwrap_or_default();
    Ok(format!("{}://{}{}", scheme, host, port))
}

/// Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
fn parse_response<'a, T>(response: &'a (http::response::Parts, hyper::Chunk)) -> Result<T, ApiError>
where
    T: Deserialize<'a>,
{
    match response.0.status.as_u16() {
        200 => serde_json::from_slice::<T>(&response.1)
            .map_err(|e| ApiError(format!("Response was not valid AuthResponse: {}", e))),
        code => {
            let debug_body = match str::from_utf8(&response.1) {
                Ok(body) => Cow::from(body),
                Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
            };
            Err(ApiError(format!(
                "Unexpected response code {}:\n{:?}\n\n{}",
                code, response.0.headers, debug_body
            )))
        }
    }
}

/// A client that implements the API by making HTTP calls out to a server.
#[derive(Clone)]
pub struct Client<C: Connect + Sync + 'static> {
    base_path: String,
    hyper_client: Arc<Fn() -> hyper::client::Client<C, hyper::Body> + Sync + Send>,
}

impl<C> fmt::Debug for Client<C>
where
    C: Connect,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl<C> Client<C>
where
    C: Connect,
{
    pub fn try_new_http(base_path: &url::Url) -> Result<Client<HttpConnector>, ClientInitError> {
        Ok(Client {
            base_path: into_base_path(base_path, Some("http"))?,
            hyper_client: Arc::new(hyper::client::Client::new),
        })
    }

    pub fn try_new_https<CA>(
        base_path: &url::Url,
        ca_certificate: CA,
    ) -> Result<Client<HttpsConnector<HttpConnector>>, ClientInitError>
    where
        CA: AsRef<Path>,
    {
        let ca_certificate = ca_certificate.as_ref().to_owned();

        let https_hyper_client = move || {
            // SSL implementation
            let mut ssl =
                openssl::ssl::SslConnector::builder(openssl::ssl::SslMethod::tls()).unwrap();

            // Server authentication
            ssl.set_ca_file(ca_certificate.clone()).unwrap();
            let http_connector = HttpConnector::new(1);
            let connector = HttpsConnector::with_connector(http_connector, ssl).unwrap();
            hyper::client::Client::builder().build(connector)
        };

        Ok(Client {
            base_path: into_base_path(base_path, Some("https"))?,
            hyper_client: Arc::new(https_hyper_client),
        })
    }

    pub fn try_new_https_mutual<CA, K, CC>(
        base_path: &url::Url,
        ca_certificate: CA,
        client_key: K,
        client_certificate: CC,
    ) -> Result<Client<HttpsConnector<HttpConnector>>, ClientInitError>
    where
        CA: AsRef<Path>,
        K: AsRef<Path>,
        CC: AsRef<Path>,
    {
        let ca_certificate = ca_certificate.as_ref().to_owned();
        let client_key = client_key.as_ref().to_owned();
        let client_certificate = client_certificate.as_ref().to_owned();

        let https_mutual_hyper_client = move || {
            // SSL implementation
            let mut ssl =
                openssl::ssl::SslConnector::builder(openssl::ssl::SslMethod::tls()).unwrap();

            // Server authentication
            ssl.set_ca_file(ca_certificate.clone()).unwrap();

            // Client authentication
            ssl.set_private_key_file(client_key.clone(), openssl::ssl::SslFiletype::PEM)
                .unwrap();
            ssl.set_certificate_chain_file(client_certificate.clone())
                .unwrap();
            ssl.check_private_key().unwrap();

            let http_connector = HttpConnector::new(1);
            let connector = HttpsConnector::with_connector(http_connector, ssl).unwrap();
            hyper::client::Client::builder().build(connector)
        };

        Ok(Client {
            base_path: into_base_path(base_path, Some("https"))?,
            hyper_client: Arc::new(https_mutual_hyper_client),
        })
    }

    /// Constructor for creating a `Client` by passing in a pre-made `hyper` client.
    ///
    /// One should avoid relying on this function if possible, since it adds a dependency on the underlying transport
    /// implementation, which it would be better to abstract away. Therefore, using this function may lead to a loss of
    /// code generality, which may make it harder to move the application to a serverless environment, for example.
    ///
    /// The reason for this function's existence is to support legacy test code, which did mocking at the hyper layer.
    /// This is not a recommended way to write new tests. If other reasons are found for using this function, they
    /// should be mentioned here.
    pub fn try_new_with_hyper_client(
        base_path: &url::Url,
        hyper_client: Arc<Fn() -> hyper::client::Client<C, hyper::Body> + Sync + Send>,
    ) -> Result<Client<C>, ClientInitError> {
        Ok(Client {
            base_path: into_base_path(base_path, None)?,
            hyper_client: hyper_client,
        })
    }
}

impl<C> Api for Client<C>
where
    C: Connect,
{
    fn sys_leases_revoke_put(
        &self,
        param_x_vault_token: String,
        param_body: models::RevokeLeaseParameters,
        context: &Context,
    ) -> Box<Future<Item = SysLeasesRevokePutResponse, Error = ApiError> + Send> {
        let url = format!("{}/v1/sys/leases/revoke", self.base_path);

        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");

        let hyper_client = (self.hyper_client)();

        let mut request = Request::put(&url);

        request.header(
            header::CONTENT_TYPE,
            mimetypes::requests::SYS_LEASES_REVOKE_PUT.clone(),
        );

        if let Some(ref xspan) = context.x_span_id {
            let x_span_value = HeaderValue::from_str(xspan).unwrap();
            request.header("x-span-id", x_span_value);
        }

        // Header parameters
        request.header(REQUEST_X_VAULT_TOKEN, param_x_vault_token);

        let request = request.body(body.into()).unwrap();

        // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
        fn parse_response<'a>(
            response: (http::response::Parts, hyper::Chunk),
        ) -> Result<SysLeasesRevokePutResponse, ApiError> {
            match response.0.status.as_u16() {
                204 => Ok(SysLeasesRevokePutResponse::Success),
                code => {
                    let debug_body = match str::from_utf8(&response.1.as_ref()) {
                        Ok(body) => Cow::from(body),
                        Err(e) => Cow::from(format!("<Body was not UTF8: {:?}>", e)),
                    };
                    Err(ApiError(format!(
                        "Unexpected response code {}:\n{:?}\n\n{}",
                        code, response.0.headers, debug_body
                    )))
                }
            }
        }

        let f = hyper_client
            .request(request)
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|response| {
                let (parts, body) = response.into_parts();
                body.concat2()
                    .map(|body| (parts, body))
                    .map_err(|e| ApiError(format!("Failed to read response body: {}", e)))
            })
            .and_then(parse_response);
        Box::new(f)
    }

    fn generate_cert(
        &self,
        param_x_vault_token: String,
        param_mount: String,
        param_name: String,
        param_body: models::GenerateCertificateParameters,
        context: &Context,
    ) -> Box<Future<Item = GenerateCertResponse, Error = ApiError> + Send> {
        let url = format!(
            "{}/v1/{mount}/issue/{name}",
            self.base_path,
            mount = utf8_percent_encode(&param_mount.to_string(), PATH_SEGMENT_ENCODE_SET),
            name = utf8_percent_encode(&param_name.to_string(), PATH_SEGMENT_ENCODE_SET)
        );

        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");

        let hyper_client = (self.hyper_client)();
        let mut request = Request::post(&url);

        request.header(
            header::CONTENT_TYPE,
            mimetypes::requests::GENERATE_CERT.clone(),
        );
        if let Some(ref xspan) = context.x_span_id {
            let x_span_value = HeaderValue::from_str(xspan).unwrap();
            request.header("x-span-id", x_span_value);
        }

        // Header parameters
        let x_vault_token = HeaderValue::from_str(param_x_vault_token.as_str()).unwrap();
        request.header(REQUEST_X_VAULT_TOKEN, x_vault_token);

        let request = request.body(body.into()).unwrap();

        let f = hyper_client
            .request(request)
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|response| {
                let (parts, body) = response.into_parts();
                body.concat2()
                    .map(|body| (parts, body))
                    .map_err(|e| ApiError(format!("Failed to read response body: {}", e)))
            })
            .and_then(|ref x| parse_response(x))
            .map(|x| GenerateCertResponse::Success(x));
        Box::new(f)
    }

    fn read_cert(
        &self,
        param_mount: String,
        param_serial: String,
        context: &Context,
    ) -> Box<Future<Item = ReadCertResponse, Error = ApiError> + Send> {
        let url = format!(
            "{}/v1/{mount}/cert/{serial}",
            self.base_path,
            mount = utf8_percent_encode(&param_mount.to_string(), PATH_SEGMENT_ENCODE_SET),
            serial = utf8_percent_encode(&param_serial.to_string(), PATH_SEGMENT_ENCODE_SET)
        );

        let hyper_client = (self.hyper_client)();
        let mut request = Request::get(&url);

        if let Some(ref xspan) = context.x_span_id {
            let x_span_value = HeaderValue::from_str(xspan).unwrap();
            request.header("x-span-id", x_span_value);
        }

        let request = request.body(hyper::Body::empty()).unwrap();

        let f = hyper_client
            .request(request)
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|response| {
                let (parts, body) = response.into_parts();
                body.concat2()
                    .map(|body| (parts, body))
                    .map_err(|e| ApiError(format!("Failed to read response body: {}", e)))
            })
            .and_then(|ref x| parse_response(x))
            .map(|x| ReadCertResponse::Success(x));
        Box::new(f)
    }

    fn create_orphan_token(
        &self,
        param_x_vault_token: String,
        param_body: models::CreateTokenParameters,
        context: &Context,
    ) -> Box<Future<Item = CreateOrphanTokenResponse, Error = ApiError> + Send> {
        let url = format!("{}/v1/auth/token/create-orphan", self.base_path);

        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");

        let hyper_client = (self.hyper_client)();
        let mut request = Request::post(&url);

        request.header(
            header::CONTENT_TYPE,
            mimetypes::requests::CREATE_ORPHAN_TOKEN.clone(),
        );

        if let Some(ref xspan) = context.x_span_id {
            let x_span_value = HeaderValue::from_str(xspan).unwrap();
            request.header("x-span-id", x_span_value);
        }

        // Header parameters
        request.header(REQUEST_X_VAULT_TOKEN, param_x_vault_token);

        let request = request.body(body.into()).unwrap();

        let f = hyper_client
            .request(request)
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|response| {
                let (parts, body) = response.into_parts();
                body.concat2()
                    .map(|body| (parts, body))
                    .map_err(|e| ApiError(format!("Failed to read response body: {}", e)))
            })
            .and_then(|ref x| parse_response(x))
            .map(|x| CreateOrphanTokenResponse::Success(x));
        Box::new(f)
    }

    fn create_token(
        &self,
        param_x_vault_token: String,
        param_body: models::CreateTokenParameters,
        context: &Context,
    ) -> Box<Future<Item = CreateTokenResponse, Error = ApiError> + Send> {
        let url = format!("{}/v1/auth/token/create", self.base_path);

        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");

        let hyper_client = (self.hyper_client)();
        let mut request = Request::post(&url);

        request.header(
            header::CONTENT_TYPE,
            mimetypes::requests::CREATE_TOKEN.clone(),
        );

        if let Some(ref xspan) = context.x_span_id {
            let x_span_value = HeaderValue::from_str(xspan).unwrap();
            request.header("x-span-id", x_span_value);
        }

        // Header parameters
        request.header(REQUEST_X_VAULT_TOKEN, param_x_vault_token);

        let request = request.body(body.into()).unwrap();

        let f = hyper_client
            .request(request)
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|response| {
                let (parts, body) = response.into_parts();
                body.concat2()
                    .map(|body| (parts, body))
                    .map_err(|e| ApiError(format!("Failed to read response body: {}", e)))
            })
            .and_then(|ref x| parse_response(x))
            .map(|x| CreateTokenResponse::Success(x));
        Box::new(f)
    }

    fn log_in_with_tls_certificate(
        &self,
        param_body: Option<models::AuthCertLoginParameters>,
        context: &Context,
    ) -> Box<Future<Item = LogInWithTLSCertificateResponse, Error = ApiError> + Send> {
        let url = format!("{}/v1/auth/cert/login", self.base_path);

        let body = param_body
            .map(|ref body| serde_json::to_string(body).expect("impossible to fail to serialize"));
        let hyper_client = (self.hyper_client)();
        let mut request = Request::post(&url);

        request.header(
            header::CONTENT_TYPE,
            mimetypes::requests::LOG_IN_WITH_TLS_CERTIFICATE.clone(),
        );

        if let Some(ref xspan) = context.x_span_id {
            let x_span_value = HeaderValue::from_str(xspan).unwrap();
            request.header("x-span-id", x_span_value);
        }

        let request = match body {
            Some(body) => request.body(body.into()).unwrap(),
            None => request.body(hyper::Body::empty()).unwrap(),
        };

        let f = hyper_client
            .request(request)
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|response| {
                let (parts, body) = response.into_parts();
                body.concat2()
                    .map(|body| (parts, body))
                    .map_err(|e| ApiError(format!("Failed to read response body: {}", e)))
            })
            .and_then(|ref x| parse_response(x))
            .map(|x| LogInWithTLSCertificateResponse::Success(x));
        Box::new(f)
    }

    fn renew_own_token(
        &self,
        param_x_vault_token: String,
        param_body: models::RenewSelfParameters,
        context: &Context,
    ) -> Box<Future<Item = RenewOwnTokenResponse, Error = ApiError> + Send> {
        let url = format!("{}/v1/auth/token/renew-self", self.base_path);

        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");

        let hyper_client = (self.hyper_client)();
        let mut request = Request::post(&url);

        request.header(
            header::CONTENT_TYPE,
            mimetypes::requests::RENEW_OWN_TOKEN.clone(),
        );

        if let Some(ref xspan) = context.x_span_id {
            let x_span_value = HeaderValue::from_str(xspan).unwrap();
            request.header("x-span-id", x_span_value);
        }

        // Header parameters
        request.header(REQUEST_X_VAULT_TOKEN, param_x_vault_token);

        let request = request.body(body.into()).unwrap();

        let f = hyper_client
            .request(request)
            .map_err(|e| ApiError(format!("No response received: {}", e)))
            .and_then(|response| {
                let (parts, body) = response.into_parts();
                body.concat2()
                    .map(|body| (parts, body))
                    .map_err(|e| ApiError(format!("Failed to read response body: {}", e)))
            })
            .and_then(|ref x| parse_response(x))
            .map(|x| RenewOwnTokenResponse::Success(x));
        Box::new(f)
    }
}

#[derive(Debug)]
pub enum ClientInitError {
    InvalidScheme,
    InvalidUrl(url::ParseError),
    MissingHost,
    SslError(openssl::error::ErrorStack),
}

impl From<url::ParseError> for ClientInitError {
    fn from(err: url::ParseError) -> ClientInitError {
        ClientInitError::InvalidUrl(err)
    }
}

impl From<openssl::error::ErrorStack> for ClientInitError {
    fn from(err: openssl::error::ErrorStack) -> ClientInitError {
        ClientInitError::SslError(err)
    }
}

impl fmt::Display for ClientInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &fmt::Debug).fmt(f)
    }
}

impl error::Error for ClientInitError {
    fn description(&self) -> &str {
        "Failed to produce a hyper client."
    }
}
