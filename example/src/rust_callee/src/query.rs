use candid::{candid_method, Principal};
use candy::value::CandyValue;
use candy::value::ToCandyValue;
use ic_cdk_macros::query;

use crate::ChunkingType;
// #[query]
// #[candid_method(query)]
// pub fn candy(name: String) -> CandyValue {
//     1_u8.to_candy()
// }
//

//Create functions for service below
#[query]
#[candid_method(query)]
pub fn get_blob() -> CandyValue {
    Principal::from_text("aaaa-aa").unwrap().to_candy()
}

#[query]
#[candid_method(query)]
pub fn get_type() -> ChunkingType {
    ChunkingType::Eof
}

//Bool
#[query]
#[candid_method(query)]
pub fn get_bool() -> CandyValue {
    true.to_candy()
}

// service : {
// getBlob: () -> (CandyValue) query;
// getBool: () -> (CandyValue) query;
// getClass: () -> (CandyValue) query;
// getEmpty: () -> (CandyValue) query;
// getFloat: () -> (CandyValue) query;
// getFrozenArray: () -> (CandyValue) query;
// getFrozenBytes: () -> (CandyValue) query;
// getFrozenFloats: () -> (CandyValue) query;
// getFrozenNats: () -> (CandyValue) query;
// getFrozenText: () -> (CandyValue) query;
// getInt: () -> (CandyValue) query;
// getInt16: () -> (CandyValue) query;
// getInt32: () -> (CandyValue) query;
// getInt64: () -> (CandyValue) query;
// getInt8: () -> (CandyValue) query;
// getNat: () -> (CandyValue) query;
// getNat16: () -> (CandyValue) query;
// getNat32: () -> (CandyValue) query;
// getNat64: () -> (CandyValue) query;
// getNat8: () -> (CandyValue) query;
// getNullOption: () -> (CandyValue) query;
// getOptSome: () -> (CandyValue) query;
// getPrincipal: () -> (CandyValue) query;
// getText: () -> (CandyValue) query;
// getThawedArray: () -> (CandyValue) query;
// getThawedBytes: () -> (CandyValue) query;
// getThawedFloats: () -> (CandyValue) query;
// getThawedNats: () -> (CandyValue) query;
// }
