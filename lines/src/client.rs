use async_trait::async_trait;
pub enum TflError {
    ApiError,
    HttpError(reqwest::Error),
}

const ROOT: &str = "https://api.tfl.gov.uk/";

#[async_trait]
pub trait Request {
    fn url(&self) -> &str;

    async fn fetch(&self, req: &reqwest::Client) -> Result<reqwest::Response, TflError> {
        let url = format!("{}/{}", ROOT, &self.url());
        let req = req.get(url).send().await;
        match req {
            Ok(resp) => return Ok(resp),
            Err(e) => return Err(TflError::HttpError(e)),
        };
    }
}

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
