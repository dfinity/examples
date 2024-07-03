use ic_cdk::query;

#[query]
fn hello() -> String {
    "Hello, world!".to_string()
}
