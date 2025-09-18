use std::borrow::Cow;
use std::iter::FromIterator;
use std::{cell::RefCell, collections::HashMap};

use candid::CandidType;
use ic_cdk::{
    api::{self, call},
    export::candid,
};
use ic_certified_map::{AsHashTree, Hash, RbTree};
use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};
use serde_cbor::Serializer;
use sha2::{Digest, Sha256};

use crate::{MetadataPurpose, MetadataVal, STATE};

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

// This could reply with a lot of data. To return this data from the function would require it to be cloned,
// because the thread_local! closure prevents us from returning data borrowed from inside it.
// Luckily, it doesn't actually get returned from the exported WASM function, that's just an abstraction. 
// What happens is it gets fed to call::reply, and we can do that explicitly to save the cost of cloning the data.
// #[query] calls call::reply unconditionally, and calling it twice would trap, so we use #[export_name] directly.
// This requires duplicating the rest of the abstraction #[query] provides for us, like setting up the panic handler with
// ic_cdk::setup() and fetching the function parameters via call::arg_data.
// cdk 0.5 makes this unnecessary, but it has not been released at the time of writing this example.
#[export_name = "canister_query http_request"]
fn http_request(/* req: HttpRequest */) /* -> HttpResponse */
{
    ic_cdk::setup();
    let req = call::arg_data::<(HttpRequest,)>().0;
    STATE.with(|state| {
        let state = state.borrow();
        let url = req.url.split('?').next().unwrap_or("/");
        let cert = format!(
            "certificate=:{}:, tree=:{}:",
            base64::encode(api::data_certificate().unwrap()),
            witness(&url)
        )
        .into();
        let mut path = url[1..].split('/')
            .map(|segment| percent_decode_str(segment).decode_utf8().unwrap());
        let mut headers = HashMap::from_iter([
            (
                "Content-Security-Policy",
                "default-src 'self' ; script-src 'none' ; frame-src 'none' ; object-src 'none'"
                    .into(),
            ),
            ("IC-Certificate", cert),
        ]);
        if cfg!(mainnet) {
            headers.insert(
                "Strict-Transport-Security",
                "max-age=31536000; includeSubDomains".into(),
            );
        }
        let root = path.next().unwrap_or_else(|| "".into());
        let body;
        let mut code = 200;
        if root == "" {
            body = format!("Total NFTs: {}", state.nfts.len())
                .into_bytes()
                .into();
        } else {
            if let Ok(num) = root.parse::<usize>() {
                // /:something
                if let Some(nft) = state.nfts.get(num) {
                    // /:nft
                    let img = path.next().unwrap_or_else(|| "".into());
                    if img == "" {
                        // /:nft/
                        let part = nft
                            .metadata
                            .iter()
                            .find(|x| x.purpose == MetadataPurpose::Rendered)
                            .or_else(|| nft.metadata.get(0));
                        if let Some(part) = part {
                            // default metadata: first non-preview metadata, or if there is none, first metadata
                            body = part.data.as_slice().into();
                            if let Some(MetadataVal::TextContent(mime)) =
                                part.key_val_data.get("contentType")
                            {
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
                                if let Some(MetadataVal::TextContent(mime)) =
                                    part.key_val_data.get("contentType")
                                {
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
    // sha256("Total NFTs: 0") = 83d0f670865c367ce95f595959abec46ed7b64033ecee9ed772e78793f3bc10f
    pub static HASHES: RefCell<RbTree<String, Hash>> = RefCell::new(RbTree::from_iter([("/".to_string(), *b"\x83\xd0\xf6\x70\x86\x5c\x36\x7c\xe9\x5f\x59\x59\x59\xab\xec\x46\xed\x7b\x64\x03\x3e\xce\xe9\xed\x77\x2e\x78\x79\x3f\x3b\xc1\x0f")]));
}

pub fn add_hash(tkid: u64) {
    crate::STATE.with(|state| {
        HASHES.with(|hashes| {
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
            hashes.insert(
                "/".to_string(),
                Sha256::digest(format!("Total NFTs: {}", state.nfts.len())).into(),
            );
            let cert = ic_certified_map::labeled_hash(b"http_assets", &hashes.root_hash());
            api::set_certified_data(&cert);
            Some(())
        })
    });
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
