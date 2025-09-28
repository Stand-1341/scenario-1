use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    UNKNOWN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    Http10,
    Http11,
    Http20,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<&str> for HttpRequest {
    fn from(req: &str) -> Self {
        let (head, body) = split_head_body(req);
        let mut lines = head.lines();
        let req_line = lines.next().unwrap_or_default();
        let mut parts = req_line.split_whitespace();
        let method = parse_method(parts.next().unwrap_or_default());
        let path = parts.next().unwrap_or("/").to_string();
        let version = parse_version(parts.next().unwrap_or("HTTP/1.1"));
        let mut headers = HashMap::new();
        for line in lines {
            if line.is_empty() {
                continue;
            }
            if let Some((k, v)) = line.split_once(':') {
                headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
            }
        }
        HttpRequest {
            method,
            version,
            resource: Resource::Path(path),
            headers,
            msg_body: body.to_string(),
        }
    }
}

impl HttpRequest {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .get(&name.to_ascii_lowercase())
            .map(|s| s.as_str())
    }
    pub fn path(&self) -> &str {
        match &self.resource {
            Resource::Path(s) => s.as_str(),
        }
    }
}

fn split_head_body(s: &str) -> (&str, &str) {
    if let Some(i) = s.find("\r\n\r\n") {
        (&s[..i], &s[i + 4..])
    } else if let Some(i) = s.find("\n\n") {
        (&s[..i], &s[i + 2..])
    } else {
        (s, "")
    }
}

fn parse_method(s: &str) -> Method {
    match s {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        "PATCH" => Method::PATCH,
        _ => Method::UNKNOWN,
    }
}

fn parse_version(s: &str) -> Version {
    match s {
        "HTTP/1.0" => Version::Http10,
        "HTTP/1.1" => Version::Http11,
        "HTTP/2.0" | "HTTP/2" => Version::Http20,
        _ => Version::Unknown,
    }
}
