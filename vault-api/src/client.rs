#![allow(unused_extern_crates)]
extern crate hyper_openssl;
extern crate chrono;
extern crate url;



use hyper;
use hyper::client::IntoUrl;
use hyper::mime;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::Url;
use self::hyper_openssl::openssl;
use self::url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET, QUERY_ENCODE_SET};
use futures;
use futures::{Future, Stream};
use futures::{future, stream};
use std::borrow::Cow;
use std::io::{Read, Error};
use std::error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use std::str;

use mimetypes;

use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;

use swagger::{Context, ApiError, XSpanId};

use {Api,
     SysLeasesRevokePutResponse,
     GenerateCertResponse,
     ReadCertResponse,
     CreateOrphanTokenResponse,
     CreateTokenResponse,
     LogInWithTLSCertificateResponse,
     RenewOwnTokenResponse
     };
use models;

/// Convert input into a base path, e.g. "http://example:123". Also checks the scheme as it goes.
fn into_base_path<T: IntoUrl>(input: T, correct_scheme: Option<&'static str>) -> Result<String, ClientInitError> {
    // First convert to Url, since a base path is a subset of Url.
    let url = input.into_url()?;

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

/// A client that implements the API by making HTTP calls out to a server.
#[derive(Clone)]
pub struct Client {
    base_path: String,
    hyper_client: Arc<Fn() -> hyper::client::Client + Sync + Send>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client {{ base_path: {} }}", self.base_path)
    }
}

impl Client {
    pub fn try_new_http<T>(base_path: T) -> Result<Client, ClientInitError>
        where T: IntoUrl
    {
        Ok(Client {
            base_path: into_base_path(base_path, Some("http"))?,
            hyper_client: Arc::new(hyper::client::Client::new),
        })
    }

    pub fn try_new_https<T, CA>(base_path: T,
                                ca_certificate: CA)
                            -> Result<Client, ClientInitError>
        where T: IntoUrl,
              CA: AsRef<Path>
    {
        let ca_certificate = ca_certificate.as_ref().to_owned();

        let https_hyper_client = move || {
            // SSL implementation
            let mut ssl = openssl::ssl::SslConnectorBuilder::new(openssl::ssl::SslMethod::tls()).unwrap();

            // Server authentication
            ssl.builder_mut().set_ca_file(ca_certificate.clone()).unwrap();

            let ssl = hyper_openssl::OpensslClient::from(ssl.build());
            let connector = hyper::net::HttpsConnector::new(ssl);
            hyper::client::Client::with_connector(connector)
        };

        Ok(Client {
                base_path: into_base_path(base_path, Some("https"))?,
                hyper_client: Arc::new(https_hyper_client),
            })
    }

    pub fn try_new_https_mutual<T, CA, K, C>(base_path: T,
                                             ca_certificate: CA,
                                             client_key: K,
                                             client_certificate: C)
                                             -> Result<Client, ClientInitError>
        where T: IntoUrl,
              CA: AsRef<Path>,
              K: AsRef<Path>,
              C: AsRef<Path>
    {
        let ca_certificate = ca_certificate.as_ref().to_owned();
        let client_key = client_key.as_ref().to_owned();
        let client_certificate = client_certificate.as_ref().to_owned();

        let https_mutual_hyper_client = move || {
            // SSL implementation
            let mut ssl = openssl::ssl::SslConnectorBuilder::new(openssl::ssl::SslMethod::tls()).unwrap();

            // Server authentication
            ssl.builder_mut().set_ca_file(ca_certificate.clone()).unwrap();

            // Client authentication
            ssl.builder_mut().set_private_key_file(client_key.clone(), openssl::x509::X509_FILETYPE_PEM).unwrap();
            ssl.builder_mut().set_certificate_chain_file(client_certificate.clone()).unwrap();
            ssl.builder_mut().check_private_key().unwrap();

            let ssl = hyper_openssl::OpensslClient::from(ssl.build());
            let connector = hyper::net::HttpsConnector::new(ssl);
            hyper::client::Client::with_connector(connector)
        };

        Ok(Client {
                base_path: into_base_path(base_path, Some("https"))?,
                hyper_client: Arc::new(https_mutual_hyper_client)
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
    pub fn try_new_with_hyper_client<T>(base_path: T,
                                    hyper_client: Arc<Fn() -> hyper::client::Client + Sync + Send>)
                                    -> Result<Client, ClientInitError>
        where T: IntoUrl
    {
        Ok(Client {
            base_path: into_base_path(base_path, None)?,
            hyper_client: hyper_client
        })
    }
}

impl Api for Client {

    fn sys_leases_revoke_put(&self, param_x_vault_token: String, param_body: models::RevokeLeaseParameters, context: &Context) -> Box<Future<Item=SysLeasesRevokePutResponse, Error=ApiError> + Send> {


        let url = format!(
            "{}/v1/sys/leases/revoke",
            self.base_path
        );


        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");

        let hyper_client = (self.hyper_client)();
        let request = hyper_client.request(hyper::method::Method::Put, &url);
        let mut custom_headers = hyper::header::Headers::new();

        let request = request.body(&body);

        custom_headers.set(ContentType(mimetypes::requests::SYS_LEASES_REVOKE_PUT.clone()));
        context.x_span_id.as_ref().map(|header| custom_headers.set(XSpanId(header.clone())));

        // Header parameters
        header! { (RequestXVaultToken, "X-Vault-Token") => [String] }
        custom_headers.set(RequestXVaultToken(param_x_vault_token));


        let request = request.headers(custom_headers);

        // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
        fn parse_response(mut response: hyper::client::response::Response) -> Result<SysLeasesRevokePutResponse, ApiError> {
            match response.status.to_u16() {
                204 => {


                    Ok(SysLeasesRevokePutResponse::Success)
                },
                code => {
                    let mut buf = [0; 100];
                    let debug_body = match response.read(&mut buf) {
                        Ok(len) => match str::from_utf8(&buf[..len]) {
                            Ok(body) => Cow::from(body),
                            Err(_) => Cow::from(format!("<Body was not UTF8: {:?}>", &buf[..len].to_vec())),
                        },
                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                    };
                    Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                         code,
                                         response.headers,
                                         debug_body)))
                }
            }
        }

        let result = request.send().map_err(|e| ApiError(format!("No response received: {}", e))).and_then(parse_response);
        Box::new(futures::done(result))
    }

