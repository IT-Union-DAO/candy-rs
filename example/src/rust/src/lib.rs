use candid::{candid_method, export_service};
use candy::value::CandyValue;
use candy::value::ToCandyValue;

#[ic_cdk_macros::query]
#[candid_method(query)]
pub fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk_macros::query]
#[candid_method(query)]
pub fn candy(name: String) -> CandyValue {
    1_u8.to_candy()
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

        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("rust.did"), export_candid()).expect("Write failed.");
    }
}
