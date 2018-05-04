use serde::ser::Serializer;
use serde::de::{Deserialize, Deserializer, Error};
use base64::{encode, decode};

// These functions are only used if the API uses base64-encoded properties, so allow them to be
// dead code.

#[allow(dead_code)]
pub fn serialize_with<S>(obj: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    serializer.serialize_str(&encode(obj))
}

#[allow(dead_code)]
pub fn deserialize_with<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where D: Deserializer<'de>
{
    let s = try!(String::deserialize(deserializer));
    match decode(&s) {
        Ok(bin) => Ok(bin),
        _ => Err(D::Error::custom("invalid base64")),
    }
}
