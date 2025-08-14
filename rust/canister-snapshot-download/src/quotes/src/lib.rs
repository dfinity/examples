use ic_cdk::{
    api::stable::{stable_grow, stable_read, stable_write},
    query, update,
};

#[update]
fn setup() {
    stable_grow(1).unwrap();
    let buf = "Colourless green ideas sleep furiously.".as_bytes();
    stable_write(0, buf);
}

#[query]
fn print() -> String {
    let mut buf = vec![0; 39];
    stable_read(0, &mut buf);
    let msg = String::from_utf8(buf).unwrap();
    // ic_cdk::println!("{}", msg);
    msg
}
