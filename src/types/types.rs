use candid::CandidType;
use hex::ToHex;
use serde::{Deserialize, Serialize};

use crate::types::value::CandyValue;

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
        format!("{}:{}{}; ", self.name, desc, self.value.clone().to_text())
    }
}

impl Property {
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
