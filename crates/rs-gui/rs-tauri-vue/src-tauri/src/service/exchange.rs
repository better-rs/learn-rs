pub struct ExchangeService {
    api_key: String,
    api_secret: String,
}

impl ExchangeService {
    pub fn new(key: String, secret: String) -> Self {
        Self { api_key: key, api_secret: secret }
    }
}
