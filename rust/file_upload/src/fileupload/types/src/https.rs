use ic_cdk::export::candid::{CandidType, Deserialize, Func};
use serde_bytes::ByteBuf;

use crate::{rc_bytes::RcBytes, TimestampMillis};

pub type HeaderField = (String, String);
pub type Filename = String;
pub type ChunkId = String;
pub type AssetId = String;
#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: RcBytes,
    pub upgrade: Option<bool>,
    pub streaming_strategy: Option<StreamingStrategy>,
}

impl HttpResponse {
    pub fn not_found() -> Self {
        HttpResponse {
            status_code: 404,
            headers: vec![],
            body: RcBytes::from(ByteBuf::from("not found")),
            streaming_strategy: None,
            upgrade: None,
        }
    }
}

//Params ans delimited by / /, exemple: /assets/data/file.png, Assets,Data and file can be params

pub type UrlParams = Vec<String>;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StreamingCallbackToken {
    pub key: String,
    pub content_encoding: String,
    pub index: usize,
    // We don't care about the sha, we just want to be backward compatible.
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum StreamingStrategy {
    Callback {
        callback: Func,
        token: StreamingCallbackToken,
    },
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StreamingCallbackHttpResponse {
    pub body: RcBytes,
    pub token: Option<StreamingCallbackToken>,
}

#[derive(Deserialize, Debug, CandidType, Clone)]
pub struct ChunkArgs {
    pub filename: Filename,
    pub chunk_index: u64,
    pub chunk: ByteBuf,
}

#[derive(Deserialize, Debug, CandidType, Clone)]
pub struct Chunk {
    pub bytes: ByteBuf,
    pub size_bytes: u64,
    pub ttl: TimestampMillis,
}

#[derive(Default, Clone, Debug, CandidType, Deserialize)]
pub struct AssetEncoding {
    pub modified: TimestampMillis,
    pub content_chunks: Vec<RcBytes>,
    pub total_length: u64,
    pub certified: bool,
    pub sha256: Option<[u8; 32]>,
}

#[derive(Deserialize, Debug, CandidType, Clone)]
pub struct Asset {
    pub encoding: AssetEncoding,
    pub content_type: String,
}
