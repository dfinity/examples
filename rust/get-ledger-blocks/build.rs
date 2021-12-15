use prost_build::Config;

fn main() {
    println!("cargo:rerun-if-changed=protos/ledger.proto");
    std::fs::create_dir_all("src/gen").unwrap();
    Config::new()
        .out_dir("src/gen")
        .default_package_filename("ledger")
        .compile_protos(&["protos/ledger.proto"], &["protos"])
        .unwrap();
}