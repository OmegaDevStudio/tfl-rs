
use reqwest;
use serde_json::{Value, from_str, from_value};
use crate::datastructs::{DataStruct, QuerySearch};

#[derive(Debug)]
pub enum TflError {
    ApiError(String),
    HttpError(reqwest::Error),
}



pub trait Request {
    fn req(&self) -> reqwest::blocking::Client {
        reqwest::blocking::Client::new()
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    root: String,
    url: Option<String>
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            root: "https://api.tfl.gov.uk/".to_string(),
            url: None
        }
    }

    fn modify_endpoint(mut self, endpoint: &str) -> Self {
        self.url = None;
        self.url = Some(endpoint.to_string());
        return self
    }

    pub fn version(self) -> Self {
        self.modify_endpoint("version")
    }

    pub fn query(self, query: &str) -> Self {
        self.modify_endpoint(&format!("Line/Search/{query}"))
    }

    pub fn fetch(&self) -> Result<DataStruct, TflError> {
        if let Some(url) = &self.url {
            let resp = self.req().get(format!("{}/{}", &self.root, url)).send();
            match resp {
                Ok(resp) => {
                    let text = resp.text();
                  
                    match text {
                        Ok(text) => {
                            // return Ok(from_str(&text).unwrap())
                            if let Ok(real_data) = from_str::<Value>(&text) {
                                let chec: String = real_data["$type"].to_string();
                                println!("{chec}");
                                
                                match chec.as_str() {
                                    r#""Tfl.Api.Presentation.Entities.RouteSearchResponse, Tfl.Api.Presentation.Entities""# => {
                                        let data: Result<QuerySearch, serde_json::Error> = from_value(real_data);
                                        if let Ok(data) = data {
                                            return Ok(DataStruct::from(data))
                                        }
                                    },

                                    _ => return Err(TflError::ApiError(format!("Couldn't deserialize: {real_data}")))
                                }
                            }
                            
                        },
                        Err(e) => return Err(TflError::HttpError(e))
                        
                    }
                },
                Err(e) => return Err(TflError::HttpError(e))
            }
        }
        return Err(TflError::ApiError("Url was not instantiated".to_string()))
    }

}

impl Request for Client {}

