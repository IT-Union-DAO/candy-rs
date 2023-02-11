use candid::{CandidType, Nat};
use hex::ToHex;
use serde::{Deserialize, Serialize};

use crate::stable::value::CandyValue;
use crate::unstable::types::PropertyUnstable;

pub type Properties = Box<[Property]>;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: CandyValue,
    pub immutable: bool,
}

impl ToString for Property {
    fn to_string(&self) -> String {
        let desc = match self.immutable {
            true => "var ".to_string(),
            false => "".to_string(),
        };
        format!(
            "{}:{} {}; ",
            self.name,
            desc,
            self.value.clone().to_string()
        )
    }
}

impl Property {
    pub fn destabilize_property(self) -> PropertyUnstable {
        PropertyUnstable {
            name: self.name.to_string(),
            value: self.value.destabilize_value(),
            immutable: self.immutable,
        }
    }

    pub fn stringify_properties(props: &[Property]) -> String {
        let mut ret = String::new();
        ret.push_str("{");
        for prop in props {
            ret.push_str(&prop.to_string());
        }
        let _ = ret.trim_end();
        ret.push_str("}");
        ret
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum PropertyError {
    Unauthorized,
    NotFound,
    InvalidRequest,
    AuthorizedPrincipalLimitReached(Nat),
    Immutable,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Array {
    Frozen(Box<[CandyValue]>),
    Thawed(Box<[CandyValue]>),
}

impl Array {
    pub fn frozen(vals: Box<[CandyValue]>) -> Self {
        Self::Frozen(vals)
    }

    pub fn thawed(vals: Box<[CandyValue]>) -> Self {
        Self::Thawed(vals)
    }
}

impl ToString for Array {
    fn to_string(&self) -> String {
        match self {
            Self::Frozen(val) => CandyValue::stringify_array_of_values(val),
            Self::Thawed(val) => CandyValue::stringify_array_of_values(val),
        }
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Nats {
    Frozen(Box<[Nat]>),
    Thawed(Box<[Nat]>),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Floats {
    Frozen(Box<[f64]>),
    Thawed(Box<[f64]>),
}

macro_rules! candy_nums_to_string {
    ($lhs:ty) => {
        impl ToString for $lhs {
            fn to_string(&self) -> String {
                let mut ret = String::new();
                ret.push_str("[");

                match self {
                    Self::Frozen(val) => val
                        .iter()
                        .map(|val| val.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                    Self::Thawed(val) => val
                        .iter()
                        .map(|val| val.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                };

                let _ = ret.trim_end();
                ret.push_str("]");

                ret
            }
        }
    };
}

candy_nums_to_string!(Floats);
candy_nums_to_string!(Nats);

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Bytes {
    Frozen(Box<[u8]>),
    Thawed(Box<[u8]>),
}

impl ToString for Bytes {
    fn to_string(&self) -> String {
        match self {
            Self::Frozen(val) => val.encode_hex::<String>(),
            Self::Thawed(val) => val.encode_hex::<String>(),
        }
    }
}
