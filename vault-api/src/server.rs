#![allow(unused_extern_crates)]
extern crate serde_ignored;
extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate urlencoded;
extern crate uuid;
extern crate chrono;


use futures::Future;
use futures::future;
use futures::{stream, Stream};
use self::iron::headers;
use self::iron::headers::{Headers, ContentType};
use self::iron::prelude::*;
use self::iron::{status, modifiers, BeforeMiddleware};
use self::iron::url::percent_encoding::percent_decode;
use self::router::Router;
use self::urlencoded::UrlEncodedQuery;
use mimetypes;


use serde_json;


#[allow(unused_imports)]
use std::collections::{HashMap, BTreeMap};
#[allow(unused_imports)]
use swagger;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::BTreeSet;

pub use swagger::auth::Authorization;
use swagger::auth::{AuthData, Scopes};
use swagger::{ApiError, Context, XSpanId};

use {Api,
     SysLeasesRevokePutResponse,
     GenerateCertResponse,
     ReadCertResponse,
     CreateOrphanTokenResponse,
     CreateTokenResponse,
     LogInWithTLSCertificateResponse,
     RenewOwnTokenResponse
     };
#[allow(unused_imports)]
use models;

/// Create a new router for `Api`
pub fn router<T>(api: T) -> Router where T: Api + Send + Sync + Clone + 'static {
    let mut router = Router::new();
    add_routes(&mut router, api);
    router
}

/// Add routes for `Api` to a provided router.
///
/// Note that these routes are added straight onto the router. This means that if the router
/// already has a route for an endpoint which clashes with those provided by this API, then the
/// old route will be lost.
///
/// It is generally a bad idea to add routes in this way to an existing router, which may have
/// routes on it for other APIs. Distinct APIs should be behind distinct paths to encourage
/// separation of interfaces, which this function does not enforce. APIs should not overlap.
///
/// Alternative approaches include:
///
/// - generate an `iron::middleware::Handler` (usually a `router::Router` or
///   `iron::middleware::chain`) for each interface, and add those handlers inside an existing
///   router, mounted at different paths - so the interfaces are separated by path
/// - use a different instance of `iron::Iron` for each interface - so the interfaces are
///   separated by the address/port they listen on
///
/// This function exists to allow legacy code, which doesn't separate its APIs properly, to make
/// use of this crate.
#[deprecated(note="APIs should not overlap - only for use in legacy code.")]
pub fn route<T>(router: &mut Router, api: T) where T: Api + Send + Sync + Clone + 'static {
    add_routes(router, api)
}

