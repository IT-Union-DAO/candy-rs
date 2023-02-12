use candid::CandidType;
use hex::ToHex;
use serde::{Deserialize, Serialize};

use crate::stable::value::CandyValue;
use crate::unstable::types::PropertyUnstable;

macro_rules! impl_frozen_thawed {
    ($($t:ty => $v:ident),*) => {
        $(
        impl $v {
             pub fn frozen(vals: $t) -> Self {
                Self::Frozen(vals)
             }
            pub fn thawed(vals: $t) -> Self {
               Self::Thawed(vals)
             }
        })*
    };
}

macro_rules! candy_nums_to_string {
    ( $ lhs: ty) => {
        impl ToString for $lhs {
            fn to_string(&self) -> String {
                let mut ret = String::new();
                ret.push_str("[");

                print!("{:?}", self);
                ret.push_str(
                    match &self {
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
                    }
                    .as_str(),
                );

                let _ = ret.trim_end();
                ret.push_str("]");

                ret
            }
        }
    };
}

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
        format!("{}:{}{}; ", self.name, desc, self.value.clone().to_string())
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
        let prop_strings: Vec<String> = props.iter().map(|p| p.to_string()).collect();
        format!("{{{}}}", prop_strings.join("").trim_end())
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum PropertyError {
    Unauthorized,
    NotFound,
    InvalidRequest,
    AuthorizedPrincipalLimitReached(u128),
    Immutable,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Array {
    Frozen(Box<[CandyValue]>),
    Thawed(Box<[CandyValue]>),
}

impl_frozen_thawed!(Box<[CandyValue]> => Array);

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
    Frozen(Box<[u128]>),
    Thawed(Box<[u128]>),
}

candy_nums_to_string!(Nats);
impl_frozen_thawed!(Box<[u128]> => Nats);

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Floats {
    Frozen(Box<[f64]>),
    Thawed(Box<[f64]>),
}

candy_nums_to_string!(Floats);
impl_frozen_thawed!(Box<[f64]> => Floats);

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Bytes {
    Frozen(Box<[u8]>),
    Thawed(Box<[u8]>),
}

impl_frozen_thawed!(Box<[u8]> => Bytes);

impl ToString for Bytes {
    fn to_string(&self) -> String {
        match self {
            Self::Frozen(val) => val.encode_hex::<String>(),
            Self::Thawed(val) => val.encode_hex::<String>(),
        }
    }
}
