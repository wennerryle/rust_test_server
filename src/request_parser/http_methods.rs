use nom::error::ErrorKind;

#[derive(Debug, PartialEq)]
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
    pub fn from(input: &str) -> Result<Self, ErrorKind> {
        match input {
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(ErrorKind::Tag),
        }
    }
}
