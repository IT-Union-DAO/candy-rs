use candid::export_service;
use candid::CandidType;
use ic_cdk::api::call::CallResult;
use serde::{Deserialize, Serialize};

use candy::value::CandyShared;
use candy::value::*;
use candy::workspace::{AddressedChunkArray, ChunkingType};

pub mod bytes;
pub mod icc;
pub mod json;
pub mod size;
pub mod workspace;

#[ic_cdk::query(name = "__get_candid")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;
        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("rust.did"), export_candid()).expect("Write failed.");
    }
}
