use std::{collections::HashMap, cell::RefCell};
use std::borrow::Cow;
use std::iter::FromIterator;

use ic_cdk::{api::{self, call}, export::candid};
use candid::CandidType;
use ic_certified_map::{Hash, RbTree, AsHashTree};
use serde::{Serialize, Deserialize};
use url::Url;
use percent_encoding::percent_decode_str;
use sha2::{Digest, Sha256};
use serde_cbor::Serializer;

use crate::{STATE, MetadataVal, MetadataPurpose};

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[derive(CandidType)]
struct HttpResponse<'a> {
    status_code: u16,
    headers: HashMap<&'a str, Cow<'a, str>>,
    body: Cow<'a, [u8]>,
}

#[export_name = "canister_query http_request"]
fn http_request(/* req: HttpRequest */) /* -> HttpResponse */ {
    ic_cdk::setup();
    let req = call::arg_data::<(HttpRequest,)>().0;
    STATE.with(|state| {
        let state = state.borrow();
        let base_url = if cfg!(mainnet) {
            format!("https://{}.raw.ic0.app", api::id())
        } else {
            format!("http://{}.localhost:8000", api::id())
        };
        let base_url = Url::parse(&base_url).unwrap();
        let url = base_url.join(&req.url).unwrap();
        let full_path = percent_decode_str(url.path()).decode_utf8().unwrap();
        let cert = format!("certificate=:{}:, tree=:{}:", base64::encode(api::data_certificate().unwrap()), witness(&full_path)).into();
        let mut path = url.path_segments().unwrap().map(|segment| percent_decode_str(segment).decode_utf8().unwrap());
        let mut headers = HashMap::from_iter([
            ("Content-Security-Policy", "default-src 'self' ; script-src 'none' ; frame-src 'none' ; object-src 'none'".into()),
            ("IC-Certificate", cert),
        ]);
        if cfg!(mainnet) {
            headers.insert("Strict-Transport-Security", "max-age=31536000; includeSubDomains".into());
        }
        let root = path.next().unwrap_or("".into());
        let body;
        let mut code = 200;
        if root == "" {
            body = format!("Total NFTs: {}", state.nfts.len()).into_bytes().into();
        } else {
            if let Ok(num) = root.parse::<usize>() {
                // /:something
                if let Some(nft) = state.nfts.get(num) {
                    // /:nft
                    let img = path.next().unwrap_or("".into());
                    if img == "" {
                        // /:nft/
                        let part = nft.metadata.iter().find(|x| x.purpose == MetadataPurpose::Rendered).or_else(|| nft.metadata.get(0));
                        if let Some(part) = part {
                            // default metadata: first non-preview metadata, or if there is none, first metadata
                            body = part.data.as_slice().into();
                            if let Some(MetadataVal::TextContent(mime)) = part.key_val_data.get("contentType") {
                                headers.insert("Content-Type", mime.as_str().into());
                            }
                        } else {
                            // no metadata to be found
                            body = b"No metadata for this NFT"[..].into();
                        }
                    } else {
                        // /:nft/:something
                        if let Ok(num) = img.parse::<usize>() {
                            // /:nft/:number
                            if let Some(part) = nft.metadata.get(num) {
                                // /:nft/:id
                                body = part.data.as_slice().into();
                                if let Some(MetadataVal::TextContent(mime)) = part.key_val_data.get("contentType") {
                                    headers.insert("Content-Type", mime.as_str().into());
                                }
                            } else {
                                code = 404;
                                body = b"No such metadata part"[..].into();
                            }
                        } else {
                            code = 400;
                            body = format!("Invalid metadata ID {}", img).into_bytes().into();
                        }
                    }
                } else {
                    code = 404;
                    body = b"No such NFT"[..].into();
                }
            } else {
                code = 400;
                body = format!("Invalid NFT ID {}", root).into_bytes().into();
            }
        }
        call::reply((HttpResponse {
            status_code: code,
            headers,
            body,
        },));
    });
}

thread_local! {
    pub static HASHES: RefCell<RbTree<String, Hash>> = RefCell::default();
}

pub fn add_hash(tkid: u64) {
    crate::STATE.with(|state| HASHES.with(|hashes| {
        let state = state.borrow();
        let mut hashes = hashes.borrow_mut();
        let nft = state.nfts.get(tkid as usize)?;
        let mut default = false;
        for (i, metadata) in nft.metadata.iter().enumerate() {
            let hash = Sha256::digest(&metadata.data);
            hashes.insert(format!("/{}/{}", tkid, i), hash.into());
            if !default && matches!(metadata.purpose, MetadataPurpose::Rendered) {
                default = true;
                hashes.insert(format!("/{}", tkid), hash.into());
            }
        }
        let cert = ic_certified_map::labeled_hash(b"http_assets", &hashes.root_hash());
        api::set_certified_data(&cert);
        Some(())
    }));
}

fn witness(name: &str) -> String {
    HASHES.with(|hashes| {
        let hashes = hashes.borrow();
        let witness = hashes.witness(name.as_bytes());
        let tree = ic_certified_map::labeled(b"http_assets", witness);
        let mut data = vec![];
        let mut serializer = Serializer::new(&mut data);
        serializer.self_describe().unwrap();
        tree.serialize(&mut serializer).unwrap();
        base64::encode(data)
    })
}
