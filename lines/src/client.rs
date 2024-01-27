
use reqwest;
use serde_json::{Value, from_str, from_value};
use crate::datastructs::{DataStruct, LineRoute, QuerySearch, Response, StopPoint, Version};
use crate::lines::Line;

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
        self
    }

    pub fn version(self) -> Self {
        self.modify_endpoint("version")
    }

    pub fn query(self, query: &str) -> Self {
        self.modify_endpoint(&format!("Line/Search/{query}"))
    }

    pub fn route(self, line_route: &str) -> Self {
        self.modify_endpoint(line_route)
    }

    pub fn stations(self, line: &str) -> Self {
        self.modify_endpoint(line)
    }

    pub fn line(self, line: &str) -> Self {
        self.modify_endpoint(line)
    }

    pub fn fetch(&self) -> Result<Response, TflError> {
        if let Some(url) = &self.url {
            let resp = self.req().get(format!("{}/{}", &self.root, url)).send();
            match resp {
                Ok(resp) => {
                    let text = resp.text();
                  
                    match text {
                        Ok(text) => {
                            // return Ok(fro"Tfl.Api.Common.ApiVersionInfo, Tfl.Api.Commonm_str(&text).unwrap())
                            if let Ok(real_data) = from_str::<Value>(&text) {

                                if let Value::Array(val) = &real_data {
                                    match &val[0]["$type"] {
                                        Value::String(v) => {
                                            match v.as_str() {
                                                "Tfl.Api.Presentation.Entities.Line, Tfl.Api.Presentation.Entities" => {
                                                    let data = val.into_iter().map(|v| if let Ok(data) = from_value::<LineRoute>(v.to_owned()) {
                                                        DataStruct::from(data)
                                                    } else {
                                                        DataStruct::from(LineRoute::default())
                                                    }).collect();
                                                    return Ok(Response::Multiple(data))
                                                },

                                                "Tfl.Api.Presentation.Entities.StopPoint, Tfl.Api.Presentation.Entities" => {
                                                    let data = val.into_iter().map(|v| if let Ok(data) = from_value::<StopPoint>(v.to_owned()) {
                                                        DataStruct::from(data)
                                                    } else {
                                                        DataStruct::from(StopPoint::default())
                                                    }).collect();
                                                    return Ok(Response::Multiple(data))
                                                }
                                                _ => ()
                                            }
                                        },
                                        _ => ()
                                    }
                                    
                    
                                
                                }

                                match &real_data["$type"] {
                                
                                    Value::String(v) => {
                                        match v.as_str() {
                                            "Tfl.Api.Presentation.Entities.RouteSearchResponse, Tfl.Api.Presentation.Entities" => {
                                                let data: Result<QuerySearch, serde_json::Error> = from_value(real_data);
                                                if let Ok(data) = data {
                                                    return Ok(Response::Single(DataStruct::from(data)))
                                                }
                                            },
                                            "Tfl.Api.Common.ApiVersionInfo, Tfl.Api.Common" => {
                                                let data: Result<Version, serde_json::Error> = from_value(real_data);
                                                if let Ok(data) = data {
                                                    return Ok(Response::Single(DataStruct::from(data)))
                                                }
                                            },
                                            "Tfl.Api.Presentation.Entities.Line, Tfl.Api.Presentation.Entities" => {
                                                let data: Result<LineRoute, serde_json::Error> = from_value(real_data.clone());
                                                if let Ok(data) = data {
                                                    return Ok(Response::Single(DataStruct::from(data)))
                                                }
                                            }
                                            _ => return Err(TflError::ApiError(format!("Couldn't deserialize: {real_data:#?}")))
                                        }
                                    
                                    }

                                    _ => return Err(TflError::ApiError(format!("Couldn't deserialize: {real_data:#?}")))
                                }
                            }
                        },
                        Err(e) => return Err(TflError::HttpError(e))
                    }
                },
                Err(e) => return Err(TflError::HttpError(e))
            }
        }
        Err(TflError::ApiError("Url was not instantiated".to_string()))
    }

}

impl Request for Client {}

