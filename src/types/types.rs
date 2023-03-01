use candid::CandidType;
use hex::ToHex;
use serde::{Deserialize, Serialize};

use crate::conversion::imp::UnboxCandyValue;
use crate::types::value::CandyValue;

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
        format!("{}:{}{}; ", self.name, desc, self.value.clone().to_text())
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
