use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::stable::value::CandyValue;
use crate::unstable::types::PropertyUnstable;

pub type Properties = Box<[Property]>;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: CandyValue,
    pub immutable: bool,
}

impl Property {
    pub fn destabilize_property(self) -> PropertyUnstable {
        PropertyUnstable {
            name: self.name.to_string(),
            value: self.value.destabilize_value(),
            immutable: self.immutable,
        }
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum PropertyError {
    Unauthorized,
    NotFound,
    InvalidRequest,
    AuthorizedPrincipalLimitReached(Nat),
    Immutable,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Array {
    Frozen(Box<[CandyValue]>),
    Thawed(Box<[CandyValue]>),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Nats {
    Frozen(Box<[Nat]>),
    Thawed(Box<[Nat]>),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Floats {
    Frozen(Box<[f64]>),
    Thawed(Box<[f64]>),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Bytes {
    Frozen(Box<[u8]>),
    Thawed(Box<[u8]>),
}
