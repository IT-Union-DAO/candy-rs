use std::hash::Hash;

use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::value::CandyShared;

pub type Properties = Vec<PropertyShared>;

#[derive(Clone, Debug, PartialEq, Eq, CandidType, Serialize, Deserialize)]
pub struct PropertyShared {
    pub name: String,
    pub value: CandyShared,
    pub immutable: bool,
}

impl ToString for PropertyShared {
    fn to_string(&self) -> String {
        let desc = match self.immutable {
            true => "var ".to_string(),
            false => "".to_string(),
        };
        format!("{}:{}{}; ", self.name, desc, self.value.clone().to_string())
    }
}

impl PropertyShared {
    pub fn stringify_properties(props: &[PropertyShared]) -> String {
        let prop_strings: Vec<String> = props.iter().map(|p| p.to_string()).collect();
        format!("{{{}}}", prop_strings.join("").trim_end())
    }

    pub fn to_json(self) -> String {
        format!("\"{}\":{}", self.name, self.value.to_json())
    }

    pub fn props_to_json(props: &[PropertyShared]) -> String {
        let prop_strings: Vec<String> = props.iter().map(|p| p.clone().to_json()).collect();
        format!("{{{}}}", prop_strings.join(",").trim_end())
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum PropertyError {
    Unauthorized,
    NotFound,
    InvalidRequest,
    AuthorizedPrincipalLimitReached(candid::Nat),
    Immutable,
}

#[derive(Clone, Debug, std::cmp::PartialOrd, CandidType, Serialize, Deserialize)]
pub struct Float(f64);

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < f64::EPSILON
    }
}

impl Eq for Float {}

impl Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}
