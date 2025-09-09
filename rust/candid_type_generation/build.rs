use ic_cdk_bindgen::{Builder, Config};
use std::env;
use std::path::PathBuf;
fn main() {
    let cargo_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Cannot find manifest dir"));
    let nns_candid_file = cargo_dir.join("candid").join("nns_governance.did");
    // Safe to call in single-threaded programs, like this build script...
    unsafe {
        env::set_var(
            "CANISTER_CANDID_PATH_NNS_GOVERNANCE",
            nns_candid_file.as_path().to_str().unwrap(),
        );
        env::set_var("CANISTER_ID_NNS_GOVERNANCE", "rrkah-fqaaa-aaaaa-aaaaq-cai");
    }

    let mut nns_governance = Config::new("nns_governance");
    nns_governance.binding.set_type_attributes(
        "#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]".to_string(),
    );
    let mut builder = Builder::new();
    builder.add(nns_governance);

    builder.build(None); // default write to src/declarations
}
