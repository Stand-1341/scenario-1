use std::path::{Component, PathBuf};

use crate::handlers::{content_type_for, Handler};
use crate::http::Resource;
use crate::http::{HttpRequest, HttpResponse};

pub struct StaticPageHandler {
    root: PathBuf,
}

impl StaticPageHandler {
    pub fn new(root: String) -> Self {
        Self {
            root: PathBuf::from(root),
        }
    }

    fn map_path(&self, req: &HttpRequest) -> PathBuf {
        let p = match &req.resource {
            Resource::Path(s) => s.as_str(),
        };
        if p == "/" {
            return self.root.join("index.html");
        }
        if p == "/health" {
            return self.root.join("health.html");
        }
        let clean = p.trim_start_matches('/');
        self.root.join(clean)
    }
}

impl Handler for StaticPageHandler {
    fn handle(&self, req: &HttpRequest) -> HttpResponse {
        let p = self.map_path(req);
        if p.components().any(|c| matches!(c, Component::ParentDir)) {
            return HttpResponse::bad_request("parent dirs not allowed");
        }
        if let Some(path) = p.to_str() {
            if let Some(txt) = self.load_file(path) {
                let ct = content_type_for(path);
                return HttpResponse::ok(txt, ct);
            }
        }
        let fallback = self.root.join("404.html");
        if let Some(path) = fallback.to_str() {
            if let Some(html) = self.load_file(path) {
                return HttpResponse::not_found(html, "text/html; charset=utf-8");
            }
        }
        HttpResponse::not_found("404 not found".to_string(), "text/plain; charset=utf-8")
    }
}
