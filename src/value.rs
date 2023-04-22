use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

use candid::{CandidType, Deserialize, Encode, Principal};
use hex::ToHex;
use num_bigint::{BigInt, BigUint};
use num_traits::cast::ToPrimitive;
use num_traits::Signed;
use serde::Serialize;

use crate::types::PropertyShared;

#[derive(CandidType, Debug, Serialize, Deserialize, Clone)]
pub enum CandyShared {
    Int(candid::Int),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Ints(Vec<candid::Int>),
    Nat(candid::Nat),
    Nat8(u8),
    Nat16(u16),
    Nat32(u32),
    Nat64(u64),
    Float(f64),
    Text(String),
    Bool(bool),
    Blob(Vec<u8>),
    Bytes(Vec<u8>),
    Class(Vec<PropertyShared>),
    Principal(Principal),
    Option(Option<Box<CandyShared>>),
    Array(Vec<CandyShared>),
    Nats(Vec<candid::Nat>),
    Floats(Vec<f64>),
    Map(HashMap<CandyShared, CandyShared>),
    Set(HashSet<CandyShared>),
}

impl Hash for CandyShared {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            CandyShared::Int(i) => i.hash(state),
            CandyShared::Int8(i) => i.hash(state),
            CandyShared::Int16(i) => i.hash(state),
            CandyShared::Int32(i) => i.hash(state),
            CandyShared::Int64(i) => i.hash(state),
            CandyShared::Nat(i) => i.hash(state),
            CandyShared::Nat8(i) => i.hash(state),
            CandyShared::Nat16(i) => i.hash(state),
            CandyShared::Nat32(i) => i.hash(state),
            CandyShared::Nat64(i) => i.hash(state),
            CandyShared::Float(i) => i.to_string().hash(state),
            CandyShared::Text(i) => i.hash(state),
            CandyShared::Bool(i) => i.hash(state),
            CandyShared::Blob(i) => i.hash(state),
            CandyShared::Bytes(i) => i.hash(state),
            CandyShared::Class(i) => {
                for prop in i {
                    prop.name.hash(state);
                    prop.value.hash(state);
                    prop.immutable.hash(state);
                }
            }
            CandyShared::Principal(i) => i.hash(state),
            CandyShared::Option(i) => i.hash(state),
            CandyShared::Array(i) => {
                for prop in i {
                    prop.hash(state)
                }
            }
            CandyShared::Nats(i) => i.hash(state),
            CandyShared::Ints(i) => i.hash(state),
            CandyShared::Floats(i) => {
                let mut buffer: Vec<u8> = Vec::new();
                for num in i {
                    buffer.append(num.to_string().into_bytes().as_mut());
                }
                buffer.hash(state)
            }
            CandyShared::Map(i) => {
                for (key, value) in i {
                    key.hash(state);
                    value.hash(state);
                }
            }
            CandyShared::Set(i) => {
                for prop in i {
                    prop.hash(state)
                }
            }
        }
    }
}

