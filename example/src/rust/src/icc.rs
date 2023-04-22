use candid::{candid_method, Principal};
use ic_candy::value::CandyShared;
use ic_candy::value::ToCandyValue;
use ic_candy::workspace::ChunkingType;
use ic_cdk::api::call::CallResult;
use ic_cdk::call;
use ic_cdk_macros::{query, update};

// inter canister call
#[update]
#[candid_method(update)]
pub async fn get_candy(principal: String, method: String) -> CandyShared {
    let principal = Principal::from_text(principal).unwrap();
    let res: CallResult<(CandyShared,)> = call(principal, &method, ()).await;
    ic_cdk::println!("{:?}", &res);
    println!("{:?}", res);
    res.unwrap().0
}
