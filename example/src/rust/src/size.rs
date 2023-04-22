use std::collections::{HashMap, HashSet};

use candid::candid_method;
use candid::Principal;
use ic_cdk_macros::{query, update};

use ic_candy::types::PropertyShared;
use ic_candy::value::CandyShared;
use ic_candy::value::ToCandyValue;
use ic_candy::workspace::ChunkingType;

//Int
#[query]
#[candid_method(query)]
pub fn size_of_candy_int() -> u128 {
    CandyShared::Int(candid::Int::from(-123_456_789_000_i128)).get_value_size()
}

//Int8
#[query]
#[candid_method(query)]
pub fn size_of_candy_int8() -> u128 {
    CandyShared::Int8(-128).get_value_size()
}

//Int16
#[query]
#[candid_method(query)]
pub fn size_of_candy_int16() -> u128 {
    CandyShared::Int16(-32768).get_value_size()
}

//Int32
#[query]
#[candid_method(query)]
pub fn size_of_candy_int32() -> u128 {
    CandyShared::Int32(-2_147_483_648).get_value_size()
}

//Int64
#[query]
#[candid_method(query)]
pub fn size_of_candy_int64() -> u128 {
    CandyShared::Int64(-9_223_372_036_854_775_808).get_value_size()
}

//Nat
#[query]
#[candid_method(query)]
pub fn size_of_candy_nat() -> u128 {
    CandyShared::Nat(candid::Nat::from(1_234_567_890)).get_value_size()
}

//Nat8
#[query]
#[candid_method(query)]
pub fn size_of_candy_nat8() -> u128 {
    CandyShared::Nat8(255).get_value_size()
}

//Nat16
#[query]
#[candid_method(query)]
pub fn size_of_candy_nat16() -> u128 {
    CandyShared::Nat16(65535).get_value_size()
}

//Nat32
#[query]
#[candid_method(query)]
pub fn size_of_candy_nat32() -> u128 {
    CandyShared::Nat32(123_456_789).get_value_size()
}

//Nat64
#[query]
#[candid_method(query)]
pub fn size_of_candy_nat64() -> u128 {
    CandyShared::Nat64(9_223_372_036_854_775_807).get_value_size()
}

//Float
#[query]
#[candid_method(query)]
pub fn size_of_candy_float() -> u128 {
    CandyShared::Float(1.234).get_value_size()
}

//Text
#[query]
#[candid_method(query)]
pub fn size_of_candy_text() -> u128 {
    "Hello, world!".to_candy().get_value_size()
}

//Bool
#[query]
#[candid_method(query)]
pub fn size_of_candy_bool() -> u128 {
    CandyShared::Bool(true).get_value_size()
}

//Blob
#[query]
#[candid_method(query)]
pub fn size_of_candy_blob() -> u128 {
    CandyShared::Blob(vec![0, 1, 2, 3, 4, 5, 6, 7]).get_value_size()
}

//Class
#[query]
#[candid_method(query)]
pub fn size_of_candy_class() -> u128 {
    CandyShared::Class(vec![
        PropertyShared {
            immutable: true,
            name: "test".to_string(),
            value: 15_u128.to_candy(),
        },
        PropertyShared {
            immutable: true,
            name: "test2".to_string(),
            value: (-15_i128).to_candy(),
        },
    ])
    .get_value_size()
}

//Principal
#[query]
#[candid_method(query)]
pub fn size_of_candy_principal() -> u128 {
    CandyShared::Principal(Principal::from_text("qjdve-lqaaa-aaaaa-aaaeq-cai").unwrap())
        .get_value_size()
}

//Option
#[query]
#[candid_method(query)]
pub fn size_of_candy_option() -> u128 {
    CandyShared::Option(Some(Box::from(15_u128.to_candy()))).get_value_size()
}

//Option None
#[query]
#[candid_method(query)]
pub fn size_of_candy_option_none() -> u128 {
    CandyShared::Option(None).get_value_size()
}

//Array
#[query]
#[candid_method(query)]
pub fn size_of_candy_array() -> u128 {
    CandyShared::Array(vec![
        CandyShared::Int(candid::Int::from(-15)),
        CandyShared::Nat(candid::Nat::from(15)),
    ])
    .get_value_size()
}

//Bytes
#[query]
#[candid_method(query)]
pub fn size_of_candy_bytes() -> u128 {
    CandyShared::Bytes(vec![0, 1, 2, 3, 4, 5, 6, 7]).get_value_size()
}

//Floats
#[query]
#[candid_method(query)]
pub fn size_of_candy_floats() -> u128 {
    CandyShared::Floats(vec![1.234, 2.345, 3.456, 4.567, 5.678, 6.789, 7.890]).get_value_size()
}

//Nats
#[query]
#[candid_method(query)]
pub fn size_of_candy_nats() -> u128 {
    vec![1_u128, 2_u128, 3_u128, 4_u128]
        .to_candy()
        .get_value_size()
}

//Ints
#[query]
#[candid_method(query)]
pub fn size_of_candy_ints() -> u128 {
    vec![-1_i128, -2_i128, -3_i128, -4_i128]
        .to_candy()
        .get_value_size()
}

//Map
#[query]
#[candid_method(query)]
pub fn size_of_candy_map() -> u128 {
    let mut map: HashMap<CandyShared, CandyShared> = HashMap::new();
    map.insert(15_u128.to_candy(), 15_u128.to_candy());
    map.to_candy().get_value_size()
}

//Set
#[query]
#[candid_method(query)]
pub fn size_of_candy_set() -> u128 {
    let mut set: HashSet<CandyShared> = HashSet::new();
    set.insert(15_u128.to_candy());
    set.insert(15_u128.to_candy());
    set.to_candy().get_value_size()
}
