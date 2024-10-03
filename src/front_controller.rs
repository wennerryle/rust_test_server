pub enum HTTPMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Trace,
    Connect,
}

pub struct Request {
    pub method: HTTPMethod,
    pub query: String,
}
