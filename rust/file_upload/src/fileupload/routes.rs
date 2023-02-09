use matchit::Router;
use percent_encoding::percent_decode_str;
use types::{HttpRequest, HttpResponse, UrlParams};

use crate::{request, ROUTER};

// A simple function to registry http route paths in th memory to be used in the http_request,
// it is initialized on init() and on post_upgrade to allow updating the routes after a upgrade
pub(crate) fn registry_routes() {
    ROUTER.with(|url_router| {
        url_router
            .borrow_mut()
            .get("/assets/:id", request::serve_image);

        url_router.borrow_mut().get("/hello", request::hello_world);
    });
}

pub struct UrlRouter {
    router: Router<Box<dyn Fn(HttpRequest, UrlParams) -> HttpResponse>>,
}

impl UrlRouter {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn registry(
        &mut self,
        path: &str,
        _method: String,
        handler: impl Fn(HttpRequest, UrlParams) -> HttpResponse + 'static,
    ) -> &Self {
        if !path.starts_with('/') {
            panic!("expect path beginning with '/', found: '{}'", path);
        }

        match self.router.insert(path, Box::new(handler)) {
            Ok(_) => (),
            Err(err) => panic!("Error registering new route: {}", err),
        }

        self
    }

    pub fn handle(&self, req: HttpRequest) -> HttpResponse {
        let path = get_path(req.url.clone());

        let router = self.router.at(&path);
        let (handler, params) = match router {
            Ok(x) => (x.value, x.params),
            Err(err) => {
                ic_cdk::println!("Handle err: {:?}", err);
                return HttpResponse::not_found();
            }
        };
        //Get Paramos Value from URL and put in a VEC
        let paramsvec: UrlParams = params.iter().map(|x| x.1.to_string()).collect();

        handler(req, paramsvec)
    }

    pub fn get(
        &mut self,
        path: &str,
        handler: impl Fn(HttpRequest, UrlParams) -> HttpResponse + 'static,
    ) -> &Self {
        self.registry(path, "GET".to_string(), handler)
    }
}

fn get_path(url: String) -> String {
    let decoded_url: String;
    if url.find("?canisterId") != None {
        let trimmed: Vec<&str> = url.split("?canisterId").collect();
        decoded_url = percent_decode_str(&trimmed[0])
            .decode_utf8()
            .unwrap()
            .to_string();
    } else if url.find("&canisterId") != None {
        let trimmed: Vec<&str> = url.split("&canisterId").collect();
        decoded_url = percent_decode_str(&trimmed[0])
            .decode_utf8()
            .unwrap()
            .to_string();
    } else {
        decoded_url = percent_decode_str(&url).decode_utf8().unwrap().to_string();
    }

    decoded_url
}
