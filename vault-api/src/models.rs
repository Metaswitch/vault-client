#![allow(unused_imports, unused_qualifications, unused_extern_crates)]
extern crate chrono;
extern crate uuid;


use serde::ser::Serializer;

use std::collections::HashMap;
use models;
use swagger;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCertLoginParameters {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

}

impl AuthCertLoginParameters {
    pub fn new() -> AuthCertLoginParameters {
        AuthCertLoginParameters {
            name: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthResponseAuth {
    #[serde(rename = "renewable")]
    pub renewable: bool,

    #[serde(rename = "lease_duration")]
    pub lease_duration: i32,

    #[serde(rename = "policies")]
    pub policies: Vec<String>,

    #[serde(rename = "accessor")]
    pub accessor: String,

    #[serde(rename = "client_token")]
    pub client_token: String,

}

impl AuthResponseAuth {
    pub fn new(renewable: bool, lease_duration: i32, policies: Vec<String>, accessor: String, client_token: String, ) -> AuthResponseAuth {
        AuthResponseAuth {
            renewable: renewable,
            lease_duration: lease_duration,
            policies: policies,
            accessor: accessor,
            client_token: client_token,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateResponseData {
    #[serde(rename = "certificate")]
    pub certificate: String,

}

impl CertificateResponseData {
    pub fn new(certificate: String, ) -> CertificateResponseData {
        CertificateResponseData {
            certificate: certificate,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonResponse {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "lease_duration")]
    pub lease_duration: i32,

    #[serde(rename = "lease_id")]
    pub lease_id: String,

    #[serde(rename = "renewable")]
    pub renewable: bool,

}

impl CommonResponse {
    pub fn new(request_id: String, lease_duration: i32, lease_id: String, renewable: bool, ) -> CommonResponse {
        CommonResponse {
            request_id: request_id,
            lease_duration: lease_duration,
            lease_id: lease_id,
            renewable: renewable,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTokenParameters {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "policies")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policies: Option<Vec<String>>,

    #[serde(rename = "no_parent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub no_parent: Option<bool>,

    #[serde(rename = "no_default_policy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub no_default_policy: Option<bool>,

    #[serde(rename = "renewable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub renewable: Option<bool>,

    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ttl: Option<String>,

    #[serde(rename = "explicit_max_ttl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub explicit_max_ttl: Option<bool>,

    #[serde(rename = "display_name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "num_uses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub num_uses: Option<i32>,

    #[serde(rename = "period")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub period: Option<String>,

}

impl CreateTokenParameters {
    pub fn new() -> CreateTokenParameters {
        CreateTokenParameters {
            id: None,
            policies: None,
            no_parent: None,
            no_default_policy: None,
            renewable: None,
            ttl: None,
            explicit_max_ttl: None,
            display_name: None,
            num_uses: None,
            period: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateCertificateParameters {
    #[serde(rename = "common_name")]
    pub common_name: String,

    #[serde(rename = "alt_names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub alt_names: Option<String>,

    #[serde(rename = "ip_sans")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ip_sans: Option<String>,

    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ttl: Option<String>,

    #[serde(rename = "format")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub format: Option<String>,

    #[serde(rename = "exclude_cn_from_sans")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclude_cn_from_sans: Option<bool>,

}

impl GenerateCertificateParameters {
    pub fn new(common_name: String, ) -> GenerateCertificateParameters {
        GenerateCertificateParameters {
            common_name: common_name,
            alt_names: None,
            ip_sans: None,
            ttl: None,
            format: None,
            exclude_cn_from_sans: Some(false),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateCertificateResponseData {
    #[serde(rename = "certificate")]
    pub certificate: String,

    #[serde(rename = "issuing_ca")]
    pub issuing_ca: String,

    #[serde(rename = "ca_chain")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ca_chain: Option<Vec<String>>,

    #[serde(rename = "private_key")]
    pub private_key: String,

    #[serde(rename = "private_key_type")]
    pub private_key_type: String,

    #[serde(rename = "serial_number")]
    pub serial_number: String,

}

impl GenerateCertificateResponseData {
    pub fn new(certificate: String, issuing_ca: String, private_key: String, private_key_type: String, serial_number: String, ) -> GenerateCertificateResponseData {
        GenerateCertificateResponseData {
            certificate: certificate,
            issuing_ca: issuing_ca,
            ca_chain: None,
            private_key: private_key,
            private_key_type: private_key_type,
            serial_number: serial_number,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenewSelfParameters {
    #[serde(rename = "increment")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub increment: Option<String>,

}

impl RenewSelfParameters {
    pub fn new() -> RenewSelfParameters {
        RenewSelfParameters {
            increment: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevokeLeaseParameters {
    #[serde(rename = "lease_id")]
    pub lease_id: String,

}

impl RevokeLeaseParameters {
    pub fn new(lease_id: String, ) -> RevokeLeaseParameters {
        RevokeLeaseParameters {
            lease_id: lease_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthResponse {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "lease_duration")]
    pub lease_duration: i32,

    #[serde(rename = "lease_id")]
    pub lease_id: String,

    #[serde(rename = "renewable")]
    pub renewable: bool,

    #[serde(rename = "auth")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth: Option<models::AuthResponseAuth>,

}

impl AuthResponse {
    pub fn new(request_id: String, lease_duration: i32, lease_id: String, renewable: bool, ) -> AuthResponse {
        AuthResponse {
            request_id: request_id,
            lease_duration: lease_duration,
            lease_id: lease_id,
            renewable: renewable,
            auth: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateResponse {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "lease_duration")]
    pub lease_duration: i32,

    #[serde(rename = "lease_id")]
    pub lease_id: String,

    #[serde(rename = "renewable")]
    pub renewable: bool,

    #[serde(rename = "data")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<models::CertificateResponseData>,

}

impl CertificateResponse {
    pub fn new(request_id: String, lease_duration: i32, lease_id: String, renewable: bool, ) -> CertificateResponse {
        CertificateResponse {
            request_id: request_id,
            lease_duration: lease_duration,
            lease_id: lease_id,
            renewable: renewable,
            data: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateCertificateResponse {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "lease_duration")]
    pub lease_duration: i32,

    #[serde(rename = "lease_id")]
    pub lease_id: String,

    #[serde(rename = "renewable")]
    pub renewable: bool,

    #[serde(rename = "data")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data: Option<models::GenerateCertificateResponseData>,

}

impl GenerateCertificateResponse {
    pub fn new(request_id: String, lease_duration: i32, lease_id: String, renewable: bool, ) -> GenerateCertificateResponse {
        GenerateCertificateResponse {
            request_id: request_id,
            lease_duration: lease_duration,
            lease_id: lease_id,
            renewable: renewable,
            data: None,
        }
    }
}
