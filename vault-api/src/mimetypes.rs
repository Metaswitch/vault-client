/// mime types for requests and responses

#[cfg(feature = "server")]
pub mod responses {
    extern crate iron;
    use self::iron::mime::{Mime, TopLevel, SubLevel};

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for GenerateCert
    lazy_static! {
        pub static ref GENERATE_CERT_SUCCESS: Mime = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
    }
    /// Create Mime objects for the response content types for ReadCert
    lazy_static! {
        pub static ref READ_CERT_SUCCESS: Mime = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
    }
    /// Create Mime objects for the response content types for CreateOrphanToken
    lazy_static! {
        pub static ref CREATE_ORPHAN_TOKEN_SUCCESS: Mime = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
    }
    /// Create Mime objects for the response content types for CreateToken
    lazy_static! {
        pub static ref CREATE_TOKEN_SUCCESS: Mime = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
    }
    /// Create Mime objects for the response content types for RenewOwnToken
    lazy_static! {
        pub static ref RENEW_OWN_TOKEN_SUCCESS: Mime = Mime(TopLevel::Application, SubLevel::Json, Vec::new());
    }

}

pub mod requests {
    use mime::{APPLICATION_JSON, Mime};
    use hyper::header::HeaderValue;

    use http;

   /// Create Mime objects for the request content types for SysLeasesRevokePut
    lazy_static! {
        pub static ref SYS_LEASES_REVOKE_PUT: HeaderValue = HeaderValue::from_str(APPLICATION_JSON.as_ref()).unwrap();
    }
   /// Create Mime objects for the request content types for GenerateCert
    lazy_static! {
        pub static ref GENERATE_CERT: HeaderValue = HeaderValue::from_str(APPLICATION_JSON.as_ref()).unwrap();
    }
   /// Create Mime objects for the request content types for CreateOrphanToken
    lazy_static! {
        pub static ref CREATE_ORPHAN_TOKEN: HeaderValue = HeaderValue::from_str(APPLICATION_JSON.as_ref()).unwrap();
    }
   /// Create Mime objects for the request content types for CreateToken
    lazy_static! {
        pub static ref CREATE_TOKEN: HeaderValue = HeaderValue::from_str(APPLICATION_JSON.as_ref()).unwrap();
    }
   /// Create Mime objects for the request content types for LogInWithTLSCertificate
    lazy_static! {
        pub static ref LOG_IN_WITH_TLS_CERTIFICATE: HeaderValue = HeaderValue::from_str(APPLICATION_JSON.as_ref()).unwrap();
    }
   /// Create Mime objects for the request content types for RenewOwnToken
    lazy_static! {
        pub static ref RENEW_OWN_TOKEN: HeaderValue = HeaderValue::from_str(APPLICATION_JSON.as_ref()).unwrap();
    }

}
