mod page_not_found_handler;
mod static_page_handler;
mod web_service_handler;

pub use page_not_found_handler::PageNotFoundHandler;
pub use static_page_handler::StaticPageHandler;
pub use web_service_handler::WebServiceHandler;

use std::fs;

use crate::http::{HttpRequest, HttpResponse};

pub trait Handler {
    fn handle(&self, req: &HttpRequest) -> HttpResponse;
    fn load_file(&self, path: &str) -> Option<String> {
        fs::read_to_string(path).ok()
    }
}

pub(crate) fn content_type_for(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}
