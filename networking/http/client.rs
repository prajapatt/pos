#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpRequest {
    pub method: &'static str,
    pub path: &'static str,
    pub host: &'static str,
    pub headers: Vec<(&'static str, &'static str)>,
}

impl HttpRequest {
    pub fn new(method: &'static str, path: &'static str, host: &'static str) -> Self {
        Self { method, path, host, headers: Vec::new() }
    }

    pub fn header(mut self, name: &'static str, value: &'static str) -> Self {
        self.headers.push((name, value));
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = format!("{} {} HTTP/1.1\r\nHost: {}\r\n", self.method, self.path, self.host).into_bytes();
        for (name, value) in &self.headers {
            out.extend_from_slice(format!("{}: {}\r\n", name, value).as_bytes());
        }
        out.extend_from_slice(b"\r\n");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::HttpRequest;

    #[test]
    fn http_request_serializes_valid_header_block() {
        let req = HttpRequest::new("GET", "/", "example.com").header("User-Agent", "pos");
        let bytes = req.to_bytes();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.starts_with("GET / HTTP/1.1\r\nHost: example.com\r\n"));
        assert!(text.contains("User-Agent: pos\r\n"));
    }
}
