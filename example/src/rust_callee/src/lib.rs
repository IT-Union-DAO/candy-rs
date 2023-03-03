use candid::export_service;
use candy::value::CandyValue;
pub mod query;

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
