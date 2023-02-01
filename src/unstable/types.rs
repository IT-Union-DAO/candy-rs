use candid::Nat;

use crate::unstable::values::CandyValueUnstable;

pub type PropertiesUnstable= Vec<PropertyUnstable>;

#[derive(Clone)]
pub struct PropertyUnstable{
    pub name: String,
    pub value: CandyValueUnstable,
    pub immutable: bool,
}

#[derive(Clone)]
pub enum Floats{
    Frozen(Box<[f64]>),
    Thawed(Vec<f64>),
}

#[derive(Clone)]
pub enum Nats{
    Frozen(Box<[Nat]>),
    Thawed(Vec<Nat>),
}

#[derive(Clone)]
pub enum Array{
    Frozen(Box<[CandyValueUnstable]>),
    Thawed(Vec<CandyValueUnstable>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Bytes{
    Frozen(Box<[u8]>),
    Thawed(Vec<u8>),
}