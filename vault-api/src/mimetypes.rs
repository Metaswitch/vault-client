/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for GenerateCert
    lazy_static! {
        pub static ref GENERATE_CERT_SUCCESS: Mime = mime!(Application/Json);
    }
    /// Create Mime objects for the response content types for ReadCert
    lazy_static! {
        pub static ref READ_CERT_SUCCESS: Mime = mime!(Application/Json);
    }
    /// Create Mime objects for the response content types for CreateOrphanToken
    lazy_static! {
        pub static ref CREATE_ORPHAN_TOKEN_SUCCESS: Mime = mime!(Application/Json);
    }
    /// Create Mime objects for the response content types for CreateToken
    lazy_static! {
        pub static ref CREATE_TOKEN_SUCCESS: Mime = mime!(Application/Json);
    }
    /// Create Mime objects for the response content types for RenewOwnToken
    lazy_static! {
        pub static ref RENEW_OWN_TOKEN_SUCCESS: Mime = mime!(Application/Json);
    }

}

pub mod requests {
    use hyper::mime::*;
   /// Create Mime objects for the request content types for SysLeasesRevokePut
    lazy_static! {
        pub static ref SYS_LEASES_REVOKE_PUT: Mime = mime!(Application/Json);
    }
   /// Create Mime objects for the request content types for GenerateCert
    lazy_static! {
        pub static ref GENERATE_CERT: Mime = mime!(Application/Json);
    }
   /// Create Mime objects for the request content types for CreateOrphanToken
    lazy_static! {
        pub static ref CREATE_ORPHAN_TOKEN: Mime = mime!(Application/Json);
    }
   /// Create Mime objects for the request content types for CreateToken
    lazy_static! {
        pub static ref CREATE_TOKEN: Mime = mime!(Application/Json);
    }
   /// Create Mime objects for the request content types for LogInWithTLSCertificate
    lazy_static! {
        pub static ref LOG_IN_WITH_TLS_CERTIFICATE: Mime = mime!(Application/Json);
    }
   /// Create Mime objects for the request content types for RenewOwnToken
    lazy_static! {
        pub static ref RENEW_OWN_TOKEN: Mime = mime!(Application/Json);
    }

}
