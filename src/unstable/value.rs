use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::stable::types::Property;
use crate::stable::value::CandyValue;
use crate::unstable::types::{Array, Bytes, Floats, Nats};
use crate::unstable::types::PropertyUnstable;

#[derive(CandidType, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CandyValueUnstable {
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
    Blob(Vec<u8>),
    Class(Box<[PropertyUnstable]>),
    Principal(Principal),
    Floats(Floats),
    Nats(Nats),
    Array(Array),
    Option(Option<Box<CandyValueUnstable>>),
    Bytes(Bytes),
    Empty,
}

type StableNats = crate::stable::types::Nats;
type StableFloats = crate::stable::types::Floats;
type StableArray = crate::stable::types::Array;

impl CandyValueUnstable {
    pub fn stabilize_value(self) -> CandyValue {
        match self {
            Self::Int(val) => CandyValue::Int(val),
            Self::Int8(val) => CandyValue::Int8(val),
            Self::Int16(val) => CandyValue::Int16(val),
            Self::Int32(val) => CandyValue::Int32(val),
            Self::Int64(val) => CandyValue::Int64(val),
            Self::Nat(val) => CandyValue::Nat(val),
            Self::Nat8(val) => CandyValue::Nat8(val),
            Self::Nat16(val) => CandyValue::Nat16(val),
            Self::Nat32(val) => CandyValue::Nat32(val),
            Self::Nat64(val) => CandyValue::Nat64(val),
            Self::Float(val) => CandyValue::Float(val),
            Self::Text(val) => CandyValue::Text(val),
            Self::Bool(val) => CandyValue::Bool(val),
            Self::Blob(val) => CandyValue::Blob(val.into_boxed_slice()),
            Self::Class(val) => CandyValue::Class(val.iter()
                .cloned()
                .map(|prop| prop.stabilize_property()).collect::<Box<[Property]>>()),
            Self::Principal(val) => CandyValue::Principal(val),
            Self::Floats(val) => CandyValue::Floats(
                match val {
                    Floats::Frozen(val) => StableFloats::Frozen(val),
                    Floats::Thawed(val) => StableFloats::Frozen(val.into_boxed_slice())
                }),
            Self::Nats(val) => CandyValue::Nats(
                match val {
                    Nats::Frozen(val) => StableNats::Frozen(val),
                    Nats::Thawed(val) => StableNats::Thawed(val.into_boxed_slice())
                }
            ),
            Self::Array(val) => CandyValue::Array(
                match val {
                    Array::Frozen(val) => StableArray::Frozen(
                        Self::stabilize_value_array(val)
                    ),
                    Array::Thawed(val) => StableArray::Thawed(
                        Self::stabilize_value_array(val.into_boxed_slice())
                    )
                }
            ),
            Self::Option(val) => CandyValue::Option(
                match val {
                    Some(val) => Some(Box::from(val.stabilize_value())),
                    None => None
                }
            ),
            Self::Bytes(val) => CandyValue::Bytes(
                match val {
                    Bytes::Frozen(val) => crate::stable::types::Bytes::Frozen(val),
                    Bytes::Thawed(val) => crate::stable::types::Bytes::Thawed(val.into_boxed_slice())
                }
            ),
            Self::Empty => CandyValue::Empty
        }
    }

    pub fn stabilize_value_array(items: Box<[CandyValueUnstable]>) -> Box<[CandyValue]> {
        items.iter()
            .cloned()
            .map(|val| val.stabilize_value())
            .collect::<Vec<CandyValue>>()
            .into_boxed_slice()
    }
}


impl From<i128> for CandyValueUnstable {
    fn from(value: i128) -> Self {
        CandyValueUnstable::Int(value)
    }
}

impl From<i8> for CandyValueUnstable {
    fn from(val: i8) -> Self {
        CandyValueUnstable::Int8(val)
    }
}

impl From<i16> for CandyValueUnstable {
    fn from(val: i16) -> Self {
        CandyValueUnstable::Int16(val)
    }
}

impl From<i32> for CandyValueUnstable {
    fn from(val: i32) -> Self {
        CandyValueUnstable::Int32(val)
    }
}

impl From<i64> for CandyValueUnstable {
    fn from(val: i64) -> Self {
        CandyValueUnstable::Int64(val)
    }
}

impl From<u128> for CandyValueUnstable {
    fn from(val: u128) -> Self {
        CandyValueUnstable::Nat(val)
    }
}

impl From<u8> for CandyValueUnstable {
    fn from(val: u8) -> Self {
        CandyValueUnstable::Nat8(val)
    }
}

impl From<u16> for CandyValueUnstable {
    fn from(val: u16) -> Self {
        CandyValueUnstable::Nat16(val)
    }
}

impl From<u32> for CandyValueUnstable {
    fn from(val: u32) -> Self {
        CandyValueUnstable::Nat32(val)
    }
}

impl From<u64> for CandyValueUnstable {
    fn from(val: u64) -> Self {
        CandyValueUnstable::Nat64(val)
    }
}

impl From<f64> for CandyValueUnstable {
    fn from(val: f64) -> Self {
        CandyValueUnstable::Float(val)
    }
}

impl From<String> for CandyValueUnstable {
    fn from(val: String) -> Self {
        CandyValueUnstable::Text(val)
    }
}

impl From<bool> for CandyValueUnstable {
    fn from(val: bool) -> Self {
        CandyValueUnstable::Bool(val)
    }
}

impl From<Vec<u8>> for CandyValueUnstable {
    fn from(value: Vec<u8>) -> Self {
        CandyValueUnstable::Blob(value)
    }
}

impl From<Box<[PropertyUnstable]>> for CandyValueUnstable {
    fn from(value: Box<[PropertyUnstable]>) -> Self {
        CandyValueUnstable::Class(value)
    }
}

impl From<Principal> for CandyValueUnstable {
    fn from(value: Principal) -> Self {
        CandyValueUnstable::Principal(value)
    }
}

impl From<Floats> for CandyValueUnstable {
    fn from(value: Floats) -> Self {
        CandyValueUnstable::Floats(value)
    }
}

impl From<Nats> for CandyValueUnstable {
    fn from(value: Nats) -> Self {
        CandyValueUnstable::Nats(value)
    }
}

impl From<Array> for CandyValueUnstable {
    fn from(value: Array) -> Self {
        CandyValueUnstable::Array(value)
    }
}

impl From<Option<Box<CandyValueUnstable>>> for CandyValueUnstable {
    fn from(value: Option<Box<CandyValueUnstable>>) -> Self {
        CandyValueUnstable::Option(value)
    }
}

impl From<Bytes> for CandyValueUnstable {
    fn from(value: Bytes) -> Self {
        CandyValueUnstable::Bytes(value)
    }
}


