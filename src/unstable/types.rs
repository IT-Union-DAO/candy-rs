use candid::{CandidType, Nat};
use serde::{Deserialize, Serialize};

use crate::stable::types::Property;
use crate::unstable::value::CandyValueUnstable;

pub type PropertiesUnstable = Vec<PropertyUnstable>;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub struct PropertyUnstable {
    pub name: String,
    pub value: CandyValueUnstable,
    pub immutable: bool,
}

impl PropertyUnstable {
    pub fn stabilize_property(self) -> Property {
        Property {
            name: self.name,
            value: self.value.stabilize_value(),
            immutable: self.immutable,
        }
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum Floats {
    Frozen(Box<[f64]>),
    Thawed(Vec<f64>),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize,  PartialEq)]
pub enum Nats {
    Frozen(Box<[Nat]>),
    Thawed(Vec<Nat>),
}


#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum Array {
    Frozen(Box<[CandyValueUnstable]>),
    Thawed(Vec<CandyValueUnstable>),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, PartialEq)]
pub enum Bytes {
    Frozen(Box<[u8]>),
    Thawed(Vec<u8>),
}
