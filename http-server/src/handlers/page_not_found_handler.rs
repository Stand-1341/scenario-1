use crate::handlers::Handler;
use crate::http::{HttpRequest, HttpResponse};

#[derive(Default)]
pub struct PageNotFoundHandler;

impl Handler for PageNotFoundHandler {
    fn handle(&self, _req: &HttpRequest) -> HttpResponse {
        HttpResponse::not_found("404 not found", "text/plain; charset=utf-8")
    }
}
