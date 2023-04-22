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

/**
 * `CandyShared` is a Rust enum that provides a wrapper type for convenient data manipulation inside ICP canisters.
 * This enum includes various data types that can be used for communication between canisters, including integers, floats, text, boolean, and more.
 *
 */
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
    /**
     * `to_nat` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a natural number of size `u128`.
     * This method uses the `to_nat_of_size!` macro to perform the conversion.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared, ToCandyValue};
     *
     * let value = 42_u128.to_candy();
     * let result = value.to_nat();
     * assert_eq!(result, Some(42));
     * ```
     */
    pub fn to_nat(self) -> Option<u128> {
        to_nat_of_size!(self, to_u128)
    }

    /**
     * `to_nat8` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to an 8-bit natural number (`u8`).
     * This method uses the `to_nat_of_size!` macro to perform the conversion.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared, ToCandyValue};
     *
     * let value = 420000_u128.to_candy();
     * let result = value.to_nat8();
     * assert_eq!(result, None);
     *
     * let value = 42_u8.to_candy();
     * let result = value.to_nat8();
     * assert_eq!(result, Some(42));
     * ```
     */
    pub fn to_nat8(self) -> Option<u8> {
        to_nat_of_size!(self, to_u8)
    }

    /**
     * `to_nat16` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a natural number of size `u16`.
     * This method uses the `to_nat_of_size!` macro to perform the conversion.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared,ToCandyValue};
     *
     * let value = 42_u16.to_candy();
     * let result = value.to_nat16();
     * assert_eq!(result, Some(42));
     * ```
     */
    pub fn to_nat16(self) -> Option<u16> {
        to_nat_of_size!(self, to_u16)
    }

    /**
     * `to_nat32` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a natural number of size `u32`.
     * This method uses the `to_nat_of_size!` macro to perform the conversion.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared,ToCandyValue};
     *
     * let value = 42_u32.to_candy();
     * let result = value.to_nat32();
     * assert_eq!(result, Some(42));
     * ```
     */
    pub fn to_nat32(self) -> Option<u32> {
        to_nat_of_size!(self, to_u32)
    }

    /**
     * `to_nat64` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a natural number of size `u64`.
     * This method returns an `Option<u64>` that contains the resulting `u64` value if the conversion is successful, and `None` otherwise.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared, ToCandyValue};
     *
     * let value = 42_u128.to_candy();
     * let result = value.to_nat64();
     * assert_eq!(result, Some(42));
     * ```
     */
    pub fn to_nat64(self) -> Option<u64> {
        to_nat_of_size!(self, to_u64)
    }

    /**
     * `to_int` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a signed integer of size `i128`.
     * This method uses the `to_int_of_size!` macro to perform the conversion.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared, ToCandyValue};
     *
     * let value = 42_u128.to_candy();
     * let result = value.to_int();
     * assert_eq!(result, Some(42));
     * ```
     */
    pub fn to_int(self) -> Option<i128> {
        to_int_of_size!(self, to_i128)
    }

    /**
     * `to_int8` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to an `i8` signed integer.
     * This method returns an `Option<i8>` that contains the resulting `i8` value if the conversion is successful, and `None` otherwise.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared,ToCandyValue};
     *
     * let value = 42_u128.to_candy();
     * let result = value.to_int8();
     * assert_eq!(result, Some(42));
     * ```
     */
    pub fn to_int8(self) -> Option<i8> {
        to_int_of_size!(self, to_i8)
    }

    /**
     * `to_int16` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a signed 16-bit integer (`i16`).
     * This method returns an `Option<i16>` that contains the resulting `i16` value if the conversion is successful, and `None` otherwise.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared, ToCandyValue};
     *
     * let value = 42_i128.to_candy();
     * let result = value.to_int16();
     * assert_eq!(result, Some(42));
     *
     * let value = (-32768_i64).to_candy();
     * let result = value.to_int16();
     * assert_eq!(result, Some(-32768));
     * ```
     */
    pub fn to_int16(self) -> Option<i16> {
        to_int_of_size!(self, to_i16)
    }

    /**
     * `to_int32` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a 32-bit signed integer.
     * This method returns an `Option<i32>` that contains the resulting `i32` value if the conversion is successful, and `None` otherwise.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared, ToCandyValue};
     *
     * let value = 42_i128.to_candy();
     * let result = value.to_int32();
     * assert_eq!(result, Some(42));
     * ```
     */
    pub fn to_int32(self) -> Option<i32> {
        to_int_of_size!(self, to_i32)
    }

    /**
     * `to_int64` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a signed integer of size `i64`.
     * This method uses the `to_int_of_size!` macro to perform the conversion.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared, ToCandyValue};
     *
     * let value = 42_i64.to_candy();
     * let result = value.to_int64();
     * assert_eq!(result, Some(42));
     * ```
     */
    pub fn to_int64(self) -> Option<i64> {
        to_int_of_size!(self, to_i64)
    }

    /**
     * `to_float` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a `f64` floating-point number.
     * This method returns an `Option<f64>` that contains the resulting `f64` value if the conversion is successful, and `None` otherwise.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared,ToCandyValue};
     *
     * let value = 42_u128.to_candy();
     * let result = value.to_float();
     * assert_eq!(result, Some(42.0));
     * ```
     */
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

    /**
     * `to_bool` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a `bool` boolean.
     * This method returns an `Option<bool>` that contains the resulting `bool` value if the conversion is successful, and `None` otherwise.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared, ToCandyValue};
     *
     * let value = true.to_candy();
     * let result = value.to_bool();
     * assert_eq!(result, Some(true));
     * ```
     */
    pub fn to_bool(self) -> Option<bool> {
        match self {
            Self::Bool(val) => Some(val),
            _ => None,
        }
    }

    /**
     * `to_principal` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a `Principal`.
     * This method returns an `Option<Principal>` that contains the resulting `Principal` value if the conversion is successful, and `None` otherwise.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared, ToCandyValue};
     * use candid::Principal;
     *
     * let principal = Principal::anonymous();
     * let value = principal.clone().to_candy();
     * let result = value.to_principal();
     * assert_eq!(result, Some(principal));
     * ```
     */
    pub fn to_principal(self) -> Option<Principal> {
        match self {
            Self::Principal(val) => Some(val),
            _ => None,
        }
    }

    /**
     * `to_blob` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a `Vec<u8>` blob.
     * This method returns a `Vec<u8>` that contains the binary representation of the value.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared,ToCandyValue};
     *
     * let value = "Hello, world!".to_candy();
     * let result = value.to_blob();
     * assert_eq!(result, vec![0, 0, 0, 72, 0, 0, 0, 101, 0, 0, 0, 108, 0, 0, 0, 108, 0, 0, 0, 111, 0, 0, 0, 44, 0, 0, 0, 32, 0, 0, 0, 119, 0, 0, 0, 111, 0, 0, 0, 114, 0, 0, 0, 108, 0, 0, 0, 100, 0, 0, 0, 33]);
     * ```
     */
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

    /**
     * `to_json` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a JSON string.
     * This method returns a `String` that represents the resulting JSON.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared,ToCandyValue};
     *
     * let value = 42_u128.to_candy();
     * let result = value.to_json();
     * assert_eq!(result, "42".to_string());
     * ```
     */
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

    /**
     * `to_value_array` is a method defined on the `CandyShared` Rust enum that provides a convenient way to convert a value to a `Vec<CandyShared>` array.
     * This method returns an `Option<Vec<CandyShared>>` that contains the resulting array if the conversion is successful, and `None` otherwise.
     *
     * # Examples
     *
     * ```
     * use ic_candy::value::{CandyShared,ToCandyValue};
     *
     * let arr = vec![42_u128.to_candy(), 50_u128.to_candy()];
     * let value = CandyShared::Array(arr);
     * let result = value.to_value_array();
     * assert_eq!(result, Some(vec![42_u128.to_candy(), 50_u128.to_candy()]));
     * ```
     */
    pub fn to_value_array(self) -> Option<Vec<CandyShared>> {
        match self {
            Self::Array(val) => Some(val),
            _ => None,
        }
    }

    // Return the size of the value in bytes
    ///
    /// ```
    /// use ic_candy::value::CandyShared;
    /// use ic_candy::workspace::DataZone;
    /// use crate::ic_candy::workspace::DataZoneTrait;
    /// use crate::ic_candy::value::ToCandyValue;
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

/**
 * `ToCandyValue` is a trait that defines a method `to_candy`, which is used to convert a value of any type into a `CandyShared` enum.
 * This trait can be implemented by any type that can be converted into a `CandyShared` enum, allowing for seamless integration with the `CandyShared` type in Rust code.
 *
 * # Examples
 *
 * ```
 * use crate::ic_candy::value::{CandyShared, ToCandyValue};
 *
 * let value = 42u8;
 * let result = value.to_candy();
 * assert_eq!(result, CandyShared::Nat8(42));
 * ```
 */
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

// Implementations
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

/**
 * `ToBlob` is a trait that is implemented by types that can be converted to a `Vec<u8>` blob.
 *
 * The `to_blob` method defined in this trait is used to perform the conversion to the `Vec<u8>` blob.
 *
 * # Examples
 *
 * ```
 * use crate::ic_candy::value::{CandyShared, ToBlob, ToCandyValue};
 *
 * let value = 42_u128.to_candy();
 * let result = value.to_blob();
 * assert_eq!(result, vec![42]);
 * ```
 */
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
