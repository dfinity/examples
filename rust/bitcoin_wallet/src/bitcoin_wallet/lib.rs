use ic_cdk::{api::caller, export::Principal};
use ic_cdk_macros::query;

#[query]
async fn whoami() -> Principal {
    caller()
}
