use std::ptr::NonNull;
use candid::{Int, Nat, Principal};

use crate::common::common::Blob;
use crate::unstable::types::{Array, Bytes, Floats, Nats};
use crate::unstable::types::PropertyUnstable;

#[derive(Clone)]
pub enum CandyValueUnstable {
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
    Class(Box<[PropertyUnstable]>),
    Principal(Principal),
    Floats(Floats),
    Nats(Nats),
    Array(Array),
    Option(Option<Box<CandyValueUnstable>>),
    Bytes(Bytes),
    Empty,
}
