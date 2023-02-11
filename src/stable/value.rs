use candid::{CandidType, Principal};
use hex::ToHex;
use serde::{Deserialize, Serialize};

use crate::stable::types::Array;
use crate::stable::types::{Bytes, Floats, Nats, Property};
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
                    .collect::<Box<[PropertyUnstable]>>(),
            ),
            Self::Principal(val) => CandyValueUnstable::Principal(val),
            Self::Floats(val) => CandyValueUnstable::Floats(match val {
                Floats::Frozen(val) => UnstableFloats::Frozen(val),
                Floats::Thawed(val) => UnstableFloats::Thawed(val.into_vec()),
            }),
            Self::Nats(val) => CandyValueUnstable::Nats(match val {
                Nats::Frozen(val) => UnstableNats::Frozen(val),
                Nats::Thawed(val) => UnstableNats::Thawed(val.into_vec()),
            }),
            Self::Array(val) => CandyValueUnstable::Array(match val {
                Array::Frozen(val) => UnstableArray::Frozen(Self::destabilize_value_array(val)),
                Array::Thawed(val) => {
                    UnstableArray::Thawed(Self::destabilize_value_array(val).into_vec())
                }
            }),
            Self::Option(val) => CandyValueUnstable::Option(match val {
                Some(val) => Some(Box::from(val.destabilize_value())),
                None => None,
            }),
            Self::Bytes(val) => CandyValueUnstable::Bytes(match val {
                Bytes::Frozen(val) => crate::unstable::types::Bytes::Frozen(val),
                Bytes::Thawed(val) => crate::unstable::types::Bytes::Thawed(val.into_vec()),
            }),
            Self::Empty => CandyValueUnstable::Empty,
        }
    }

    pub fn destabilize_value_array(items: Box<[CandyValue]>) -> Box<[CandyValueUnstable]> {
        items
            .iter()
            .cloned()
            .map(|item| item.destabilize_value())
            .collect()
    }

    pub fn to_string(self) -> String {
        match self {
            Self::Int(val) => val.to_string(),
            Self::Int8(val) => val.to_string(),
            Self::Int16(val) => val.to_string(),
            Self::Int32(val) => val.to_string(),
            Self::Int64(val) => val.to_string(),
            Self::Nat(val) => val.to_string(),
            Self::Nat8(val) => val.to_string(),
            Self::Nat16(val) => val.to_string(),
            Self::Nat32(val) => val.to_string(),
            Self::Nat64(val) => val.to_string(),
            Self::Float(val) => val.to_string(),
            Self::Text(val) => val,
            Self::Bool(val) => val.to_string(),
            Self::Option(val) => val.map(|val| val.to_string()).unwrap_or("null".to_string()),
            Self::Blob(val) => val.encode_hex::<String>(),
            Self::Class(val) => Property::stringify_properties(val.as_ref()),
            Self::Principal(val) => val.to_string(),
            Self::Array(val) => val.to_string(),
            Self::Floats(val) => val.to_string(),
            Self::Bytes(val) => val.to_string(),
            _ => panic!("Type can not be converted to String!"),
        }
    }

    pub fn stringify_array_of_values(vals: &[CandyValue]) -> String {
        let mut result = String::new();
        result.push_str("[");
        for value in vals {
            let converted = format!("{{{}}} ", value.clone().to_string());
            result.push_str(&converted);
        }
        let mut trimmed = result.trim_end().to_string();
        trimmed.push_str("]");
        trimmed
    }
}

macro_rules! impl_from {
    ($($t:ty => $v:ident),*) => {
        $(impl From<$t> for CandyValue {
            fn from(value: $t) -> Self {
                CandyValue::$v(value)
            }
        })*
    };
}

impl_from!(
    i128 => Int,
    i8 => Int8,
    i16 => Int16,
    i32 => Int32,
    i64 => Int64
);
impl_from!(
    u128 => Nat,
    u8 => Nat8,
    u16 => Nat16,
    u32 => Nat32,
    u64 => Nat64,
    f64 => Float
);

impl From<&str> for CandyValue {
    fn from(value: &str) -> Self {
        CandyValue::Text(value.to_string())
    }
}
impl_from!(
    String => Text,
    bool => Bool
);
impl_from!(
    Box<[u8]> => Blob,
    Box<[Property]> => Class
);
impl_from!(
    Principal => Principal ,
    Option<Box<CandyValue>> => Option ,
    Array => Array
);
impl_from!(
    Nats => Nats ,
    Floats => Floats,
    Bytes => Bytes
);
