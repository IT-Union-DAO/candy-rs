use std::fmt::Display;
use std::ops::Add;

use crate::types::{Array, Bytes, Floats, Nats, Property};
use candid::{CandidType, Principal};
use hex::ToHex;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};

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
    Bytes(Bytes),
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
            Self::Bytes(val) => {
                let value = match val {
                    Bytes::thawed(val) => val,
                    Bytes::frozen(val) => val,
                };
                write!(f, "{}", value.encode_hex::<String>())
            }
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

macro_rules! to_nat_of_size {
    ($x:tt, $method: ident) => {
        match $x {
            Self::Nat(val) => val.$method(),
            Self::Nat8(val) => val.$method(),
            Self::Nat16(val) => val.$method(),
            Self::Nat32(val) => val.$method(),
            Self::Nat64(val) => val.$method(),
            Self::Float(val) => match val {
                val if val < 0.0 => None,
                _ => val.round().$method(),
            },
            Self::Int(val) => val.$method(),
            Self::Int8(val) => val.$method(),
            Self::Int16(val) => val.$method(),
            Self::Int32(val) => val.$method(),
            Self::Int64(val) => val.$method(),
            _ => None,
        }
    };
}

macro_rules! to_int_of_size {
    ($x:tt, $method: ident) => {
        match $x {
            Self::Nat(val) => val.$method(),
            Self::Nat8(val) => val.$method(),
            Self::Nat16(val) => val.$method(),
            Self::Nat32(val) => val.$method(),
            Self::Nat64(val) => val.$method(),
            Self::Float(val) => val.round().$method(),
            Self::Int(val) => val.$method(),
            Self::Int8(val) => val.$method(),
            Self::Int16(val) => val.$method(),
            Self::Int32(val) => val.$method(),
            Self::Int64(val) => val.$method(),
            _ => None,
        }
    };
}

impl CandyValue {
    pub fn to_nat(self) -> Option<u128> {
        to_nat_of_size!(self, to_u128)
    }

    pub fn to_nat8(self) -> Option<u8> {
        to_nat_of_size!(self, to_u8)
    }

    pub fn to_nat16(self) -> Option<u16> {
        to_nat_of_size!(self, to_u16)
    }

    pub fn to_nat32(self) -> Option<u32> {
        to_nat_of_size!(self, to_u32)
    }

    pub fn to_nat64(self) -> Option<u64> {
        to_nat_of_size!(self, to_u64)
    }

    pub fn to_int(self) -> Option<i128> {
        to_int_of_size!(self, to_i128)
    }

    pub fn to_int8(self) -> Option<i8> {
        to_int_of_size!(self, to_i8)
    }

    pub fn to_int16(self) -> Option<i16> {
        to_int_of_size!(self, to_i16)
    }

    pub fn to_int32(self) -> Option<i32> {
        to_int_of_size!(self, to_i32)
    }

    pub fn to_int64(self) -> Option<i64> {
        to_int_of_size!(self, to_i64)
    }

    pub fn to_float(self) -> Option<f64> {
        match self {
            Self::Nat(val) => val.to_f64(),
            Self::Nat8(val) => val.to_f64(),
            Self::Nat16(val) => val.to_f64(),
            Self::Nat32(val) => val.to_f64(),
            Self::Nat64(val) => val.to_f64(),
            Self::Float(val) => Some(val),
            Self::Int(val) => val.to_f64(),
            Self::Int8(val) => val.to_f64(),
            Self::Int16(val) => val.to_f64(),
            Self::Int32(val) => val.to_f64(),
            Self::Int64(val) => val.to_f64(),
            _ => None,
        }
    }

    pub fn to_bool(self) -> Option<bool> {
        match self {
            Self::Bool(val) => Some(val),
            _ => None,
        }
    }

    pub fn to_principal(self) -> Option<Principal> {
        match self {
            Self::Principal(val) => Some(val),
            _ => None,
        }
    }

    pub fn to_blob(self) -> Vec<u8> {
        let value = match self {
            Self::Blob(val) => Ok(val),
            Self::Bytes(val) => match val {
                Bytes::thawed(val) => Ok(val),
                Bytes::frozen(val) => Ok(val),
            },
            Self::Text(val) => Ok(val.as_bytes().into()),
            Self::Float(val) => serde_cbor::to_vec(&val),
            Self::Int(val) => serde_cbor::to_vec(&val),
            Self::Int8(val) => serde_cbor::to_vec(&val),
            Self::Int16(val) => serde_cbor::to_vec(&val),
            Self::Int32(val) => serde_cbor::to_vec(&val),
            Self::Int64(val) => serde_cbor::to_vec(&val),
            Self::Bool(val) => serde_cbor::to_vec(&val),
            Self::Nat(val) => serde_cbor::to_vec(&val),
            Self::Nat8(val) => serde_cbor::to_vec(&val),
            Self::Nat16(val) => serde_cbor::to_vec(&val),
            Self::Nat32(val) => serde_cbor::to_vec(&val),
            Self::Nat64(val) => serde_cbor::to_vec(&val),
            Self::Class(val) => serde_cbor::to_vec(&val),
            Self::Principal(val) => Ok(val.as_slice().into()),
            Self::Array(val) => serde_cbor::to_vec(&val),
            Self::Option(val) => serde_cbor::to_vec(&val),
            Self::Nats(val) => serde_cbor::to_vec(&val),
            Self::Floats(val) => serde_cbor::to_vec(&val),
            _ => ic_cdk::trap("Empty type cannot be serialized"),
        };
        match value {
            Ok(val) => val,
            Err(e) => ic_cdk::trap(&e.to_string()),
        }
    }

