use candid::Principal;
use hex::ToHex;
use num_traits::cast::ToPrimitive;
use std::fmt::format;

use crate::types::types::{Array, Bytes, Floats, Nats, Property};
use crate::types::value::CandyValue;
use crate::value::ToCandyValue;

pub trait UnboxCandyValue {
    fn to_nat(self) -> Option<u128>;
    fn to_nat8(self) -> Option<u8>;
    fn to_nat16(self) -> Option<u16>;
    fn to_nat32(self) -> Option<u32>;
    fn to_nat64(self) -> Option<u64>;
    fn to_int(self) -> Option<i128>;
    fn to_int8(self) -> Option<i8>;
    fn to_int16(self) -> Option<i16>;
    fn to_int32(self) -> Option<i32>;
    fn to_int64(self) -> Option<i64>;
    fn to_float(self) -> Option<f64>;
    fn to_bool(self) -> Option<bool>;
    fn to_principal(self) -> Option<Principal>;
    fn to_blob(self) -> Result<Vec<u8>, String>;
    fn to_value_array(self) -> Option<Vec<CandyValue>>;
    fn to_json(self) -> String;
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

impl UnboxCandyValue for CandyValue {
    fn to_nat(self) -> Option<u128> {
        to_nat_of_size!(self, to_u128)
    }

    fn to_nat8(self) -> Option<u8> {
        to_nat_of_size!(self, to_u8)
    }

    fn to_nat16(self) -> Option<u16> {
        to_nat_of_size!(self, to_u16)
    }

    fn to_nat32(self) -> Option<u32> {
        to_nat_of_size!(self, to_u32)
    }

    fn to_nat64(self) -> Option<u64> {
        to_nat_of_size!(self, to_u64)
    }

    fn to_int(self) -> Option<i128> {
        to_int_of_size!(self, to_i128)
    }

    fn to_int8(self) -> Option<i8> {
        to_int_of_size!(self, to_i8)
    }

    fn to_int16(self) -> Option<i16> {
        to_int_of_size!(self, to_i16)
    }

    fn to_int32(self) -> Option<i32> {
        to_int_of_size!(self, to_i32)
    }

    fn to_int64(self) -> Option<i64> {
        to_int_of_size!(self, to_i64)
    }

    fn to_float(self) -> Option<f64> {
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

    fn to_bool(self) -> Option<bool> {
        match self {
            Self::Bool(val) => Some(val),
            _ => None,
        }
    }

    fn to_principal(self) -> Option<Principal> {
        match self {
            Self::Principal(val) => Some(val),
            _ => None,
        }
    }

    fn to_blob(self) -> Result<Vec<u8>, String> {
        match self {
            Self::Blob(val) => Ok(val),
            Self::Bytes(val) => match val {
                Bytes::thawed(val) => Ok(val),
                Bytes::frozen(val) => Ok(val),
            },
            Self::Text(val) => Ok(val.as_bytes().into()),
            Self::Float(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Int(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Int8(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Int16(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Int32(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Int64(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Bool(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Nat(val) => serde_cbor::to_vec(&val)
                .map_err(|e| e.to_string())
                .map(|v| v.into()),
            Self::Nat8(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Nat16(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Nat32(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Nat64(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Class(val) => serde_cbor::to_vec(&val).map_err(|e| e.to_string()),
            Self::Principal(val) => Ok(val.as_slice().into()),
            _ => Err("Can not be converted to blob".to_string()),
        }
    }

    fn to_json(self) -> String {
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

    fn to_value_array(self) -> Option<Vec<CandyValue>> {
        match self {
            Self::Array(val) => match val {
                Array::thawed(val) => Some(val),
                Array::frozen(val) => Some(val),
            },
            _ => None,
        }
    }
}
