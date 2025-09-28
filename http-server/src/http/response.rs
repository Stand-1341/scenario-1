use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn new(
        status_code: u16,
        status_text: impl Into<String>,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> Self {
        Self {
            status_code,
            status_text: status_text.into(),
            headers,
            body,
        }
    }
    pub fn ok(body: impl Into<String>, content_type: &str) -> Self {
        let mut h = HashMap::new();
        h.insert("Content-Type".into(), content_type.into());
        let b = body.into();
        h.insert("Content-Length".into(), b.as_bytes().len().to_string());
        Self::new(200, "OK", h, Some(b))
    }
    pub fn not_found(body: impl Into<String>, content_type: &str) -> Self {
        let mut h = HashMap::new();
        h.insert("Content-Type".into(), content_type.into());
        let b = body.into();
        h.insert("Content-Length".into(), b.as_bytes().len().to_string());
        Self::new(404, "Not Found", h, Some(b))
    }
    pub fn bad_request(body: impl Into<String>) -> Self {
        let mut h = HashMap::new();
        let b = body.into();
        h.insert("Content-Type".into(), "text/plain; charset=utf-8".into());
        h.insert("Content-Length".into(), b.as_bytes().len().to_string());
        Self::new(400, "Bad Request", h, Some(b))
    }
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code, self.status_text
        ));
        for (k, v) in &self.headers {
            out.push_str(k);
            out.push_str(": ");
            out.push_str(v);
            out.push_str("\r\n");
        }
        out.push_str("Connection: close\r\n");
        out.push_str("\r\n");
        if let Some(b) = &self.body {
            out.push_str(b);
        }
        out
    }
}
