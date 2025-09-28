use std::fs;
use std::path::PathBuf;

use crate::handlers::Handler;
use crate::http::{HttpRequest, HttpResponse, Method};

pub struct WebServiceHandler {
    data_dir: PathBuf,
}

impl WebServiceHandler {
    pub fn new(data_dir: String) -> Self {
        Self {
            data_dir: PathBuf::from(data_dir),
        }
    }
}

impl Handler for WebServiceHandler {
    fn handle(&self, req: &HttpRequest) -> HttpResponse {
        let not_found = "{\"error\":\"not found\"}";
        let missing = "{\"error\":\"orders.json missing\"}";
        if !matches!(req.method, Method::GET) {
            return HttpResponse::not_found(not_found, "application/json; charset=utf-8");
        }
        if req.path() != "/api/shipping/orders" {
            return HttpResponse::not_found(not_found, "application/json; charset=utf-8");
        }
        let file = self.data_dir.join("orders.json");
        match fs::read_to_string(&file) {
            Ok(txt) => HttpResponse::ok(txt, "application/json; charset=utf-8"),
            Err(_) => HttpResponse::not_found(missing, "application/json; charset=utf-8"),
        }
    }
}
