#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;


// Methods for converting between header::IntoHeaderValue<AuthCertLoginParameters> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AuthCertLoginParameters>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AuthCertLoginParameters>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AuthCertLoginParameters - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AuthCertLoginParameters> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AuthCertLoginParameters as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AuthCertLoginParameters - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

/// Converts the AuthCertLoginParameters value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AuthCertLoginParameters {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref name) = self.name {
            params.push("name".to_string());
            params.push(name.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AuthCertLoginParameters value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AuthCertLoginParameters {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AuthCertLoginParameters".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "name" => intermediate_rep.name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AuthCertLoginParameters".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AuthCertLoginParameters {
            name: intermediate_rep.name.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<AuthResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AuthResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AuthResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AuthResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AuthResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AuthResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AuthResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AuthResponse {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "lease_duration")]
    pub lease_duration: isize,

    #[serde(rename = "lease_id")]
    pub lease_id: String,

    #[serde(rename = "renewable")]
    pub renewable: bool,

    #[serde(rename = "auth")]
    pub auth: models::AuthResponseAllOfAuth,

}

impl AuthResponse {
    pub fn new(request_id: String, lease_duration: isize, lease_id: String, renewable: bool, auth: models::AuthResponseAllOfAuth, ) -> AuthResponse {
        AuthResponse {
            request_id: request_id,
            lease_duration: lease_duration,
            lease_id: lease_id,
            renewable: renewable,
            auth: auth,
        }
    }
}

/// Converts the AuthResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AuthResponse {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("request_id".to_string());
        params.push(self.request_id.to_string());


        params.push("lease_duration".to_string());
        params.push(self.lease_duration.to_string());


        params.push("lease_id".to_string());
        params.push(self.lease_id.to_string());


        params.push("renewable".to_string());
        params.push(self.renewable.to_string());

        // Skipping auth in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AuthResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AuthResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub request_id: Vec<String>,
            pub lease_duration: Vec<isize>,
            pub lease_id: Vec<String>,
            pub renewable: Vec<bool>,
            pub auth: Vec<models::AuthResponseAllOfAuth>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AuthResponse".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "request_id" => intermediate_rep.request_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "lease_duration" => intermediate_rep.lease_duration.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "lease_id" => intermediate_rep.lease_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "renewable" => intermediate_rep.renewable.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "auth" => intermediate_rep.auth.push(models::AuthResponseAllOfAuth::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AuthResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AuthResponse {
            request_id: intermediate_rep.request_id.into_iter().next().ok_or("request_id missing in AuthResponse".to_string())?,
            lease_duration: intermediate_rep.lease_duration.into_iter().next().ok_or("lease_duration missing in AuthResponse".to_string())?,
            lease_id: intermediate_rep.lease_id.into_iter().next().ok_or("lease_id missing in AuthResponse".to_string())?,
            renewable: intermediate_rep.renewable.into_iter().next().ok_or("renewable missing in AuthResponse".to_string())?,
            auth: intermediate_rep.auth.into_iter().next().ok_or("auth missing in AuthResponse".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<AuthResponseAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AuthResponseAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AuthResponseAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AuthResponseAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AuthResponseAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AuthResponseAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AuthResponseAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AuthResponseAllOf {
    #[serde(rename = "auth")]
    pub auth: models::AuthResponseAllOfAuth,

}

impl AuthResponseAllOf {
    pub fn new(auth: models::AuthResponseAllOfAuth, ) -> AuthResponseAllOf {
        AuthResponseAllOf {
            auth: auth,
        }
    }
}

/// Converts the AuthResponseAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AuthResponseAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping auth in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AuthResponseAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AuthResponseAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub auth: Vec<models::AuthResponseAllOfAuth>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AuthResponseAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "auth" => intermediate_rep.auth.push(models::AuthResponseAllOfAuth::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AuthResponseAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AuthResponseAllOf {
            auth: intermediate_rep.auth.into_iter().next().ok_or("auth missing in AuthResponseAllOf".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<AuthResponseAllOfAuth> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AuthResponseAllOfAuth>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AuthResponseAllOfAuth>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AuthResponseAllOfAuth - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AuthResponseAllOfAuth> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AuthResponseAllOfAuth as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AuthResponseAllOfAuth - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AuthResponseAllOfAuth {
    #[serde(rename = "renewable")]
    pub renewable: bool,

    #[serde(rename = "lease_duration")]
    pub lease_duration: isize,

    #[serde(rename = "policies")]
    pub policies: Vec<String>,

    #[serde(rename = "accessor")]
    pub accessor: String,

    #[serde(rename = "client_token")]
    pub client_token: String,

}

impl AuthResponseAllOfAuth {
    pub fn new(renewable: bool, lease_duration: isize, policies: Vec<String>, accessor: String, client_token: String, ) -> AuthResponseAllOfAuth {
        AuthResponseAllOfAuth {
            renewable: renewable,
            lease_duration: lease_duration,
            policies: policies,
            accessor: accessor,
            client_token: client_token,
        }
    }
}

/// Converts the AuthResponseAllOfAuth value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AuthResponseAllOfAuth {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("renewable".to_string());
        params.push(self.renewable.to_string());


        params.push("lease_duration".to_string());
        params.push(self.lease_duration.to_string());


        params.push("policies".to_string());
        params.push(self.policies.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());


        params.push("accessor".to_string());
        params.push(self.accessor.to_string());


        params.push("client_token".to_string());
        params.push(self.client_token.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AuthResponseAllOfAuth value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AuthResponseAllOfAuth {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub renewable: Vec<bool>,
            pub lease_duration: Vec<isize>,
            pub policies: Vec<Vec<String>>,
            pub accessor: Vec<String>,
            pub client_token: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AuthResponseAllOfAuth".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "renewable" => intermediate_rep.renewable.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "lease_duration" => intermediate_rep.lease_duration.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "policies" => return std::result::Result::Err("Parsing a container in this style is not supported in AuthResponseAllOfAuth".to_string()),
                    "accessor" => intermediate_rep.accessor.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "client_token" => intermediate_rep.client_token.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AuthResponseAllOfAuth".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AuthResponseAllOfAuth {
            renewable: intermediate_rep.renewable.into_iter().next().ok_or("renewable missing in AuthResponseAllOfAuth".to_string())?,
            lease_duration: intermediate_rep.lease_duration.into_iter().next().ok_or("lease_duration missing in AuthResponseAllOfAuth".to_string())?,
            policies: intermediate_rep.policies.into_iter().next().ok_or("policies missing in AuthResponseAllOfAuth".to_string())?,
            accessor: intermediate_rep.accessor.into_iter().next().ok_or("accessor missing in AuthResponseAllOfAuth".to_string())?,
            client_token: intermediate_rep.client_token.into_iter().next().ok_or("client_token missing in AuthResponseAllOfAuth".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<CertificateResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CertificateResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CertificateResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CertificateResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CertificateResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CertificateResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CertificateResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CertificateResponse {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "lease_duration")]
    pub lease_duration: isize,

    #[serde(rename = "lease_id")]
    pub lease_id: String,

    #[serde(rename = "renewable")]
    pub renewable: bool,

    #[serde(rename = "data")]
    pub data: models::CertificateResponseAllOfData,

}

impl CertificateResponse {
    pub fn new(request_id: String, lease_duration: isize, lease_id: String, renewable: bool, data: models::CertificateResponseAllOfData, ) -> CertificateResponse {
        CertificateResponse {
            request_id: request_id,
            lease_duration: lease_duration,
            lease_id: lease_id,
            renewable: renewable,
            data: data,
        }
    }
}

/// Converts the CertificateResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CertificateResponse {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("request_id".to_string());
        params.push(self.request_id.to_string());


        params.push("lease_duration".to_string());
        params.push(self.lease_duration.to_string());


        params.push("lease_id".to_string());
        params.push(self.lease_id.to_string());


        params.push("renewable".to_string());
        params.push(self.renewable.to_string());

        // Skipping data in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CertificateResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CertificateResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub request_id: Vec<String>,
            pub lease_duration: Vec<isize>,
            pub lease_id: Vec<String>,
            pub renewable: Vec<bool>,
            pub data: Vec<models::CertificateResponseAllOfData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CertificateResponse".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "request_id" => intermediate_rep.request_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "lease_duration" => intermediate_rep.lease_duration.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "lease_id" => intermediate_rep.lease_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "renewable" => intermediate_rep.renewable.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "data" => intermediate_rep.data.push(models::CertificateResponseAllOfData::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CertificateResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CertificateResponse {
            request_id: intermediate_rep.request_id.into_iter().next().ok_or("request_id missing in CertificateResponse".to_string())?,
            lease_duration: intermediate_rep.lease_duration.into_iter().next().ok_or("lease_duration missing in CertificateResponse".to_string())?,
            lease_id: intermediate_rep.lease_id.into_iter().next().ok_or("lease_id missing in CertificateResponse".to_string())?,
            renewable: intermediate_rep.renewable.into_iter().next().ok_or("renewable missing in CertificateResponse".to_string())?,
            data: intermediate_rep.data.into_iter().next().ok_or("data missing in CertificateResponse".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<CertificateResponseAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CertificateResponseAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CertificateResponseAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CertificateResponseAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CertificateResponseAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CertificateResponseAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CertificateResponseAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CertificateResponseAllOf {
    #[serde(rename = "data")]
    pub data: models::CertificateResponseAllOfData,

}

impl CertificateResponseAllOf {
    pub fn new(data: models::CertificateResponseAllOfData, ) -> CertificateResponseAllOf {
        CertificateResponseAllOf {
            data: data,
        }
    }
}

/// Converts the CertificateResponseAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CertificateResponseAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping data in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CertificateResponseAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CertificateResponseAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub data: Vec<models::CertificateResponseAllOfData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CertificateResponseAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "data" => intermediate_rep.data.push(models::CertificateResponseAllOfData::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CertificateResponseAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CertificateResponseAllOf {
            data: intermediate_rep.data.into_iter().next().ok_or("data missing in CertificateResponseAllOf".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<CertificateResponseAllOfData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CertificateResponseAllOfData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CertificateResponseAllOfData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CertificateResponseAllOfData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CertificateResponseAllOfData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CertificateResponseAllOfData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CertificateResponseAllOfData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CertificateResponseAllOfData {
    #[serde(rename = "certificate")]
    pub certificate: String,

}

impl CertificateResponseAllOfData {
    pub fn new(certificate: String, ) -> CertificateResponseAllOfData {
        CertificateResponseAllOfData {
            certificate: certificate,
        }
    }
}

/// Converts the CertificateResponseAllOfData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CertificateResponseAllOfData {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("certificate".to_string());
        params.push(self.certificate.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CertificateResponseAllOfData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CertificateResponseAllOfData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub certificate: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CertificateResponseAllOfData".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "certificate" => intermediate_rep.certificate.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CertificateResponseAllOfData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CertificateResponseAllOfData {
            certificate: intermediate_rep.certificate.into_iter().next().ok_or("certificate missing in CertificateResponseAllOfData".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<CommonResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CommonResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CommonResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CommonResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CommonResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CommonResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CommonResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CommonResponse {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "lease_duration")]
    pub lease_duration: isize,

    #[serde(rename = "lease_id")]
    pub lease_id: String,

    #[serde(rename = "renewable")]
    pub renewable: bool,

}

impl CommonResponse {
    pub fn new(request_id: String, lease_duration: isize, lease_id: String, renewable: bool, ) -> CommonResponse {
        CommonResponse {
            request_id: request_id,
            lease_duration: lease_duration,
            lease_id: lease_id,
            renewable: renewable,
        }
    }
}

/// Converts the CommonResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CommonResponse {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("request_id".to_string());
        params.push(self.request_id.to_string());


        params.push("lease_duration".to_string());
        params.push(self.lease_duration.to_string());


        params.push("lease_id".to_string());
        params.push(self.lease_id.to_string());


        params.push("renewable".to_string());
        params.push(self.renewable.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CommonResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CommonResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub request_id: Vec<String>,
            pub lease_duration: Vec<isize>,
            pub lease_id: Vec<String>,
            pub renewable: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CommonResponse".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "request_id" => intermediate_rep.request_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "lease_duration" => intermediate_rep.lease_duration.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "lease_id" => intermediate_rep.lease_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "renewable" => intermediate_rep.renewable.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CommonResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CommonResponse {
            request_id: intermediate_rep.request_id.into_iter().next().ok_or("request_id missing in CommonResponse".to_string())?,
            lease_duration: intermediate_rep.lease_duration.into_iter().next().ok_or("lease_duration missing in CommonResponse".to_string())?,
            lease_id: intermediate_rep.lease_id.into_iter().next().ok_or("lease_id missing in CommonResponse".to_string())?,
            renewable: intermediate_rep.renewable.into_iter().next().ok_or("renewable missing in CommonResponse".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<CreateTokenParameters> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CreateTokenParameters>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CreateTokenParameters>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CreateTokenParameters - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CreateTokenParameters> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CreateTokenParameters as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CreateTokenParameters - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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
    pub num_uses: Option<isize>,

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

/// Converts the CreateTokenParameters value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CreateTokenParameters {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref id) = self.id {
            params.push("id".to_string());
            params.push(id.to_string());
        }


        if let Some(ref policies) = self.policies {
            params.push("policies".to_string());
            params.push(policies.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());
        }


        if let Some(ref no_parent) = self.no_parent {
            params.push("no_parent".to_string());
            params.push(no_parent.to_string());
        }


        if let Some(ref no_default_policy) = self.no_default_policy {
            params.push("no_default_policy".to_string());
            params.push(no_default_policy.to_string());
        }


        if let Some(ref renewable) = self.renewable {
            params.push("renewable".to_string());
            params.push(renewable.to_string());
        }


        if let Some(ref ttl) = self.ttl {
            params.push("ttl".to_string());
            params.push(ttl.to_string());
        }


        if let Some(ref explicit_max_ttl) = self.explicit_max_ttl {
            params.push("explicit_max_ttl".to_string());
            params.push(explicit_max_ttl.to_string());
        }


        if let Some(ref display_name) = self.display_name {
            params.push("display_name".to_string());
            params.push(display_name.to_string());
        }


        if let Some(ref num_uses) = self.num_uses {
            params.push("num_uses".to_string());
            params.push(num_uses.to_string());
        }


        if let Some(ref period) = self.period {
            params.push("period".to_string());
            params.push(period.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CreateTokenParameters value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CreateTokenParameters {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub id: Vec<String>,
            pub policies: Vec<Vec<String>>,
            pub no_parent: Vec<bool>,
            pub no_default_policy: Vec<bool>,
            pub renewable: Vec<bool>,
            pub ttl: Vec<String>,
            pub explicit_max_ttl: Vec<bool>,
            pub display_name: Vec<String>,
            pub num_uses: Vec<isize>,
            pub period: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CreateTokenParameters".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "id" => intermediate_rep.id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "policies" => return std::result::Result::Err("Parsing a container in this style is not supported in CreateTokenParameters".to_string()),
                    "no_parent" => intermediate_rep.no_parent.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "no_default_policy" => intermediate_rep.no_default_policy.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "renewable" => intermediate_rep.renewable.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "ttl" => intermediate_rep.ttl.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "explicit_max_ttl" => intermediate_rep.explicit_max_ttl.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "display_name" => intermediate_rep.display_name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "num_uses" => intermediate_rep.num_uses.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "period" => intermediate_rep.period.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CreateTokenParameters".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CreateTokenParameters {
            id: intermediate_rep.id.into_iter().next(),
            policies: intermediate_rep.policies.into_iter().next(),
            no_parent: intermediate_rep.no_parent.into_iter().next(),
            no_default_policy: intermediate_rep.no_default_policy.into_iter().next(),
            renewable: intermediate_rep.renewable.into_iter().next(),
            ttl: intermediate_rep.ttl.into_iter().next(),
            explicit_max_ttl: intermediate_rep.explicit_max_ttl.into_iter().next(),
            display_name: intermediate_rep.display_name.into_iter().next(),
            num_uses: intermediate_rep.num_uses.into_iter().next(),
            period: intermediate_rep.period.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<GenerateCertificateParameters> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GenerateCertificateParameters>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GenerateCertificateParameters>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GenerateCertificateParameters - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GenerateCertificateParameters> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GenerateCertificateParameters as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GenerateCertificateParameters - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

/// Converts the GenerateCertificateParameters value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GenerateCertificateParameters {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("common_name".to_string());
        params.push(self.common_name.to_string());


        if let Some(ref alt_names) = self.alt_names {
            params.push("alt_names".to_string());
            params.push(alt_names.to_string());
        }


        if let Some(ref ip_sans) = self.ip_sans {
            params.push("ip_sans".to_string());
            params.push(ip_sans.to_string());
        }


        if let Some(ref ttl) = self.ttl {
            params.push("ttl".to_string());
            params.push(ttl.to_string());
        }


        if let Some(ref format) = self.format {
            params.push("format".to_string());
            params.push(format.to_string());
        }


        if let Some(ref exclude_cn_from_sans) = self.exclude_cn_from_sans {
            params.push("exclude_cn_from_sans".to_string());
            params.push(exclude_cn_from_sans.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GenerateCertificateParameters value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GenerateCertificateParameters {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub common_name: Vec<String>,
            pub alt_names: Vec<String>,
            pub ip_sans: Vec<String>,
            pub ttl: Vec<String>,
            pub format: Vec<String>,
            pub exclude_cn_from_sans: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GenerateCertificateParameters".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "common_name" => intermediate_rep.common_name.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "alt_names" => intermediate_rep.alt_names.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "ip_sans" => intermediate_rep.ip_sans.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "ttl" => intermediate_rep.ttl.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "format" => intermediate_rep.format.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "exclude_cn_from_sans" => intermediate_rep.exclude_cn_from_sans.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GenerateCertificateParameters".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GenerateCertificateParameters {
            common_name: intermediate_rep.common_name.into_iter().next().ok_or("common_name missing in GenerateCertificateParameters".to_string())?,
            alt_names: intermediate_rep.alt_names.into_iter().next(),
            ip_sans: intermediate_rep.ip_sans.into_iter().next(),
            ttl: intermediate_rep.ttl.into_iter().next(),
            format: intermediate_rep.format.into_iter().next(),
            exclude_cn_from_sans: intermediate_rep.exclude_cn_from_sans.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<GenerateCertificateResponse> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GenerateCertificateResponse>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GenerateCertificateResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GenerateCertificateResponse - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GenerateCertificateResponse> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GenerateCertificateResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GenerateCertificateResponse - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GenerateCertificateResponse {
    #[serde(rename = "request_id")]
    pub request_id: String,

    #[serde(rename = "lease_duration")]
    pub lease_duration: isize,

    #[serde(rename = "lease_id")]
    pub lease_id: String,

    #[serde(rename = "renewable")]
    pub renewable: bool,

    #[serde(rename = "data")]
    pub data: models::GenerateCertificateResponseAllOfData,

}

impl GenerateCertificateResponse {
    pub fn new(request_id: String, lease_duration: isize, lease_id: String, renewable: bool, data: models::GenerateCertificateResponseAllOfData, ) -> GenerateCertificateResponse {
        GenerateCertificateResponse {
            request_id: request_id,
            lease_duration: lease_duration,
            lease_id: lease_id,
            renewable: renewable,
            data: data,
        }
    }
}

/// Converts the GenerateCertificateResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GenerateCertificateResponse {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("request_id".to_string());
        params.push(self.request_id.to_string());


        params.push("lease_duration".to_string());
        params.push(self.lease_duration.to_string());


        params.push("lease_id".to_string());
        params.push(self.lease_id.to_string());


        params.push("renewable".to_string());
        params.push(self.renewable.to_string());

        // Skipping data in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GenerateCertificateResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GenerateCertificateResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub request_id: Vec<String>,
            pub lease_duration: Vec<isize>,
            pub lease_id: Vec<String>,
            pub renewable: Vec<bool>,
            pub data: Vec<models::GenerateCertificateResponseAllOfData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GenerateCertificateResponse".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "request_id" => intermediate_rep.request_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "lease_duration" => intermediate_rep.lease_duration.push(isize::from_str(val).map_err(|x| format!("{}", x))?),
                    "lease_id" => intermediate_rep.lease_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "renewable" => intermediate_rep.renewable.push(bool::from_str(val).map_err(|x| format!("{}", x))?),
                    "data" => intermediate_rep.data.push(models::GenerateCertificateResponseAllOfData::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GenerateCertificateResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GenerateCertificateResponse {
            request_id: intermediate_rep.request_id.into_iter().next().ok_or("request_id missing in GenerateCertificateResponse".to_string())?,
            lease_duration: intermediate_rep.lease_duration.into_iter().next().ok_or("lease_duration missing in GenerateCertificateResponse".to_string())?,
            lease_id: intermediate_rep.lease_id.into_iter().next().ok_or("lease_id missing in GenerateCertificateResponse".to_string())?,
            renewable: intermediate_rep.renewable.into_iter().next().ok_or("renewable missing in GenerateCertificateResponse".to_string())?,
            data: intermediate_rep.data.into_iter().next().ok_or("data missing in GenerateCertificateResponse".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<GenerateCertificateResponseAllOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GenerateCertificateResponseAllOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GenerateCertificateResponseAllOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GenerateCertificateResponseAllOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GenerateCertificateResponseAllOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GenerateCertificateResponseAllOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GenerateCertificateResponseAllOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GenerateCertificateResponseAllOf {
    #[serde(rename = "data")]
    pub data: models::GenerateCertificateResponseAllOfData,

}

impl GenerateCertificateResponseAllOf {
    pub fn new(data: models::GenerateCertificateResponseAllOfData, ) -> GenerateCertificateResponseAllOf {
        GenerateCertificateResponseAllOf {
            data: data,
        }
    }
}

/// Converts the GenerateCertificateResponseAllOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GenerateCertificateResponseAllOf {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping data in query parameter serialization

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GenerateCertificateResponseAllOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GenerateCertificateResponseAllOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub data: Vec<models::GenerateCertificateResponseAllOfData>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GenerateCertificateResponseAllOf".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "data" => intermediate_rep.data.push(models::GenerateCertificateResponseAllOfData::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GenerateCertificateResponseAllOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GenerateCertificateResponseAllOf {
            data: intermediate_rep.data.into_iter().next().ok_or("data missing in GenerateCertificateResponseAllOf".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<GenerateCertificateResponseAllOfData> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<GenerateCertificateResponseAllOfData>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<GenerateCertificateResponseAllOfData>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for GenerateCertificateResponseAllOfData - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<GenerateCertificateResponseAllOfData> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <GenerateCertificateResponseAllOfData as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into GenerateCertificateResponseAllOfData - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GenerateCertificateResponseAllOfData {
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

impl GenerateCertificateResponseAllOfData {
    pub fn new(certificate: String, issuing_ca: String, private_key: String, private_key_type: String, serial_number: String, ) -> GenerateCertificateResponseAllOfData {
        GenerateCertificateResponseAllOfData {
            certificate: certificate,
            issuing_ca: issuing_ca,
            ca_chain: None,
            private_key: private_key,
            private_key_type: private_key_type,
            serial_number: serial_number,
        }
    }
}

/// Converts the GenerateCertificateResponseAllOfData value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for GenerateCertificateResponseAllOfData {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("certificate".to_string());
        params.push(self.certificate.to_string());


        params.push("issuing_ca".to_string());
        params.push(self.issuing_ca.to_string());


        if let Some(ref ca_chain) = self.ca_chain {
            params.push("ca_chain".to_string());
            params.push(ca_chain.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());
        }


        params.push("private_key".to_string());
        params.push(self.private_key.to_string());


        params.push("private_key_type".to_string());
        params.push(self.private_key_type.to_string());


        params.push("serial_number".to_string());
        params.push(self.serial_number.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a GenerateCertificateResponseAllOfData value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for GenerateCertificateResponseAllOfData {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub certificate: Vec<String>,
            pub issuing_ca: Vec<String>,
            pub ca_chain: Vec<Vec<String>>,
            pub private_key: Vec<String>,
            pub private_key_type: Vec<String>,
            pub serial_number: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing GenerateCertificateResponseAllOfData".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "certificate" => intermediate_rep.certificate.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "issuing_ca" => intermediate_rep.issuing_ca.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "ca_chain" => return std::result::Result::Err("Parsing a container in this style is not supported in GenerateCertificateResponseAllOfData".to_string()),
                    "private_key" => intermediate_rep.private_key.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "private_key_type" => intermediate_rep.private_key_type.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    "serial_number" => intermediate_rep.serial_number.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing GenerateCertificateResponseAllOfData".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(GenerateCertificateResponseAllOfData {
            certificate: intermediate_rep.certificate.into_iter().next().ok_or("certificate missing in GenerateCertificateResponseAllOfData".to_string())?,
            issuing_ca: intermediate_rep.issuing_ca.into_iter().next().ok_or("issuing_ca missing in GenerateCertificateResponseAllOfData".to_string())?,
            ca_chain: intermediate_rep.ca_chain.into_iter().next(),
            private_key: intermediate_rep.private_key.into_iter().next().ok_or("private_key missing in GenerateCertificateResponseAllOfData".to_string())?,
            private_key_type: intermediate_rep.private_key_type.into_iter().next().ok_or("private_key_type missing in GenerateCertificateResponseAllOfData".to_string())?,
            serial_number: intermediate_rep.serial_number.into_iter().next().ok_or("serial_number missing in GenerateCertificateResponseAllOfData".to_string())?,
        })
    }
}



// Methods for converting between header::IntoHeaderValue<RenewSelfParameters> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RenewSelfParameters>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RenewSelfParameters>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RenewSelfParameters - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RenewSelfParameters> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RenewSelfParameters as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RenewSelfParameters - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

/// Converts the RenewSelfParameters value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RenewSelfParameters {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref increment) = self.increment {
            params.push("increment".to_string());
            params.push(increment.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RenewSelfParameters value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RenewSelfParameters {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub increment: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RenewSelfParameters".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "increment" => intermediate_rep.increment.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RenewSelfParameters".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RenewSelfParameters {
            increment: intermediate_rep.increment.into_iter().next(),
        })
    }
}



// Methods for converting between header::IntoHeaderValue<RevokeLeaseParameters> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RevokeLeaseParameters>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RevokeLeaseParameters>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RevokeLeaseParameters - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RevokeLeaseParameters> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RevokeLeaseParameters as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RevokeLeaseParameters - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
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

/// Converts the RevokeLeaseParameters value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RevokeLeaseParameters {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("lease_id".to_string());
        params.push(self.lease_id.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RevokeLeaseParameters value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RevokeLeaseParameters {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub lease_id: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RevokeLeaseParameters".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "lease_id" => intermediate_rep.lease_id.push(String::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RevokeLeaseParameters".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RevokeLeaseParameters {
            lease_id: intermediate_rep.lease_id.into_iter().next().ok_or("lease_id missing in RevokeLeaseParameters".to_string())?,
        })
    }
}


