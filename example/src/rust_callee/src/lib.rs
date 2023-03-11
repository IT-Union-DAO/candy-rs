use candid::export_service;
use candid::CandidType;
use candy::value::CandyValue;
use serde::{Deserialize, Serialize};
pub mod query;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum ChunkingType {
    #[serde(rename = "eof")]
    Eof,
    #[serde(rename = "chunk")]
    Chunk,
}

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

        let dir = PathBuf::from(env::current_dir().unwrap().parent().unwrap());
        write(
            dir.join("declarations/rust/rust_callee.did"),
            export_candid(),
        )
        .expect("Write failed.");
    }
}
