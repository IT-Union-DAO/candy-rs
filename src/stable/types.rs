use candid::Nat;

use crate::stable::values::CandyValue;

pub type Properties = Vec<Property>;

pub struct Property {
    pub name: String,
    pub value: CandyValue,
    pub immutable: bool,
}

pub enum PropertyError {
    Unauthorized,
    NotFound,
    InvalidRequest,
    AuthorizedPrincipalLimitReached(Nat),
    Immutable,
}

pub enum Array {
    Frozen(Box<[CandyValue]>),
    Thawed(Box<[CandyValue]>),
}

pub enum Nats {
    Frozen(Box<[Nat]>),
    Thawed(Box<[Nat]>),
}

pub enum Floats {
    Frozen(Box<[f64]>),
    Thawed(Box<[f64]>),
}

pub enum Bytes {
    Frozen(Box<[u8]>),
    Thawed(Box<[u8]>),
}
