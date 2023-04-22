use candid::candid_method;
use candid::Principal;
use candy::types::PropertyShared;
use candy::value::CandyShared;
use candy::value::ToCandyValue;
use candy::workspace::ChunkingType;
use ic_cdk_macros::{query, update};

#[query]
#[candid_method(query)]
pub fn candy_nat_to_json() -> String {
    CandyShared::Nat(candid::Nat::from(9_223_372_036_854_775_807_u128)).to_json()
}

#[query]
#[candid_method(query)]
pub fn candy_nat8_to_json() -> String {
    CandyShared::Nat8(255).to_json()
}

#[query]
#[candid_method(query)]
pub fn candy_nat16_to_json() -> String {
    CandyShared::Nat16(65535).to_json()
}

#[query]
#[candid_method(query)]
pub fn candy_nat32_to_json() -> String {
    CandyShared::Nat32(123_456_789).to_json()
}

//Nat64
#[query]
#[candid_method(query)]
pub fn candy_nat64_to_json() -> String {
    CandyShared::Nat64(9_223_372_036_854_775_807).to_json()
}

#[query]
#[candid_method(query)]
pub fn candy_int_to_json() -> String {
    CandyShared::Int(candid::Int::from(-123_456_789_000_i128)).to_json()
}

//Int8
#[query]
#[candid_method(query)]
pub fn candy_int8_to_json() -> String {
    CandyShared::Int8(-128).to_json()
}

//Int16
#[query]
#[candid_method(query)]
pub fn candy_int16_to_json() -> String {
    CandyShared::Int16(-32768).to_json()
}

//Int32
#[query]
#[candid_method(query)]
pub fn candy_int32_to_json() -> String {
    CandyShared::Int32(-2147483648).to_json()
}

//Int64
#[query]
#[candid_method(query)]
pub fn candy_int64_to_json() -> String {
    CandyShared::Int64(-9223372036854775808).to_json()
}

//Text
#[query]
#[candid_method(query)]
pub fn candy_text_to_json() -> String {
    "Hello, world!".to_candy().to_json()
}

//Class
#[query]
#[candid_method(query)]
pub fn candy_class_to_json() -> String {
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
    .to_json()
}

//Array
#[query]
#[candid_method(query)]
pub fn candy_array_to_json() -> String {
    CandyShared::Array(vec![15_u128.to_candy(), (-15_i128).to_candy()]).to_json()
}

//Option
#[query]
#[candid_method(query)]
pub fn candy_option_to_json() -> String {
    CandyShared::Option(None).to_json()
}

#[query]
#[candid_method(query)]
pub fn candy_option_some_to_json() -> String {
    CandyShared::Option(Some(Box::from(15_u128.to_candy()))).to_json()
}

//Nats
#[query]
#[candid_method(query)]
pub fn candy_nats_to_json() -> String {
    vec![1_u128, 2_u128, 3_u128, 4_u128].to_candy().to_json()
}

//Ints
#[query]
#[candid_method(query)]
pub fn candy_ints_to_json() -> String {
    vec![1_i128, 2_i128, 3_i128, 4_i128].to_candy().to_json()
}

//Floats
#[query]
#[candid_method(query)]
pub fn candy_floats_to_json() -> String {
    vec![1.0, 2.0, 3.5, 4.123].to_candy().to_json()
}

//Bytes
#[query]
#[candid_method(query)]
pub fn candy_bytes_to_json() -> String {
    CandyShared::Bytes(vec![0, 1, 2, 3, 4, 5, 6, 7]).to_json()
}

//Blob
#[query]
#[candid_method(query)]
pub fn candy_blob_to_json() -> String {
    CandyShared::Blob(vec![0, 1, 2, 3, 4, 5, 6, 7]).to_json()
}

//Principal
#[query]
#[candid_method(query)]
pub fn candy_principal_to_json() -> String {
    Principal::from_text("qjdve-lqaaa-aaaaa-aaaeq-cai")
        .unwrap()
        .to_candy()
        .to_json()
}

//Float
#[query]
#[candid_method(query)]
pub fn candy_float_to_json() -> String {
    CandyShared::Float(1.234).to_json()
}

//Bool
#[query]
#[candid_method(query)]
pub fn candy_bool_to_json() -> String {
    CandyShared::Bool(true).to_json()
}
