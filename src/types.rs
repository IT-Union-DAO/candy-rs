use crate::value::CandyValue;
use candid::CandidType;
use serde::{Deserialize, Serialize};
pub type Properties = Vec<Property>;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: CandyValue,
    pub immutable: bool,
}

impl ToString for Property {
    fn to_string(&self) -> String {
        let desc = match self.immutable {
            true => "var ".to_string(),
            false => "".to_string(),
        };
        format!("{}:{}{}; ", self.name, desc, self.value.clone().to_string())
    }
}

impl Property {
    pub fn stringify_properties(props: &[Property]) -> String {
        let prop_strings: Vec<String> = props.iter().map(|p| p.to_string()).collect();
        format!("{{{}}}", prop_strings.join("").trim_end())
    }

    fn to_json(self) -> String {
        format!("\"{}\":{}", self.name, self.value.to_json())
    }

    pub fn props_to_json(props: &[Property]) -> String {
        let prop_strings: Vec<String> = props.iter().map(|p| p.clone().to_json()).collect();
        format!("{{{}}}", prop_strings.join(",").trim_end())
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum PropertyError {
    Unauthorized,
    NotFound,
    InvalidRequest,
    AuthorizedPrincipalLimitReached(u128),
    Immutable,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Array {
    frozen(Vec<CandyValue>),
    thawed(Vec<CandyValue>),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Nats {
    frozen(Vec<u128>),
    thawed(Vec<u128>),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Floats {
    frozen(Vec<f64>),
    thawed(Vec<f64>),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Bytes {
    frozen(Vec<u8>),
    thawed(Vec<u8>),
}

impl From<Vec<u8>> for Bytes {
    fn from(bytes: Vec<u8>) -> Self {
        Bytes::frozen(bytes)
    }
}
