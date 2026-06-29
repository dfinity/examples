use ic_cdk_bindgen::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let cargo_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Cannot find manifest dir"));

    let nns_candid_file = cargo_dir.join("..").join("candid").join("nns_governance.did");
    let nns_type_selector = cargo_dir.join("..").join("candid").join("nns_governance.toml");

    if !nns_candid_file.exists() {
        panic!(
            "candid/nns_governance.did not found. \
            Run scripts/fetch_candid.sh to fetch it from the live canister."
        );
    }

    // Only re-run this build script when the input files change.
    // Without these hints, Cargo re-runs the script on every build.
    println!(
        "cargo:rerun-if-changed={}",
        nns_candid_file.display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        nns_type_selector.display()
    );
    println!("cargo:rerun-if-changed=build.rs");

    // Generate Rust types from the Candid interface definition.
    //
    // - Config::new(name, path): "name" becomes the output filename ($OUT_DIR/nns_governance.rs)
    // - static_callee(principal): embeds CANISTER_ID as a constant in the generated code.
    //   Use dynamic_callee("ENV_VAR") if the target canister ID varies per environment.
    // - set_type_selector_config(path): TOML file controlling derived traits on generated types.
    // - generate(): writes the output to $OUT_DIR, where it is included via include! in
    //   src/declarations/mod.rs.
    Config::new("nns_governance", &nns_candid_file)
        .static_callee(
            candid::Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai")
                .expect("Invalid NNS Governance canister ID"),
        )
        .set_type_selector_config(&nns_type_selector)
        .generate();
}
