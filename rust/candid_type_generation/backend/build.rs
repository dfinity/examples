use ic_cdk_bindgen::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let cargo_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Cannot find manifest dir"));
    let nns_candid_file = cargo_dir.join("..").join("candid").join("nns_governance.did");

    if !nns_candid_file.exists() {
        panic!(
            "nns_candid_file does not exist.  \
                Please run scripts/fetch_candid.sh to get the latest"
        );
    }

    let nns_type_selector = cargo_dir.join("..").join("candid").join("nns_governance.toml");

    Config::new("nns_governance", &nns_candid_file)
        .static_callee(
            candid::Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai")
                .expect("Invalid NNS Governance canister ID"),
        )
        .set_type_selector_config(&nns_type_selector)
        .generate();
}
