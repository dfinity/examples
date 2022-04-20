use sha2::Digest;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub enum ContentEncoding {
    Identity,
    GZip,
}

#[derive(Debug)]
pub enum ContentType {
    HTML,
    JS,
    ICO,
    WEBP,
    SVG,
}

fn hash_file(path: &str) -> [u8; 32] {
    let bytes = fs::read(path).unwrap_or_else(|e| panic!("failed to read file {}: {}", path, e));
    let mut hasher = sha2::Sha256::new();
    hasher.update(&bytes);
    hasher.finalize().into()
}

fn main() -> Result<(), String> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let assets_module_path = Path::new(&out_dir).join("assets.rs");
    let asset_rel_paths = [
        (
            "/",
            "../../dist/index.html",
            ContentEncoding::Identity,
            ContentType::HTML,
        ),
        // The FAQ and about pages are the same webapp, but the webapp routes to the correct page
        (
            "/faq",
            "../../dist/index.html",
            ContentEncoding::Identity,
            ContentType::HTML,
        ),
        (
            "/about",
            "../../dist/index.html",
            ContentEncoding::Identity,
            ContentType::HTML,
        ),
        (
            "/index.html",
            "../../dist/index.html",
            ContentEncoding::Identity,
            ContentType::HTML,
        ),
        (
            "/index.js",
            "../../dist/index.js.gz",
            ContentEncoding::GZip,
            ContentType::JS,
        ),
        (
            "/loader.webp",
            "../../dist/loader.webp",
            ContentEncoding::Identity,
            ContentType::WEBP,
        ),
        (
            "/favicon.ico",
            "../../dist/favicon.ico",
            ContentEncoding::Identity,
            ContentType::ICO,
        ),
        (
            "/ic-badge.svg",
            "../../dist/ic-badge.svg",
            ContentEncoding::Identity,
            ContentType::SVG,
        ),
    ];

    for (_, path, _, _) in asset_rel_paths.iter() {
        if !Path::new(path).exists() {
            return Err(format!("asset file {} doesn't exist", path));
        }
    }

    let mut assets_module = fs::File::create(&assets_module_path).map_err(|e| {
        format!(
            "failed to create file {}: {}",
            assets_module_path.display(),
            e
        )
    })?;
    writeln!(
        assets_module,
        r#"
#[derive(Debug, PartialEq, Eq)]
pub enum ContentEncoding {{
    Identity,
    GZip,
}}

#[derive(Debug, PartialEq, Eq)]
pub enum ContentType {{
    HTML,
    JS,
    ICO,
    WEBP,
    SVG
}}

pub fn for_each_asset(mut f: impl FnMut(&'static str, ContentEncoding, ContentType, &'static [u8], &[u8; 32])) {{
"#
    )
    .unwrap();

    for (name, path, encoding, content_type) in asset_rel_paths.iter() {
        let hash = hash_file(path);
        let abs_path = Path::new(path).canonicalize().unwrap();
        writeln!(
            assets_module,
            "  f(\"{}\", ContentEncoding::{:?}, ContentType::{:?}, &include_bytes!(\"{}\")[..], &{:?});",
            name,
            encoding,
            content_type,
            abs_path.display(),
            hash
        )
        .unwrap();
    }
    writeln!(assets_module, "}}").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    for (_, path, _, _) in asset_rel_paths.iter() {
        println!("cargo:rerun-if-changed={}", path);
    }

    Ok(())
}