    pub fn to_json(self) -> String {
        match self {
            Self::Nat(val) => val.to_string(),
            Self::Nat8(val) => val.to_string(),
            Self::Nat16(val) => val.to_string(),
            Self::Nat32(val) => val.to_string(),
            Self::Nat64(val) => val.to_string(),
            Self::Int(val) => val.to_string(),
            Self::Int8(val) => val.to_string(),
            Self::Int16(val) => val.to_string(),
            Self::Int32(val) => val.to_string(),
            Self::Int64(val) => val.to_string(),
            Self::Float(val) => val.to_string(),
            Self::Text(val) => serde_json::to_string(&val).unwrap(),
            Self::Class(val) => Property::props_to_json(&val),
            Self::Array(val) => {
                let val = match val {
                    Array::thawed(val) => val,
                    Array::frozen(val) => val,
                };
                format!(
                    "[{}]",
                    val.iter()
                        .map(|i| i.clone().to_json())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Self::Option(val) => match val {
                Some(val) => val.to_json(),
                None => "null".to_string(),
            },
            Self::Nats(val) => {
                let val = match val {
                    Nats::thawed(val) => val,
                    Nats::frozen(val) => val,
                };
                format!(
                    "[{}]",
                    val.iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Self::Floats(val) => {
                let val = match val {
                    Floats::thawed(val) => val,
                    Floats::frozen(val) => val,
                };
                format!(
                    "[{}]",
                    val.iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Self::Bytes(val) => format!("\"{}\"", val.to_candy().to_string()),
            Self::Blob(val) => format!("\"{}\"", val.encode_hex::<String>()),
            Self::Principal(val) => format!("\"{}\"", val.to_string()),
            Self::Bool(val) => format!("\"{}\"", val),
            Self::Empty => "null".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn to_value_array(self) -> Option<Vec<CandyValue>> {
        match self {
            Self::Array(val) => match val {
                Array::thawed(val) => Some(val),
                Array::frozen(val) => Some(val),
            },
            _ => None,
        }
    }

    //TODO: verify implementation is appropriate
    pub fn get_value_size(&self) -> u128 {
        match self {
            Self::Nat(val) => std::mem::size_of_val(val) as u128,
            Self::Nat8(val) => std::mem::size_of_val(val) as u128,
            Self::Nat16(val) => std::mem::size_of_val(val) as u128,
            Self::Nat32(val) => std::mem::size_of_val(val) as u128,
            Self::Nat64(val) => std::mem::size_of_val(val) as u128,
            Self::Int(val) => std::mem::size_of_val(val) as u128,
            Self::Int8(val) => std::mem::size_of_val(val) as u128,
            Self::Int16(val) => std::mem::size_of_val(val) as u128,
            Self::Int32(val) => std::mem::size_of_val(val) as u128,
            Self::Int64(val) => std::mem::size_of_val(val) as u128,
            Self::Float(val) => std::mem::size_of_val(val) as u128,
            Self::Text(val) => std::mem::size_of_val(val) as u128,
            Self::Class(val) => std::mem::size_of_val(val) as u128,
            Self::Array(val) => match val {
                Array::thawed(val) => val
                    .iter()
                    .map(|i| i.get_value_size())
                    .fold(0, |acc, x| acc.add(x)),
                Array::frozen(val) => val
                    .iter()
                    .map(|i| i.get_value_size())
                    .fold(0, |acc, x| acc.add(x)),
            },
            Self::Option(val) => match val {
                Some(val) => val.get_value_size(),
                None => 0,
            },
            Self::Nats(val) => match val {
                Nats::thawed(val) => std::mem::size_of_val(val) as u128,
                Nats::frozen(val) => std::mem::size_of_val(val) as u128,
            },
            Self::Floats(val) => match val {
                Floats::thawed(val) => std::mem::size_of_val(val) as u128,
                Floats::frozen(val) => std::mem::size_of_val(val) as u128,
            },
            Self::Bytes(val) => match val {
                Bytes::thawed(val) => std::mem::size_of_val(val) as u128,
                Bytes::frozen(val) => std::mem::size_of_val(val) as u128,
            },
            Self::Blob(val) => std::mem::size_of_val(val) as u128,
            Self::Principal(val) => std::mem::size_of_val(val) as u128,
            Self::Bool(val) => std::mem::size_of_val(val) as u128,
            Self::Empty => 0,
        }
    }

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

impl From<Bytes> for CandyValue {
    fn from(value: Bytes) -> Self {
        Self::Bytes(value)
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
    Vec<CandyValue>,
    Bytes
);