impl PartialEq for CandyShared {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CandyShared::Int(i1), CandyShared::Int(i2)) => i1 == i2,
            (CandyShared::Int8(i1), CandyShared::Int8(i2)) => i1 == i2,
            (CandyShared::Int16(i1), CandyShared::Int16(i2)) => i1 == i2,
            (CandyShared::Int32(i1), CandyShared::Int32(i2)) => i1 == i2,
            (CandyShared::Int64(i1), CandyShared::Int64(i2)) => i1 == i2,
            (CandyShared::Nat(i1), CandyShared::Nat(i2)) => i1 == i2,
            (CandyShared::Nat8(i1), CandyShared::Nat8(i2)) => i1 == i2,
            (CandyShared::Nat16(i1), CandyShared::Nat16(i2)) => i1 == i2,
            (CandyShared::Nat32(i1), CandyShared::Nat32(i2)) => i1 == i2,
            (CandyShared::Nat64(i1), CandyShared::Nat64(i2)) => i1 == i2,
            (CandyShared::Float(i1), CandyShared::Float(i2)) => i1 == i2,
            (CandyShared::Text(i1), CandyShared::Text(i2)) => i1 == i2,
            (CandyShared::Bool(i1), CandyShared::Bool(i2)) => i1 == i2,
            (CandyShared::Blob(i1), CandyShared::Blob(i2)) => i1 == i2,
            (CandyShared::Bytes(i1), CandyShared::Bytes(i2)) => i1 == i2,
            (CandyShared::Class(i1), CandyShared::Class(i2)) => i1 == i2,
            (CandyShared::Principal(i1), CandyShared::Principal(i2)) => i1 == i2,
            (CandyShared::Option(i1), CandyShared::Option(i2)) => i1 == i2,
            (CandyShared::Array(i1), CandyShared::Array(i2)) => i1 == i2,
            (CandyShared::Nats(i1), CandyShared::Nats(i2)) => i1 == i2,
            (CandyShared::Ints(i1), CandyShared::Ints(i2)) => i1 == i2,
            (CandyShared::Floats(i1), CandyShared::Floats(i2)) => i1 == i2,
            (CandyShared::Map(map1), CandyShared::Map(map2)) => {
                if map1.len() != map2.len() {
                    false
                } else {
                    for (key1, value1) in map1.iter() {
                        match map2.get(key1) {
                            Some(value2) => {
                                if !(value1 == value2) {
                                    return false;
                                }
                            }
                            None => return false,
                        }
                    }
                    true
                }
            }
            (CandyShared::Set(set1), CandyShared::Set(set2)) => {
                if set1.len() != set2.len() {
                    false
                } else {
                    for element in set1 {
                        if !set2.contains(element) {
                            return false;
                        }
                    }
                    true
                }
            }
            _ => false,
        }
    }
}

impl Eq for CandyShared {}

