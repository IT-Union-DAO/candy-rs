use std::fmt::Display;

use candid::{CandidType, Principal};
use hex::ToHex;
use serde::{Deserialize, Serialize};

use crate::types::types::{Array, Floats, Nats, Property};

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
    Blob(Vec<u8>),
    Class(Vec<Property>),
    Principal(Principal),
    Option(Option<Box<CandyValue>>),
    Array(Array),
    Nats(Nats),
    Floats(Floats),
    Empty,
}

impl Display for CandyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(val) => write!(f, "{}", val),
            Self::Int8(val) => write!(f, "{}", val),
            Self::Int16(val) => write!(f, "{}", val),
            Self::Int32(val) => write!(f, "{}", val),
            Self::Int64(val) => write!(f, "{}", val),
            Self::Nat(val) => write!(f, "{}", val),
            Self::Nat8(val) => write!(f, "{}", val),
            Self::Nat16(val) => write!(f, "{}", val),
            Self::Nat32(val) => write!(f, "{}", val),
            Self::Nat64(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::Text(val) => write!(f, "{}", val),
            Self::Bool(val) => write!(f, "{}", val),
            Self::Blob(val) => write!(f, "{}", val.encode_hex::<String>()),
            Self::Class(val) => write!(f, "{}", Property::stringify_properties(val)),
            Self::Principal(val) => write!(f, "{}", val.to_string()),
            Self::Option(val) => write!(
                f,
                "{}",
                val.as_ref()
                    .map(|val| val.to_string())
                    .unwrap_or("null".to_string())
            ),
            Self::Array(val) => write!(f, "{}", {
                let vec = match val {
                    Array::thawed(val) => val,
                    Array::frozen(val) => val,
                };
                let mut ret = String::new();
                ret.push('[');
                ret.push_str(
                    vec.iter()
                        .map(|val| format!("{{{}}}", val.to_string()))
                        .collect::<Vec<String>>()
                        .join(" ")
                        .as_str(),
                );
                let _ = ret.trim_end();
                ret.push(']');

                ret
            }),
            Self::Nats(val) => write!(f, "{}", {
                let vec = match val {
                    Nats::thawed(val) => val,
                    Nats::frozen(val) => val,
                };
                let mut ret = String::new();
                ret.push('[');
                ret.push_str(
                    vec.iter()
                        .map(|val| val.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                        .as_str(),
                );
                let _ = ret.trim_end();
                ret.push(']');
                ret
            }),
            Self::Floats(val) => write!(f, "{}", {
                let vec = match val {
                    Floats::thawed(val) => val,
                    Floats::frozen(val) => val,
                };
                let mut ret = String::new();
                ret.push('[');
                ret.push_str(
                    vec.iter()
                        .map(|val| val.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                        .as_str(),
                );
                let _ = ret.trim_end();
                ret.push(']');

                ret
            }),
            Self::Empty => write!(f, ""),
        }
    }
}

impl CandyValue {
    pub fn stringify_array_of_values(vals: &[CandyValue]) -> String {
        let mut result = String::new();
        result.push('[');
        for value in vals {
            let converted = format!("{{{}}} ", value.clone().to_string());
            result.push_str(&converted);
        }
        let mut trimmed = result.trim_end().to_string();
        trimmed.push(']');
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
    i64 => Int64,
    u128 => Nat,
    u8 => Nat8,
    u16 => Nat16,
    u32 => Nat32,
    u64 => Nat64,
    f64 => Float
);

impl_from!(
    String => Text,
    bool => Bool,
    Vec<Property> => Class,
    Principal => Principal ,
    Option<Box<CandyValue>> => Option,
    Vec<u8> => Blob
);

impl From<&str> for CandyValue {
    fn from(value: &str) -> Self {
        CandyValue::Text(value.to_string())
    }
}

impl From<Vec<CandyValue>> for CandyValue {
    fn from(value: Vec<CandyValue>) -> Self {
        CandyValue::Array(Array::frozen(value))
    }
}

impl From<Vec<u128>> for CandyValue {
    fn from(value: Vec<u128>) -> Self {
        CandyValue::Nats(Nats::frozen(value))
    }
}

impl From<Vec<f64>> for CandyValue {
    fn from(value: Vec<f64>) -> Self {
        CandyValue::Floats(Floats::frozen(value))
    }
}

pub trait ToCandyValue {
    fn to_candy(self) -> CandyValue;
}

macro_rules! to_candy {
    ($($t:ty),*) => {
        $(impl ToCandyValue for $t {
            #[inline]
            fn to_candy(self) -> CandyValue {
                CandyValue::from(self)
            }
        })*
    };
}

to_candy!(
    i128,
    i8,
    i16,
    i32,
    i64,
    u128,
    u8,
    u16,
    u32,
    u64,
    f64,
    Vec<u128>,
    Vec<f64>,
    String,
    bool,
    Vec<Property>,
    Principal,
    Option<Box<CandyValue>>,
    Vec<u8>,
    Vec<CandyValue>
);
