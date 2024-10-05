pub enum HTTPMethods {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    TRACE,
    PATCH,
}

impl HTTPMethods {
    pub fn from(input: &str) -> Option<Self> {
        match input {
            "GET" => Some(Self::GET),
            "HEAD" => Some(Self::HEAD),
            "POST" => Some(Self::POST),
            "PUT" => Some(Self::PUT),
            "DELETE" => Some(Self::DELETE),
            "CONNECT" => Some(Self::CONNECT),
            "TRACE" => Some(Self::TRACE),
            "PATCH" => Some(Self::PATCH),
            _ => None,
        }
    }
}
