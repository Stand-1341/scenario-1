use crate::handlers::Handler;
use crate::handlers::{PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use crate::http::{HttpRequest, HttpResponse, Method, Version};

pub struct Router {
    static_handler: StaticPageHandler,
    web_service_handler: WebServiceHandler,
    page_not_found_handler: PageNotFoundHandler,
}

impl Router {
    pub fn new(public_dir: String, data_dir: String) -> Self {
        Self {
            static_handler: StaticPageHandler::new(public_dir),
            web_service_handler: WebServiceHandler::new(data_dir),
            page_not_found_handler: PageNotFoundHandler::default(),
        }
    }

    pub fn route(&self, req: &HttpRequest) -> HttpResponse {
        if matches!(req.version, Version::Unknown) {
            return HttpResponse::bad_request("unsupported http version");
        }
        if req.path().starts_with("/api/") {
            return self.web_service_handler.handle(req);
        }
        if matches!(req.method, Method::GET) {
            return self.static_handler.handle(req);
        }
        self.page_not_found_handler.handle(req)
    }
}
