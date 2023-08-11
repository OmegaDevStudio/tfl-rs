pub struct Client<S: AsRef<str>> {
    api_key: S,
    req: reqwest::Client,
}

impl<S: AsRef<str>> Client<S> {
    pub fn new(api_key: S) -> Self {
        Self {
            api_key,
            req: reqwest::Client::new(),
        }
    }
}
