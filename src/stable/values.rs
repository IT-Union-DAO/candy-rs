use candid::{Int, Nat, Principal};

use crate::common::common::Blob;
use crate::stable::types::{Bytes, Floats, Nats, Property};
use crate::stable::types::Array;

pub enum CandyValue {
    Int(Int),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Nat(Nat),
    Nat8(u8),
    Nat16(u16),
    Nat32(u32),
    Nat64(u64),
    Float(f64),
    Text(String),
    Bool(bool),
    Blob(Blob),
    Class(Box<[Property]>),
    Principal(Principal),
    Option(Option<Box<CandyValue>>),
    Array(Array),
    Nats(Nats),
    Floats(Floats),
    Bytes(Bytes),
    Empty,
}


