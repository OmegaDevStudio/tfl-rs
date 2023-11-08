use async_trait::async_trait;
use reqwest;
use serde_json::{Value, from_str};
use crate::datastructs::QuerySearch;

#[derive(Debug)]
pub enum TflError {
    ApiError(String),
    HttpError(reqwest::Error),
}

const ROOT: &str = "https://api.tfl.gov.uk/";

#[async_trait]
pub trait Request {
    fn url(&self) -> &str;

    fn req(&self) -> reqwest::Client {
        reqwest::Client::new()
    }

    async fn fetch(&self) -> Result<String, TflError> {
        let req = self.req();
        let url = format!("{}/{}", ROOT, &self.url());
        let req = req.get(url).send().await;
        if let Ok(req) = req {
            if let Ok(text) = req.text().await { return Ok(text) }
        }
        Err(TflError::ApiError("I suck".to_string()))
    }
}

#[derive(Debug, Clone)]
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

    pub fn url(&self) -> &str {
        "version"
    }

    pub fn get_client(&self) -> &reqwest::Client {
        &self.req
    } 

    pub async fn version(&self) -> Result<String, TflError> {
        let url = format!("{}/{}", ROOT, &self.url());
        let resp = self.req.get(url).send().await;
        match resp {
            Ok(resp) => return Ok(resp.text().await.unwrap()),
            Err(e) => return Err(TflError::HttpError(e))
        }
    }

    pub async fn query(&self, query: &str) -> Result<QuerySearch, TflError> {
        let url = format!("{}/{}/{}", ROOT, "Line/Search/", query);
        let req = self.req.get(url).send().await;
        if let Ok(req) = req {
            if let Ok(text) = req.text().await {
                let data: QuerySearch = from_str(&text).expect("Query Search");
                return Ok(data)
            }
        }
        Err(TflError::ApiError("I suck".to_string()))
    }

}