    fn generate_cert(&self, param_x_vault_token: String, param_mount: String, param_name: String, param_body: models::GenerateCertificateParameters, context: &Context) -> Box<Future<Item=GenerateCertResponse, Error=ApiError> + Send> {


        let url = format!(
            "{}/v1/{mount}/issue/{name}",
            self.base_path, mount=utf8_percent_encode(&param_mount.to_string(), PATH_SEGMENT_ENCODE_SET), name=utf8_percent_encode(&param_name.to_string(), PATH_SEGMENT_ENCODE_SET)
        );


        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");

        let hyper_client = (self.hyper_client)();
        let request = hyper_client.request(hyper::method::Method::Post, &url);
        let mut custom_headers = hyper::header::Headers::new();

        let request = request.body(&body);

        custom_headers.set(ContentType(mimetypes::requests::GENERATE_CERT.clone()));
        context.x_span_id.as_ref().map(|header| custom_headers.set(XSpanId(header.clone())));

        // Header parameters
        header! { (RequestXVaultToken, "X-Vault-Token") => [String] }
        custom_headers.set(RequestXVaultToken(param_x_vault_token));


        let request = request.headers(custom_headers);

        // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
        fn parse_response(mut response: hyper::client::response::Response) -> Result<GenerateCertResponse, ApiError> {
            match response.status.to_u16() {
                200 => {
                    let mut buf = String::new();
                    response.read_to_string(&mut buf).map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                    let body = serde_json::from_str::<models::GenerateCertificateResponse>(&buf)?;



                    Ok(GenerateCertResponse::Success(body))
                },
                code => {
                    let mut buf = [0; 100];
                    let debug_body = match response.read(&mut buf) {
                        Ok(len) => match str::from_utf8(&buf[..len]) {
                            Ok(body) => Cow::from(body),
                            Err(_) => Cow::from(format!("<Body was not UTF8: {:?}>", &buf[..len].to_vec())),
                        },
                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                    };
                    Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                         code,
                                         response.headers,
                                         debug_body)))
                }
            }
        }

        let result = request.send().map_err(|e| ApiError(format!("No response received: {}", e))).and_then(parse_response);
        Box::new(futures::done(result))
    }

    fn read_cert(&self, param_mount: String, param_serial: String, context: &Context) -> Box<Future<Item=ReadCertResponse, Error=ApiError> + Send> {


        let url = format!(
            "{}/v1/{mount}/cert/{serial}",
            self.base_path, mount=utf8_percent_encode(&param_mount.to_string(), PATH_SEGMENT_ENCODE_SET), serial=utf8_percent_encode(&param_serial.to_string(), PATH_SEGMENT_ENCODE_SET)
        );


        let hyper_client = (self.hyper_client)();
        let request = hyper_client.request(hyper::method::Method::Get, &url);
        let mut custom_headers = hyper::header::Headers::new();

        context.x_span_id.as_ref().map(|header| custom_headers.set(XSpanId(header.clone())));


        let request = request.headers(custom_headers);

        // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
        fn parse_response(mut response: hyper::client::response::Response) -> Result<ReadCertResponse, ApiError> {
            match response.status.to_u16() {
                200 => {
                    let mut buf = String::new();
                    response.read_to_string(&mut buf).map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                    let body = serde_json::from_str::<models::CertificateResponse>(&buf)?;



                    Ok(ReadCertResponse::Success(body))
                },
                code => {
                    let mut buf = [0; 100];
                    let debug_body = match response.read(&mut buf) {
                        Ok(len) => match str::from_utf8(&buf[..len]) {
                            Ok(body) => Cow::from(body),
                            Err(_) => Cow::from(format!("<Body was not UTF8: {:?}>", &buf[..len].to_vec())),
                        },
                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                    };
                    Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                         code,
                                         response.headers,
                                         debug_body)))
                }
            }
        }

        let result = request.send().map_err(|e| ApiError(format!("No response received: {}", e))).and_then(parse_response);
        Box::new(futures::done(result))
    }

    fn create_orphan_token(&self, param_x_vault_token: String, param_body: models::CreateTokenParameters, context: &Context) -> Box<Future<Item=CreateOrphanTokenResponse, Error=ApiError> + Send> {


        let url = format!(
            "{}/v1/auth/token/create-orphan",
            self.base_path
        );


        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");

        let hyper_client = (self.hyper_client)();
        let request = hyper_client.request(hyper::method::Method::Post, &url);
        let mut custom_headers = hyper::header::Headers::new();

        let request = request.body(&body);

        custom_headers.set(ContentType(mimetypes::requests::CREATE_ORPHAN_TOKEN.clone()));
        context.x_span_id.as_ref().map(|header| custom_headers.set(XSpanId(header.clone())));

        // Header parameters
        header! { (RequestXVaultToken, "X-Vault-Token") => [String] }
        custom_headers.set(RequestXVaultToken(param_x_vault_token));


        let request = request.headers(custom_headers);

        // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
        fn parse_response(mut response: hyper::client::response::Response) -> Result<CreateOrphanTokenResponse, ApiError> {
            match response.status.to_u16() {
                200 => {
                    let mut buf = String::new();
                    response.read_to_string(&mut buf).map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                    let body = serde_json::from_str::<models::AuthResponse>(&buf)?;



                    Ok(CreateOrphanTokenResponse::Success(body))
                },
                code => {
                    let mut buf = [0; 100];
                    let debug_body = match response.read(&mut buf) {
                        Ok(len) => match str::from_utf8(&buf[..len]) {
                            Ok(body) => Cow::from(body),
                            Err(_) => Cow::from(format!("<Body was not UTF8: {:?}>", &buf[..len].to_vec())),
                        },
                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                    };
                    Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                         code,
                                         response.headers,
                                         debug_body)))
                }
            }
        }

        let result = request.send().map_err(|e| ApiError(format!("No response received: {}", e))).and_then(parse_response);
        Box::new(futures::done(result))
    }

    fn create_token(&self, param_x_vault_token: String, param_body: models::CreateTokenParameters, context: &Context) -> Box<Future<Item=CreateTokenResponse, Error=ApiError> + Send> {


        let url = format!(
            "{}/v1/auth/token/create",
            self.base_path
        );


        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");

        let hyper_client = (self.hyper_client)();
        let request = hyper_client.request(hyper::method::Method::Post, &url);
        let mut custom_headers = hyper::header::Headers::new();

        let request = request.body(&body);

        custom_headers.set(ContentType(mimetypes::requests::CREATE_TOKEN.clone()));
        context.x_span_id.as_ref().map(|header| custom_headers.set(XSpanId(header.clone())));

        // Header parameters
        header! { (RequestXVaultToken, "X-Vault-Token") => [String] }
        custom_headers.set(RequestXVaultToken(param_x_vault_token));


        let request = request.headers(custom_headers);

        // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
        fn parse_response(mut response: hyper::client::response::Response) -> Result<CreateTokenResponse, ApiError> {
            match response.status.to_u16() {
                200 => {
                    let mut buf = String::new();
                    response.read_to_string(&mut buf).map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                    let body = serde_json::from_str::<models::AuthResponse>(&buf)?;



                    Ok(CreateTokenResponse::Success(body))
                },
                code => {
                    let mut buf = [0; 100];
                    let debug_body = match response.read(&mut buf) {
                        Ok(len) => match str::from_utf8(&buf[..len]) {
                            Ok(body) => Cow::from(body),
                            Err(_) => Cow::from(format!("<Body was not UTF8: {:?}>", &buf[..len].to_vec())),
                        },
                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                    };
                    Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                         code,
                                         response.headers,
                                         debug_body)))
                }
            }
        }

        let result = request.send().map_err(|e| ApiError(format!("No response received: {}", e))).and_then(parse_response);
        Box::new(futures::done(result))
    }

    fn log_in_with_tls_certificate(&self, param_body: Option<models::AuthCertLoginParameters>, context: &Context) -> Box<Future<Item=LogInWithTLSCertificateResponse, Error=ApiError> + Send> {


        let url = format!(
            "{}/v1/auth/cert/login",
            self.base_path
        );

        let body = param_body.map(|ref body| {

            serde_json::to_string(body).expect("impossible to fail to serialize")
        });
        let hyper_client = (self.hyper_client)();
        let request = hyper_client.request(hyper::method::Method::Post, &url);
        let mut custom_headers = hyper::header::Headers::new();

        let request = match body {
            Some(ref body) => request.body(body),
            None => request,
        };

        custom_headers.set(ContentType(mimetypes::requests::LOG_IN_WITH_TLS_CERTIFICATE.clone()));
        context.x_span_id.as_ref().map(|header| custom_headers.set(XSpanId(header.clone())));


        let request = request.headers(custom_headers);

        // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
        fn parse_response(mut response: hyper::client::response::Response) -> Result<LogInWithTLSCertificateResponse, ApiError> {
            match response.status.to_u16() {
                200 => {
                    let mut buf = String::new();
                    response.read_to_string(&mut buf).map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                    let body = serde_json::from_str::<models::AuthResponse>(&buf)?;



                    Ok(LogInWithTLSCertificateResponse::Success(body))
                },
                code => {
                    let mut buf = [0; 100];
                    let debug_body = match response.read(&mut buf) {
                        Ok(len) => match str::from_utf8(&buf[..len]) {
                            Ok(body) => Cow::from(body),
                            Err(_) => Cow::from(format!("<Body was not UTF8: {:?}>", &buf[..len].to_vec())),
                        },
                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                    };
                    Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                         code,
                                         response.headers,
                                         debug_body)))
                }
            }
        }

        let result = request.send().map_err(|e| ApiError(format!("No response received: {}", e))).and_then(parse_response);
        Box::new(futures::done(result))
    }

    fn renew_own_token(&self, param_x_vault_token: String, param_body: models::RenewSelfParameters, context: &Context) -> Box<Future<Item=RenewOwnTokenResponse, Error=ApiError> + Send> {


        let url = format!(
            "{}/v1/auth/token/renew-self",
            self.base_path
        );


        let body = serde_json::to_string(&param_body).expect("impossible to fail to serialize");

        let hyper_client = (self.hyper_client)();
        let request = hyper_client.request(hyper::method::Method::Post, &url);
        let mut custom_headers = hyper::header::Headers::new();

        let request = request.body(&body);

        custom_headers.set(ContentType(mimetypes::requests::RENEW_OWN_TOKEN.clone()));
        context.x_span_id.as_ref().map(|header| custom_headers.set(XSpanId(header.clone())));

        // Header parameters
        header! { (RequestXVaultToken, "X-Vault-Token") => [String] }
        custom_headers.set(RequestXVaultToken(param_x_vault_token));


        let request = request.headers(custom_headers);

        // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
        fn parse_response(mut response: hyper::client::response::Response) -> Result<RenewOwnTokenResponse, ApiError> {
            match response.status.to_u16() {
                200 => {
                    let mut buf = String::new();
                    response.read_to_string(&mut buf).map_err(|e| ApiError(format!("Response was not valid UTF8: {}", e)))?;
                    let body = serde_json::from_str::<models::AuthResponse>(&buf)?;



                    Ok(RenewOwnTokenResponse::Success(body))
                },
                code => {
                    let mut buf = [0; 100];
                    let debug_body = match response.read(&mut buf) {
                        Ok(len) => match str::from_utf8(&buf[..len]) {
                            Ok(body) => Cow::from(body),
                            Err(_) => Cow::from(format!("<Body was not UTF8: {:?}>", &buf[..len].to_vec())),
                        },
                        Err(e) => Cow::from(format!("<Failed to read body: {}>", e)),
                    };
                    Err(ApiError(format!("Unexpected response code {}:\n{:?}\n\n{}",
                                         code,
                                         response.headers,
                                         debug_body)))
                }
            }
        }

        let result = request.send().map_err(|e| ApiError(format!("No response received: {}", e))).and_then(parse_response);
        Box::new(futures::done(result))
    }

}

#[derive(Debug)]
pub enum ClientInitError {
    InvalidScheme,
    InvalidUrl(hyper::error::ParseError),
    MissingHost,
    SslError(openssl::error::ErrorStack)
}

impl From<hyper::error::ParseError> for ClientInitError {
    fn from(err: hyper::error::ParseError) -> ClientInitError {
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
