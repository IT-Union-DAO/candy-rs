use candid::candid_method;
use candid::Principal;
use ic_cdk_macros::query;

use ic_candy::value::CandyShared;
use ic_candy::value::ToCandyValue;

#[query]
#[candid_method(query)]
pub fn candy_blob_to_blob() -> Vec<u8> {
    CandyShared::Blob(vec![0, 1, 2, 3, 4, 5, 6, 7]).to_blob()
}

//Bytes
#[query]
#[candid_method(query)]
pub fn candy_bytes_to_blob() -> Vec<u8> {
    CandyShared::Bytes(vec![0, 1, 2, 3, 4, 5, 6, 7]).to_blob()
}

//Text
#[query]
#[candid_method(query)]
pub fn candy_text_to_blob() -> Vec<u8> {
    "Hello, world!".to_candy().to_blob()
}

//Nat
#[query]
#[candid_method(query)]
pub fn candy_nat_to_blob() -> Vec<u8> {
    CandyShared::Nat(candid::Nat::from(1_234_567_890_u128)).to_blob()
}

//Int
#[query]
#[candid_method(query)]
pub fn candy_int_to_blob() -> Vec<u8> {
    CandyShared::Int(candid::Int::from(-123_456_789_000_i128)).to_blob()
}

#[query]
#[candid_method(query)]
pub fn candy_nat8_to_blob() -> Vec<u8> {
    CandyShared::Nat8(255).to_blob()
}

#[query]
#[candid_method(query)]
pub fn candy_nat16_to_blob() -> Vec<u8> {
    CandyShared::Nat16(65535).to_blob()
}

#[query]
#[candid_method(query)]
pub fn candy_nat32_to_blob() -> Vec<u8> {
    CandyShared::Nat32(123_456_789).to_blob()
}

#[query]
#[candid_method(query)]
pub fn candy_nat64_to_blob() -> Vec<u8> {
    CandyShared::Nat64(9_223_372_036_854_775_807).to_blob()
}

//Principal
#[query]
#[candid_method(query)]
pub fn candy_principal_to_blob() -> Vec<u8> {
    Principal::management_canister().to_candy().to_blob()
}

//Float (traps)
#[query]
#[candid_method(query)]
pub fn candy_float_to_blob() -> Vec<u8> {
    CandyShared::Float(1.234).to_blob()
}
