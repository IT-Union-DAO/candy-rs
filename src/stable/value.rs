use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::stable::types::{Bytes, Floats, Nats, Property};
use crate::stable::types::Array;
use crate::unstable::types::PropertyUnstable;
use crate::unstable::value::CandyValueUnstable;

#[derive(CandidType, Debug, Serialize, Deserialize, Clone)]
pub enum CandyValue {
    Int(i128),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Nat(u128),
    Nat8(u8),
    Nat16(u16),
    Nat32(u32),
    Nat64(u64),
    Float(f64),
    Text(String),
    Bool(bool),
    Blob(Box<[u8]>),
    Class(Box<[Property]>),
    Principal(Principal),
    Option(Option<Box<CandyValue>>),
    Array(Array),
    Nats(Nats),
    Floats(Floats),
    Bytes(Bytes),
    Empty,
}


type UnstableFloats = crate::unstable::types::Floats;
type UnstableNats = crate::unstable::types::Nats;
type UnstableArray = crate::unstable::types::Array;

impl CandyValue {
    pub fn destabilize_value(self) -> CandyValueUnstable {
        match self {
            Self::Int(val) => CandyValueUnstable::Int(val),
            Self::Int8(val) => CandyValueUnstable::Int8(val),
            Self::Int16(val) => CandyValueUnstable::Int16(val),
            Self::Int32(val) => CandyValueUnstable::Int32(val),
            Self::Int64(val) => CandyValueUnstable::Int64(val),
            Self::Nat(val) => CandyValueUnstable::Nat(val),
            Self::Nat8(val) => CandyValueUnstable::Nat8(val),
            Self::Nat16(val) => CandyValueUnstable::Nat16(val),
            Self::Nat32(val) => CandyValueUnstable::Nat32(val),
            Self::Nat64(val) => CandyValueUnstable::Nat64(val),
            Self::Float(val) => CandyValueUnstable::Float(val),
            Self::Text(val) => CandyValueUnstable::Text(val),
            Self::Bool(val) => CandyValueUnstable::Bool(val),
            Self::Blob(val) => CandyValueUnstable::Blob(val.into_vec()),
            Self::Class(val) => CandyValueUnstable::Class(
                val.iter()
                    .cloned()
                    .map(|prop| prop.destabilize_property())
                    .collect::<Box<[PropertyUnstable]>>()
            ),
            Self::Principal(val) => CandyValueUnstable::Principal(val),
            Self::Floats(val) => CandyValueUnstable::Floats(
                match val {
                    Floats::Frozen(val) => UnstableFloats::Frozen(
                        val
                    ),
                    Floats::Thawed(val) => UnstableFloats::Thawed(
                        val.into_vec()
                    )
                }
            ),
            Self::Nats(val) => CandyValueUnstable::Nats(
                match val {
                    Nats::Frozen(val) => UnstableNats::Frozen(val),
                    Nats::Thawed(val) => UnstableNats::Thawed(val.into_vec())
                }
            ),
            Self::Array(val) => CandyValueUnstable::Array(
                match val {
                    Array::Frozen(val) => UnstableArray::Frozen(Self::destabilize_value_array(val)),
                    Array::Thawed(val) => UnstableArray::Thawed(
                        Self::destabilize_value_array(val).into_vec()
                    )
                }
            ),
            Self::Option(val) => CandyValueUnstable::Option(
                match val {
                    Some(val) => Some(Box::from(val.destabilize_value())),
                    None => None
                }
            ),
            Self::Bytes(val) => CandyValueUnstable::Bytes(
                match val {
                    Bytes::Frozen(val) => crate::unstable::types::Bytes::Frozen(val),
                    Bytes::Thawed(val) => crate::unstable::types::Bytes::Thawed(val.into_vec())
                }
            ),
            Self::Empty => CandyValueUnstable::Empty
        }
    }

    pub fn destabilize_value_array(items: Box<[CandyValue]>) -> Box<[CandyValueUnstable]> {
        items.iter()
            .cloned()
            .map(|item| item.destabilize_value())
            .collect()
    }
}

impl From<i128> for CandyValue {
    fn from(value: i128) -> Self {
        CandyValue::Int(value)
    }
}

impl From<i8> for CandyValue {
    fn from(value: i8) -> Self {
        CandyValue::Int8(value)
    }
}


impl From<i16> for CandyValue {
    fn from(value: i16) -> Self {
        CandyValue::Int16(value)
    }
}

impl From<i32> for CandyValue {
    fn from(value: i32) -> Self {
        CandyValue::Int32(value)
    }
}

impl From<i64> for CandyValue {
    fn from(value: i64) -> Self {
        CandyValue::Int64(value)
    }
}

impl From<u128> for CandyValue {
    fn from(value: u128) -> Self {
        CandyValue::Nat(value)
    }
}

impl From<u8> for CandyValue {
    fn from(value: u8) -> Self {
        CandyValue::Nat8(value)
    }
}

impl From<u16> for CandyValue {
    fn from(value: u16) -> Self {
        CandyValue::Nat16(value)
    }
}

impl From<u32> for CandyValue {
    fn from(value: u32) -> Self {
        CandyValue::Nat32(value)
    }
}

impl From<u64> for CandyValue {
    fn from(value: u64) -> Self {
        CandyValue::Nat64(value)
    }
}

impl From<f64> for CandyValue {
    fn from(value: f64) -> Self {
        CandyValue::Float(value)
    }
}

impl From<String> for CandyValue {
    fn from(value: String) -> Self {
        CandyValue::Text(value)
    }
}

impl From<bool> for CandyValue {
    fn from(value: bool) -> Self {
        CandyValue::Bool(value)
    }
}

impl From<Box<[u8]>> for CandyValue {
    fn from(value: Box<[u8]>) -> Self {
        CandyValue::Blob(value)
    }
}

impl From<Box<[Property]>> for CandyValue {
    fn from(value: Box<[Property]>) -> Self {
        CandyValue::Class(value)
    }
}

impl From<Principal> for CandyValue {
    fn from(value: Principal) -> Self {
        CandyValue::Principal(value)
    }
}

impl From<Option<Box<CandyValue>>> for CandyValue {
    fn from(value: Option<Box<CandyValue>>) -> Self {
        CandyValue::Option(value)
    }
}

impl From<Array> for CandyValue {
    fn from(value: Array) -> Self {
        CandyValue::Array(value)
    }
}

impl From<Nats> for CandyValue {
    fn from(value: Nats) -> Self {
        CandyValue::Nats(value)
    }
}


impl From<Floats> for CandyValue {
    fn from(value: Floats) -> Self {
        CandyValue::Floats(value)
    }
}

impl From<Bytes> for CandyValue {
    fn from(value: Bytes) -> Self {
        CandyValue::Bytes(value)
    }
}