/// Add routes for `Api` to a provided router
fn add_routes<T>(router: &mut Router, api: T) where T: Api + Send + Sync + Clone + 'static {

    let api_clone = api.clone();
    router.put(
        "/v1/sys/leases/revoke",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Header parameters
                let param_x_vault_token = req.headers.get::<RequestXVaultToken>().ok_or_else(|| Response::with((status::BadRequest, "Missing or invalid required header X-Vault-Token".to_string())))?.0.clone();


                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.

                let param_body_raw = req.get::<bodyparser::Raw>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - not valid UTF-8: {}", e))))?;
                let mut unused_elements = Vec::new();

                let param_body = if let Some(param_body_raw) = param_body_raw { 
                    let deserializer = &mut serde_json::Deserializer::from_str(&param_body_raw);

                    let param_body: Option<models::RevokeLeaseParameters> = serde_ignored::deserialize(deserializer, |path| {
                            warn!("Ignoring unknown field in body: {}", path);
                            unused_elements.push(path.to_string());
                        }).map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;

                    param_body
                } else {
                    None
                };
                let param_body = param_body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.sys_leases_revoke_put(param_x_vault_token, param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        SysLeasesRevokePutResponse::Success => {


                            let mut response = Response::with(status::Status::from_u16(204));
                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            if !unused_elements.is_empty() {
                                response.headers.set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                            }
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "SysLeasesRevokePut");

    let api_clone = api.clone();
    router.post(
        "/v1/:mount/issue/:name",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Path parameters
                let param_mount = {
                    let param = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                        .find("mount").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter mount".to_string())))?;
                    percent_decode(param.as_bytes()).decode_utf8()
                        .map_err(|_| Response::with((status::BadRequest, format!("Couldn't percent-decode path parameter as UTF-8: {}", param))))?
                        .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter mount: {}", e))))?
                };
                let param_name = {
                    let param = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                        .find("name").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter name".to_string())))?;
                    percent_decode(param.as_bytes()).decode_utf8()
                        .map_err(|_| Response::with((status::BadRequest, format!("Couldn't percent-decode path parameter as UTF-8: {}", param))))?
                        .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter name: {}", e))))?
                };

                // Header parameters
                let param_x_vault_token = req.headers.get::<RequestXVaultToken>().ok_or_else(|| Response::with((status::BadRequest, "Missing or invalid required header X-Vault-Token".to_string())))?.0.clone();


                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.

                let param_body_raw = req.get::<bodyparser::Raw>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - not valid UTF-8: {}", e))))?;
                let mut unused_elements = Vec::new();

                let param_body = if let Some(param_body_raw) = param_body_raw { 
                    let deserializer = &mut serde_json::Deserializer::from_str(&param_body_raw);

                    let param_body: Option<models::GenerateCertificateParameters> = serde_ignored::deserialize(deserializer, |path| {
                            warn!("Ignoring unknown field in body: {}", path);
                            unused_elements.push(path.to_string());
                        }).map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;

                    param_body
                } else {
                    None
                };
                let param_body = param_body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.generate_cert(param_x_vault_token, param_mount, param_name, param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        GenerateCertResponse::Success(body) => {

                            let body_string = serde_json::to_string(&body).expect("impossible to fail to serialize");

                            let mut response = Response::with((status::Status::from_u16(200), body_string));
                            response.headers.set(ContentType(mimetypes::responses::GENERATE_CERT_SUCCESS.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            if !unused_elements.is_empty() {
                                response.headers.set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                            }
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "GenerateCert");

    let api_clone = api.clone();
    router.get(
        "/v1/:mount/cert/:serial",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Path parameters
                let param_mount = {
                    let param = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                        .find("mount").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter mount".to_string())))?;
                    percent_decode(param.as_bytes()).decode_utf8()
                        .map_err(|_| Response::with((status::BadRequest, format!("Couldn't percent-decode path parameter as UTF-8: {}", param))))?
                        .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter mount: {}", e))))?
                };
                let param_serial = {
                    let param = req.extensions.get::<Router>().ok_or_else(|| Response::with((status::InternalServerError, "An internal error occurred".to_string())))?
                        .find("serial").ok_or_else(|| Response::with((status::BadRequest, "Missing path parameter serial".to_string())))?;
                    percent_decode(param.as_bytes()).decode_utf8()
                        .map_err(|_| Response::with((status::BadRequest, format!("Couldn't percent-decode path parameter as UTF-8: {}", param))))?
                        .parse().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse path parameter serial: {}", e))))?
                };



                match api.read_cert(param_mount, param_serial, context).wait() {
                    Ok(rsp) => match rsp {
                        ReadCertResponse::Success(body) => {

                            let body_string = serde_json::to_string(&body).expect("impossible to fail to serialize");

                            let mut response = Response::with((status::Status::from_u16(200), body_string));
                            response.headers.set(ContentType(mimetypes::responses::READ_CERT_SUCCESS.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));

                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "ReadCert");

    let api_clone = api.clone();
    router.post(
        "/v1/auth/token/create-orphan",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Header parameters
                let param_x_vault_token = req.headers.get::<RequestXVaultToken>().ok_or_else(|| Response::with((status::BadRequest, "Missing or invalid required header X-Vault-Token".to_string())))?.0.clone();


                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.

                let param_body_raw = req.get::<bodyparser::Raw>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - not valid UTF-8: {}", e))))?;
                let mut unused_elements = Vec::new();

                let param_body = if let Some(param_body_raw) = param_body_raw {
                    let deserializer = &mut serde_json::Deserializer::from_str(&param_body_raw);

                    let param_body: Option<models::CreateTokenParameters> = serde_ignored::deserialize(deserializer, |path| {
                            warn!("Ignoring unknown field in body: {}", path);
                            unused_elements.push(path.to_string());
                        }).map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;

                    param_body
                } else {
                    None
                };
                let param_body = param_body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.create_orphan_token(param_x_vault_token, param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        CreateOrphanTokenResponse::Success(body) => {

                            let body_string = serde_json::to_string(&body).expect("impossible to fail to serialize");

                            let mut response = Response::with((status::Status::from_u16(200), body_string));
                            response.headers.set(ContentType(mimetypes::responses::CREATE_ORPHAN_TOKEN_SUCCESS.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            if !unused_elements.is_empty() {
                                let warn = format!("Ignoring unknown fields in body: {:?}", unused_elements);
                                response.headers.set(Warning(warn));
                            }
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "CreateOrphanToken");

    let api_clone = api.clone();
    router.post(
        "/v1/auth/token/create",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Header parameters
                let param_x_vault_token = req.headers.get::<RequestXVaultToken>().ok_or_else(|| Response::with((status::BadRequest, "Missing or invalid required header X-Vault-Token".to_string())))?.0.clone();


                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.

                let param_body_raw = req.get::<bodyparser::Raw>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - not valid UTF-8: {}", e))))?;
                let mut unused_elements = Vec::new();

                let param_body = if let Some(param_body_raw) = param_body_raw { 
                    let deserializer = &mut serde_json::Deserializer::from_str(&param_body_raw);

                    let param_body: Option<models::CreateTokenParameters> = serde_ignored::deserialize(deserializer, |path| {
                            warn!("Ignoring unknown field in body: {}", path);
                            unused_elements.push(path.to_string());
                        }).map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;

                    param_body
                } else {
                    None
                };
                let param_body = param_body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.create_token(param_x_vault_token, param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        CreateTokenResponse::Success(body) => {

                            let body_string = serde_json::to_string(&body).expect("impossible to fail to serialize");

                            let mut response = Response::with((status::Status::from_u16(200), body_string));
                            response.headers.set(ContentType(mimetypes::responses::CREATE_TOKEN_SUCCESS.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            if !unused_elements.is_empty() {
                                let warn = format!("Ignoring unknown fields in body: {:?}", unused_elements);
                                response.headers.set(Warning(warn));
                            }
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "CreateToken");

    let api_clone = api.clone();
    router.post(
        "/v1/auth/cert/login",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();




                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.

                let param_body_raw = req.get::<bodyparser::Raw>().unwrap_or(None);
                let mut unused_elements = Vec::new();

                let param_body = if let Some(param_body_raw) = param_body_raw {
                    let deserializer = &mut serde_json::Deserializer::from_str(&param_body_raw);

                    let param_body: Option<models::AuthCertLoginParameters> = serde_ignored::deserialize(deserializer, |path| {
                            warn!("Ignoring unknown field in body: {}", path);
                            unused_elements.push(path.to_string());
                        }).unwrap_or(None);

                    param_body
                } else {
                    None
                };;


                match api.log_in_with_tls_certificate(param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        LogInWithTLSCertificateResponse::Success(body) => {

                            let body_string = serde_json::to_string(&body).expect("impossible to fail to serialize");

                            let mut response = Response::with((status::Status::from_u16(200), body_string));
                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            if !unused_elements.is_empty() {
                                response.headers.set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                            }
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "LogInWithTLSCertificate");

    let api_clone = api.clone();
    router.post(
        "/v1/auth/token/renew-self",
        move |req: &mut Request| {
            let mut context = Context::default();

            // Helper function to provide a code block to use `?` in (to be replaced by the `catch` block when it exists).
            fn handle_request<T>(req: &mut Request, api: &T, context: &mut Context) -> Result<Response, Response> where T: Api {

                context.x_span_id = Some(req.headers.get::<XSpanId>().map(XSpanId::to_string).unwrap_or_else(|| self::uuid::Uuid::new_v4().to_string()));
                context.auth_data = req.extensions.remove::<AuthData>();
                context.authorization = req.extensions.remove::<Authorization>();



                // Header parameters
                let param_x_vault_token = req.headers.get::<RequestXVaultToken>().ok_or_else(|| Response::with((status::BadRequest, "Missing or invalid required header X-Vault-Token".to_string())))?.0.clone();


                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.

                let param_body_raw = req.get::<bodyparser::Raw>().map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - not valid UTF-8: {}", e))))?;
                let mut unused_elements = Vec::new();

                let param_body = if let Some(param_body_raw) = param_body_raw {
                    let deserializer = &mut serde_json::Deserializer::from_str(&param_body_raw);

                    let param_body: Option<models::RenewSelfParameters> = serde_ignored::deserialize(deserializer, |path| {
                            warn!("Ignoring unknown field in body: {}", path);
                            unused_elements.push(path.to_string());
                        }).map_err(|e| Response::with((status::BadRequest, format!("Couldn't parse body parameter body - doesn't match schema: {}", e))))?;

                    param_body
                } else {
                    None
                };
                let param_body = param_body.ok_or_else(|| Response::with((status::BadRequest, "Missing required body parameter body".to_string())))?;


                match api.renew_own_token(param_x_vault_token, param_body, context).wait() {
                    Ok(rsp) => match rsp {
                        RenewOwnTokenResponse::Success(body) => {

                            let body_string = serde_json::to_string(&body).expect("impossible to fail to serialize");

                            let mut response = Response::with((status::Status::from_u16(200), body_string));
                            response.headers.set(ContentType(mimetypes::responses::RENEW_OWN_TOKEN_SUCCESS.clone()));

                            context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                            if !unused_elements.is_empty() {
                                response.headers.set(Warning(format!("Ignoring unknown fields in body: {:?}", unused_elements)));
                            }
                            Ok(response)
                        },
                    },
                    Err(_) => {
                        // Application code returned an error. This should not happen, as the implementation should
                        // return a valid response.
                        Err(Response::with((status::InternalServerError, "An internal error occurred".to_string())))
                    }
                }
            }

            handle_request(req, &api_clone, &mut context).or_else(|mut response| {
                context.x_span_id.as_ref().map(|header| response.headers.set(XSpanId(header.clone())));
                Ok(response)
            })
        },
        "RenewOwnToken");

}

/// Middleware to extract authentication data from request
pub struct ExtractAuthData;

impl BeforeMiddleware for ExtractAuthData {
    fn before(&self, req: &mut Request) -> IronResult<()> {

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct RequestXVaultToken(pub String);

impl iron::headers::Header for RequestXVaultToken {
    fn header_name() -> &'static str {
        "X-Vault-Token"
    }
    fn parse_header(raw: &[Vec<u8>]) -> Result<Self, iron::error::HttpError> {
        iron::headers::parsing::from_one_raw_str(raw).map(RequestXVaultToken)
    }
}

impl iron::headers::HeaderFormat for RequestXVaultToken {
    fn fmt_header(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&*self.0, f)
    }
}


#[derive(Clone, Debug)]
pub struct Warning(pub String);

impl iron::headers::Header for Warning {
    fn header_name() -> &'static str {
        "Warning"
    }
    fn parse_header(raw: &[Vec<u8>]) -> Result<Self, iron::error::HttpError> {
        iron::headers::parsing::from_one_raw_str(raw).map(Warning)
    }
}

impl iron::headers::HeaderFormat for Warning {
    fn fmt_header(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&*self.0, f)
    }
}