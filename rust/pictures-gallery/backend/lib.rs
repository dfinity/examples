use candid::{CandidType, Deserialize};
use ic_cdk::{
    api::{data_certificate, set_certified_data},
    *,
};
use ic_http_certification::{
    utils::{add_skip_certification_header, skip_certification_certified_data},
    HttpResponse
};
use regex::Regex;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct HeaderField(pub String, pub String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: Vec<u8>,
}

#[init]
fn init() {
    set_certified_data(&skip_certification_certified_data());
}

type ImageId = u64;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ImageData {
    pub id: ImageId,
    pub name: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ImageInfo {
    pub id: ImageId,
    pub name: String,
    pub content_type: String,
}

thread_local! {
    static IMAGES: std::cell::RefCell<HashMap<ImageId, ImageData>> = std::cell::RefCell::new(HashMap::new());
    static NEXT_ID: std::cell::RefCell<ImageId> = const { std::cell::RefCell::new(1) };
}

#[ic_cdk::update]
fn upload_image(name: String, content_type: String, data: Vec<u8>) -> ImageId {
    let id = NEXT_ID.with(|next_id| {
        let mut next_id = next_id.borrow_mut();
        let current_id = *next_id;
        *next_id += 1;
        current_id
    });

    let image = ImageData {
        id,
        name,
        content_type,
        data,
    };

    IMAGES.with(|images| {
        images.borrow_mut().insert(id, image);
    });

    id
}

#[ic_cdk::query]
fn list_images() -> Vec<ImageInfo> {
    IMAGES.with(|images| {
        images
            .borrow()
            .iter()
            .map(|(id, image)| ImageInfo {
                id: *id,
                name: image.name.clone(),
                content_type: image.content_type.clone(),
            })
            .collect()
    })
}

fn create_not_found_response() -> HttpResponse<'static> {
    HttpResponse::not_found(
        Cow::Borrowed(b"Not Found" as &[u8]),
        vec![("Content-Type".to_string(), "text/plain".to_string())],
    )
    .build()
}

fn create_image_reponse(image: ImageData) -> HttpResponse<'static> {
    HttpResponse::ok(
        Cow::Owned(image.data),
        vec![
            ("Content-Type".to_string(), image.content_type.clone()),
            (
                "Cache-Control".to_string(),
                "public, max-age=31536000, immutable".to_string(),
            ),
        ]).build()
}

fn load_image(url: &str) -> Option<ImageData> {

    let re = Regex::new(r"^/image/(\d+)$").unwrap();

    // Try to match and extract the ID
    if let Some(captures) = re.captures(url) {
        // Parse the matched ID as u32
        if let Some(matched_id) = captures.get(1) {
            if let Ok(image_id) = matched_id.as_str().parse::<ImageId>() {
                if let Some(image) = IMAGES.with(|images| images.borrow().get(&image_id).cloned()) {
                    return Some(image);
                }
            }
        }
    }

    None
    
}

#[ic_cdk::query]
fn http_request(request: HttpRequest) -> ic_http_certification::HttpResponse<'static> {

    let mut response = match load_image(&request.url) {
        Some(image) => create_image_reponse(image),
        None => create_not_found_response()
    };

    add_skip_certification_header(data_certificate().unwrap(), &mut response);
    response
}
