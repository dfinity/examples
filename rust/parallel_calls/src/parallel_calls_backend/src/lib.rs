#[ic_cdk::update]

#[ic_cdk::update]
pub async fn sequential_calls(n: u64) -> String {
    for i in 0..n {
        mirror(i).await;
    }
}
