use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=DFX_NETWORK");
    if matches!(env::var("DFX_NETWORK"), Ok(network) if network == "ic") {
        println!("cargo:rustc-cfg=mainnet");
    }
}
