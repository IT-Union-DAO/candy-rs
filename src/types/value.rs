use candid::{CandidType, Principal};
use hex::ToHex;
use serde::{Deserialize, Serialize};

use crate::types::types::Property;

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
    Array(Vec<CandyValue>),
    Nats(Vec<u128>),
    Floats(Vec<f64>),
    Empty,
}

trait ToText {
    fn to_text(self) -> String;
}
macro_rules! candy_nums_to_text {
    ($ lhs : ty) => {
        impl ToText for $lhs {
            fn to_text(self) -> String {
                let mut ret = String::new();
                ret.push_str("[");
                ret.push_str(
                    self.iter()
                        .map(|val| val.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                        .as_str(),
                );
                let _ = ret.trim_end();
                ret.push_str("]");

                ret
            }
        }
    };
}

candy_nums_to_text!(Vec<u128>);
candy_nums_to_text!(Vec<f64>);

impl CandyValue {
    pub fn to_text(self) -> String {
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
            Self::Nats(val) => val.to_text(),
            Self::Float(val) => val.to_string(),
            Self::Text(val) => val,
            Self::Bool(val) => val.to_string(),
            Self::Option(val) => val.map(|val| val.to_text()).unwrap_or("null".to_string()),
            Self::Blob(val) => val.encode_hex::<String>(),
            Self::Class(val) => Property::stringify_properties(val.as_ref()),
            Self::Principal(val) => val.to_string(),
            Self::Array(val) => {
                let mut ret = String::new();
                ret.push('[');
                for value in val {
                    let converted = format!("{{{}}} ", value.clone().to_text());
                    ret.push_str(&converted)
                }
                let mut trimmed = ret.trim_end().to_string();
                trimmed.push(']');
                trimmed
            }
            Self::Floats(val) => val.to_text(),
            _ => panic!("Type can not be converted to String!"),
        }
    }

    pub fn stringify_array_of_values(vals: &[CandyValue]) -> String {
        let mut result = String::new();
        result.push('[');
        for value in vals {
            let converted = format!("{{{}}} ", value.clone().to_text());
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
    f64 => Float,
    Vec<u128> => Nats,
    Vec<f64> => Floats
);

impl_from!(
    String => Text,
    bool => Bool,
    Vec<Property> => Class,
    Principal => Principal ,
    Option<Box<CandyValue>> => Option,
    Vec<u8> => Blob,
    Vec<CandyValue> => Array
);

impl From<&str> for CandyValue {
    fn from(value: &str) -> Self {
        CandyValue::Text(value.to_string())
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
