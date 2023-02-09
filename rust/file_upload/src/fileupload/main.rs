use ic_cdk::api::{stable, trap};
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{init, post_upgrade, query, update};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::vec;
use types::{Asset, AssetId, Chunk, ChunkId};
use utils::time;
mod request;
mod routes;
use crate::routes::UrlRouter;
use types::{
    AssetEncoding, ChunkArgs, HttpRequest, HttpResponse, RcBytes, StreamingCallbackHttpResponse,
    StreamingCallbackToken,
};

pub const CHUNK_TTL_MS: u64 = 60000; // 60 seconds Chunk Time to Live

thread_local!(
    static CHUNK_MEM: RefCell<HashMap<ChunkId, Chunk>> = RefCell::new(HashMap::new());
    pub static ASSET_MEM: RefCell<HashMap<AssetId, Asset>> = RefCell::new(HashMap::new());
    static ROUTER: RefCell<UrlRouter> = RefCell::new(UrlRouter::new());
);

#[init]
#[candid_method(init)]
fn init() {
    //Create a router to accept incoming request from /assets/filename
    routes::registry_routes();
}

#[candid_method(query, rename = "http_request")]
#[query(name = "http_request")]
fn http_request(req: HttpRequest) -> HttpResponse {
    ROUTER.with(|url_router| url_router.borrow_mut().handle(req))
}

#[post_upgrade]
fn post_upgrade() {
    routes::registry_routes();
}

#[query(name = "http_request_streaming_callback")]
#[candid_method(query)]
fn http_request_streaming_callback(
    streaming_token: StreamingCallbackToken,
) -> StreamingCallbackHttpResponse {
    request::http_request_streaming_callback(streaming_token).unwrap_or_else(|msg| trap(&msg))
}

#[update(name = "create_chunk")]
#[candid_method(update, rename = "create_chunk")]
fn create_chunk(
    ChunkArgs {
        filename,
        chunk_index,
        chunk,
    }: ChunkArgs,
) -> ChunkId {
    if check_supported_file_extentions(&filename) != true {
        panic!("Unsupported file extension");
    }
    let chunk_id = format!("{}-{}-{}", filename, chunk_index, time::now_millis());
    let chunk_size = chunk.len() as u64;
    CHUNK_MEM.with(|m| {
        m.borrow_mut().insert(
            chunk_id.clone(),
            Chunk {
                bytes: chunk,
                size_bytes: chunk_size,
                ttl: time::now_millis() + CHUNK_TTL_MS,
            },
        );
    });
    chunk_id
}

#[update(name = "commit_batch")]
#[candid_method(update, rename = "commit_batch")]
fn commit_batch(file_name: String, chunk_ids: Vec<String>, content_type: String) -> String {
    let mut content_chunks: Vec<RcBytes> = vec![];
    let mut total_size: u64 = 0;
    chunk_ids.iter().for_each(|chunk_id| {
        CHUNK_MEM.with(|m| {
            if !m.borrow().contains_key(chunk_id) {
                panic!("Chunk not found");
            }
            let x = m.borrow();
            let y = x.get(chunk_id).unwrap();
            total_size += y.size_bytes;
            content_chunks.push(RcBytes::from(y.bytes.clone()));
        });
    });

    let asset_id = format!("{}-{}-{}", file_name, total_size, time::now_millis());

    ASSET_MEM.with(|m| {
        m.borrow_mut().insert(
            asset_id.clone(),
            Asset {
                encoding: AssetEncoding {
                    modified: time::now_millis(),
                    content_chunks: content_chunks,
                    total_length: total_size,
                    certified: false,
                    sha256: None,
                },
                content_type,
            },
        );
    });
    clear_chunks(chunk_ids);
    clear_expired_chunks();
    asset_id
}

fn clear_chunks(chunk_ids: Vec<String>) {
    CHUNK_MEM.with(|m| {
        //Remove Upload chunks from memory
        chunk_ids.iter().for_each(|chunk_id| {
            m.borrow_mut().remove(chunk_id);
        });
    });
}

fn clear_expired_chunks() {
    CHUNK_MEM.with(|m| {
        //Remove all old chunks
        m.borrow_mut()
            .retain(|_, chunk| chunk.ttl > time::now_millis());
    });
}

#[query(name = "list_chunks")]
#[candid_method(query, rename = "list_chunks")]
fn list_chunks() -> Vec<ChunkId> {
    CHUNK_MEM.with(|m| m.borrow().clone().into_keys().collect())
}

#[query(name = "read")]
#[candid_method(query, rename = "read")]
fn read(position: u64, size: u64) -> Vec<u8> {
    let mut buf = [0].repeat(size as usize);
    stable::stable64_read(position, &mut buf);
    return buf;
}

#[query(name = "stablesize")]
#[candid_method(query, rename = "stablesize")]
fn stable_size() -> u64 {
    stable::stable64_size()
}
#[update(name = "stablegrow")]
#[candid_method(update, rename = "stablegrow")]
fn stable_grow(pages: u64) -> u64 {
    stable::stable64_grow(pages).unwrap()
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    ic_cdk::export::candid::export_service!();
    std::print!("{}", __export_service());
}

fn check_supported_file_extentions(filename: &str) -> bool {
    let supported_extensions = vec!["jpg", "jpeg", "png", "gif"];
    let file_extension = Path::new(filename).extension().unwrap().to_str().unwrap();
    supported_extensions.contains(&file_extension)
}
