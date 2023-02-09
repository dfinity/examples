use crate::{AssetEncoding, ASSET_MEM};

use ic_cdk::export::candid;

use types::{
    HttpRequest, HttpResponse, RcBytes, StreamingCallbackHttpResponse, StreamingCallbackToken,
    StreamingStrategy, UrlParams,
};

pub fn serve_image(_request: HttpRequest, params: UrlParams) -> HttpResponse {
    let filename = &params[0];
    let mut res_headers = vec![
        ("accept-ranges".to_string(), "bytes".to_string()),
        (
            "cache-control".to_string(),
            "private, max-age=0".to_string(),
        ),
    ];
    ASSET_MEM.with(|m| {
        match m.borrow_mut().get(filename) {
            Some(img) => {
                res_headers.push(("Content-Type".to_string(), img.content_type.clone()));
                res_headers.push((
                    "Content-Length".to_string(),
                    img.encoding.total_length.to_string(),
                ));

                let n_chunks = img.encoding.content_chunks.len() as u64;
                if n_chunks == 1 {
                    return HttpResponse {
                        status_code: 200,
                        headers: res_headers,
                        body: img.encoding.content_chunks[0].clone(),
                        streaming_strategy: None,
                        upgrade: Some(false),
                    };
                } else {
                    let streaming_strategy = create_token(&img.encoding, &filename, 0 as usize)
                        .map(|token| StreamingStrategy::Callback {
                            callback: candid::Func {
                                method: "http_request_streaming_callback".to_string(),
                                principal: ic_cdk::id(),
                            },
                            token: token,
                        });

                    return HttpResponse {
                        status_code: 200,
                        headers: res_headers,
                        body: img.encoding.content_chunks[0].clone(),
                        streaming_strategy: streaming_strategy,
                        upgrade: Some(false),
                    };
                }
            }
            None => {
                return HttpResponse::not_found();
            }
        };
    })
}

pub fn hello_world(_request: HttpRequest, _params: UrlParams) -> HttpResponse {
    return HttpResponse {
        status_code: 200,
        headers: vec![],
        body: RcBytes::from(serde_bytes::ByteBuf::from("Hello")),
        streaming_strategy: None,
        upgrade: Some(false),
    };
}

fn create_token(
    enc: &AssetEncoding,
    key: &str,
    chunk_index: usize,
) -> Option<StreamingCallbackToken> {
    if chunk_index + 1 >= enc.content_chunks.len() {
        None
    } else {
        Some(StreamingCallbackToken {
            key: key.to_string(),
            content_encoding: "gzip".to_string(),
            index: chunk_index + 1,
        })
    }
}

pub fn http_request_streaming_callback(
    StreamingCallbackToken {
        key,
        content_encoding: _,
        index,
    }: StreamingCallbackToken,
) -> Result<StreamingCallbackHttpResponse, String> {
    ASSET_MEM.with(|m| {
        let img = m
            .borrow()
            .get(&key)
            .cloned()
            .ok_or_else(|| "Invalid token on streaming: key not found.".to_string())?;

        // MAX is good enough. This means a chunk would be above 64-bits, which is impossible...

        Ok(StreamingCallbackHttpResponse {
            body: img.encoding.content_chunks[index].clone(),
            token: create_token(&img.encoding, &key, index),
        })
    })
}
