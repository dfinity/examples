use ic_cdk::query;

#[query]
fn get_value() -> Option<String> {
    Some("Hello, world!".to_string())
}
