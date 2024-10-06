pub struct HTTPCode {
    code: u16,
}

impl HTTPCode {
    fn new(code: u16) -> Self {
        HTTPCode { code }
    }

    pub fn from(input: u16) -> Option<HTTPCode> {
        match input {
            100..=103
            | 200..=208
            | 226
            | 300..=308
            | 400..=419
            | 421..=429
            | 431
            | 449
            | 451
            | 499
            | 500..=511
            | 520..=526 => Some(HTTPCode { code: input }),
            _ => None,
        }
    }

    pub fn code(&self) -> u16 {
        self.code
    }
}
