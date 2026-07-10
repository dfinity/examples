use ic_cdk::export_candid;

#[ic_cdk::update]
pub async fn ping() {}

export_candid!();