impl Display for CandyShared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(val) => write!(f, "{}", val.0),
            Self::Int8(val) => write!(f, "{}", val),
            Self::Int16(val) => write!(f, "{}", val),
            Self::Int32(val) => write!(f, "{}", val),
            Self::Int64(val) => write!(f, "{}", val),
            Self::Nat(val) => write!(f, "{}", val.0),
            Self::Nat8(val) => write!(f, "{}", val),
            Self::Nat16(val) => write!(f, "{}", val),
            Self::Nat32(val) => write!(f, "{}", val),
            Self::Nat64(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::Text(val) => write!(f, "{}", val),
            Self::Bool(val) => write!(f, "{}", val),
            Self::Blob(val) => write!(f, "{}", val.encode_hex::<String>()),
            Self::Bytes(val) => {
                write!(f, "{}", val.encode_hex::<String>())
            }
            Self::Class(val) => write!(f, "{}", PropertyShared::stringify_properties(val)),
            Self::Principal(val) => write!(f, "{}", val),
            Self::Option(val) => write!(
                f,
                "{}",
                val.as_ref()
                    .map(|val| val.to_string())
                    .unwrap_or_else(|| "null".to_string())
            ),
            Self::Array(val) => write!(f, "{}", {
                let mut ret = String::new();
                ret.push('[');
                ret.push_str(
                    val.iter()
                        .map(|val| format!("{{{}}}", val))
                        .collect::<Vec<String>>()
                        .join(" ")
                        .as_str(),
                );
                let _ = ret.trim_end();
                ret.push(']');

                ret
            }),
            Self::Nats(val) => write!(f, "{}", {
                let mut ret = String::new();
                ret.push('[');
                ret.push_str(
                    val.iter()
                        .map(|val| val.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                        .as_str(),
                );
                let _ = ret.trim_end();
                ret.push(']');
                ret
            }),
            Self::Ints(val) => write!(f, "{}", {
                let mut ret = String::new();
                ret.push('[');
                ret.push_str(
                    val.iter()
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
                let mut ret = String::new();
                ret.push('[');
                ret.push_str(
                    val.iter()
                        .map(|val| val.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                        .as_str(),
                );
                let _ = ret.trim_end();
                ret.push(']');

                ret
            }),
            Self::Map(val) => write!(f, "{:?}", val),
            Self::Set(val) => write!(f, "{:?}", val),
        }
    }
}

macro_rules! to_nat_of_size {
    ($x:tt, $method: ident) => {
        match $x {
            Self::Nat(val) => val.0.$method(),
            Self::Nat8(val) => val.$method(),
            Self::Nat16(val) => val.$method(),
            Self::Nat32(val) => val.$method(),
            Self::Nat64(val) => val.$method(),
            Self::Float(val) => match val {
                val if val < 0.0 => None,
                _ => val.round().$method(),
            },
            Self::Int(val) => val.0.$method(),
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
            Self::Nat(val) => val.0.$method(),
            Self::Nat8(val) => val.$method(),
            Self::Nat16(val) => val.$method(),
            Self::Nat32(val) => val.$method(),
            Self::Nat64(val) => val.$method(),
            Self::Float(val) => val.round().$method(),
            Self::Int(val) => val.0.$method(),
            Self::Int8(val) => val.$method(),
            Self::Int16(val) => val.$method(),
            Self::Int32(val) => val.$method(),
            Self::Int64(val) => val.$method(),
            _ => None,
        }
    };
}

impl CandyShared {
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
            Self::Nat(val) => val.0.to_f64(),
            Self::Nat8(val) => val.to_f64(),
            Self::Nat16(val) => val.to_f64(),
            Self::Nat32(val) => val.to_f64(),
            Self::Nat64(val) => val.to_f64(),
            Self::Float(val) => Some(val),
            Self::Int(val) => val.0.to_f64(),
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
        match self {
            Self::Blob(val) => val,
            Self::Bytes(val) => val,
            Self::Text(val) => val.chars().flat_map(|c| (c as u32).to_be_bytes()).collect(),
            Self::Int(val) => val.0.to_blob(),
            Self::Nat(val) => val.0.to_blob(),
            Self::Nat8(val) => [val].to_vec(),
            Self::Nat16(val) => val.to_be_bytes().to_vec(),
            Self::Nat32(val) => val.to_be_bytes().to_vec(),
            Self::Nat64(val) => val.to_be_bytes().to_vec(),
            Self::Principal(val) => val.as_slice().into(),
            _ => ic_cdk::trap(format!("Cannot convert to blob {}", self).as_str()),
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
            Self::Class(val) => PropertyShared::props_to_json(&val),
            Self::Array(val) => format!(
                "[{}]",
                val.iter()
                    .map(|i| i.clone().to_json())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Option(val) => match val {
                Some(val) => val.to_json(),
                None => "null".to_string(),
            },
            Self::Nats(val) => format!(
                "[{}]",
                val.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Ints(val) => format!(
                "[{}]",
                val.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Floats(val) => format!(
                "[{}]",
                val.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Self::Bytes(val) => format!("\"{}\"", val.encode_hex::<String>()),
            Self::Blob(val) => format!("\"{}\"", val.encode_hex::<String>()),
            Self::Principal(val) => format!("\"{}\"", val),
            Self::Bool(val) => format!("\"{}\"", val),
            _ => "".to_string(),
        }
    }

    pub fn to_value_array(self) -> Option<Vec<CandyShared>> {
        match self {
            Self::Array(val) => Some(val),
            _ => None,
        }
    }

    // Return the size of the value in bytes
    ///
    /// ```
    /// use candy::value::CandyShared;
    /// use candy::workspace::DataZone;
    /// use crate::candy::workspace::DataZoneTrait;
    /// use crate::candy::value::ToCandyValue;
    /// use candid::Principal;
    ///
    ///
    /// let dz : CandyShared = vec![0_u8;3_000_000].to_candy();
    /// assert_eq!(dz.get_value_size(),3000196);
    /// ```
    pub fn get_value_size(&self) -> u128 {
        Encode!(self).unwrap().len() as u128
    }

    pub fn stringify_array_of_values(vals: &[CandyShared]) -> String {
        let mut result = String::new();
        result.push('[');
        for value in vals {
            let converted = format!("{{{}}} ", value.clone());
            result.push_str(&converted);
        }
        let mut trimmed = result.trim_end().to_string();
        trimmed.push(']');
        trimmed
    }
}

macro_rules! impl_from {
    ($($t:ty => $v:ident),*) => {
        $(impl From<$t> for CandyShared {
            fn from(value: $t) -> Self {
                CandyShared::$v(value)
            }
        })*
    };
}

impl_from!(
    i8 => Int8,
    i16 => Int16,
    i32 => Int32,
    i64 => Int64,
    u8 => Nat8,
    u16 => Nat16,
    u32 => Nat32,
    u64 => Nat64,
    f64 => Float
);

impl_from!(
    String => Text,
    bool => Bool,
    Vec<PropertyShared> => Class,
    Principal => Principal ,
    Option<Box<CandyShared >> => Option,
    Vec<u8> => Blob,
    HashMap<CandyShared,CandyShared> => Map,
    HashSet<CandyShared> => Set
);

impl From<u128> for CandyShared {
    fn from(value: u128) -> Self {
        CandyShared::Nat(candid::Nat::from(value))
    }
}

impl From<BigInt> for CandyShared {
    fn from(value: BigInt) -> Self {
        CandyShared::Int(candid::Int::from(value))
    }
}

impl From<BigUint> for CandyShared {
    fn from(value: BigUint) -> Self {
        CandyShared::Nat(candid::Nat::from(value))
    }
}

impl From<i128> for CandyShared {
    fn from(value: i128) -> Self {
        CandyShared::Int(candid::Int::from(value))
    }
}

impl From<&str> for CandyShared {
    fn from(value: &str) -> Self {
        CandyShared::Text(value.to_string())
    }
}

impl From<Vec<CandyShared>> for CandyShared {
    fn from(value: Vec<CandyShared>) -> Self {
        CandyShared::Array(value)
    }
}

impl From<Vec<candid::Nat>> for CandyShared {
    fn from(value: Vec<candid::Nat>) -> Self {
        CandyShared::Nats(value)
    }
}

impl From<Vec<candid::Int>> for CandyShared {
    fn from(value: Vec<candid::Int>) -> Self {
        CandyShared::Ints(value)
    }
}

impl From<Vec<u128>> for CandyShared {
    fn from(value: Vec<u128>) -> Self {
        CandyShared::Nats(value.into_iter().map(candid::Nat::from).collect())
    }
}

impl From<Vec<i128>> for CandyShared {
    fn from(value: Vec<i128>) -> Self {
        CandyShared::Ints(value.into_iter().map(candid::Int::from).collect())
    }
}

impl From<Vec<f64>> for CandyShared {
    fn from(value: Vec<f64>) -> Self {
        CandyShared::Floats(value)
    }
}

pub trait ToCandyValue {
    fn to_candy(self) -> CandyShared;
}

macro_rules! to_candy {
    ($($t:ty),*) => {
        $(impl ToCandyValue for $t {
            #[inline]
            fn to_candy(self) -> CandyShared {
                CandyShared::from(self)
            }
        })*
    };
}

to_candy!(
    BigInt,
    BigUint,
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
    Vec<i128>,
    Vec<f64>,
    String,
    bool,
    Vec<PropertyShared>,
    Principal,
    Option<Box<CandyShared>>,
    Vec<u8>,
    Vec<CandyShared>,
    &str,
    HashMap<CandyShared, CandyShared>,
    HashSet<CandyShared>
);

//declate const in trait
pub trait ToBlob {
    fn to_blob(self) -> Vec<u8>;
}

impl ToBlob for BigUint {
    #[inline]
    fn to_blob(self) -> Vec<u8> {
        let mut b = self;
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            let a = (b.clone() % BigUint::from(256_u32))
                .to_u8()
                .unwrap_or_else(|| ic_cdk::trap("Can not convert BigUint to u8"));
            b /= BigUint::from(256_u32);
            bytes.push(a);
            if b == BigUint::from(0_u32) {
                break;
            }
        }
        bytes.reverse();
        bytes
    }
}

impl ToBlob for BigInt {
    #[inline]
    fn to_blob(self) -> Vec<u8> {
        let c = u8::from(self < BigInt::from(0_i32));
        let mut b = self.abs();
        let mut bytes: Vec<u8> = vec![];
        loop {
            let a = (b.clone() % BigInt::from(128_i32))
                .to_u8()
                .unwrap_or_else(|| ic_cdk::trap("Can not convert BigInt to u8"));
            b /= BigInt::from(128_i32);
            bytes.push(a);
            if b == BigInt::from(0_i32) {
                break;
            }
        }
        bytes.reverse();
        bytes.insert(0, c);
        bytes
    }
}

impl ToBlob for char {
    #[inline]
    fn to_blob(self) -> Vec<u8> {
        (self as u32).to_be_bytes().to_vec()
    }
}
