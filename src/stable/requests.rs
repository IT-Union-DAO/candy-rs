use candid::Nat;

use crate::stable::values::CandyValue;

pub struct Query {
    name: String,
    next: Box<[Query]>,
}

pub enum QueryMode {
    All,
    Some(Box<[Query]>),
}

pub struct Update {
    name: String,
    mode: UpdateMode,
}

pub enum UpdateMode {
    Set(CandyValue),
    Lock(CandyValue),
    Next(Box<[Update]>),
}

pub struct UpdateRequest {
    id: String,
    update: Box<[Update]>,
}

pub type AddressedChunk = (Nat, Nat, CandyValue);
pub type AddressedChunkArray = Box<[AddressedChunk]>;
pub type AddressedChunkBuffer = Vec<AddressedChunk>;
