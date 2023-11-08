use crate::client::{Request, TflError};
use crate::datastructs::{LineRoute, StopPoint};
use serde_json::{Value, from_str, Error};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Line {
    District,
    Circle,
    Elizabeth,
    Bakerloo,
    Central,
    Jubilee,
    Metropolitan,
    HammersmithCity,
    Northern,
    Piccadilly,
    Victoria,
    WaterlooCity,
}

const ROOT: &str = "https://api.tfl.gov.uk/";

impl Request for Line {
    fn url(&self) ->  &str {
        // Gathers Route data regarding a line
        match self {
            Self::District => "Line/District/Route",
            Self::Circle => "Line/Cirlce/Route",
            Self::Elizabeth => "Line/Elizabeth/Route",
            Self::Bakerloo => "Line/Bakerloo/Route",
            Self::Central => "Line/Central/Route",
            Self::Jubilee => "Line/Jubilee/Route",
            Self::Metropolitan => "Line/Metropolitan/Route",
            Self::HammersmithCity => "Line/Hammersmith-City/Route",
            Self::Northern => "Line/Northern/Route",
            Self::Piccadilly => "Line/Piccadilly/Route",
            Self::Victoria => "Line/Victoria/Route",
            Self::WaterlooCity => "Line/Waterloo-City/Route"
        }
    }
}

impl Line {
    pub fn convert(&self, data: String) -> Result<LineRoute, Error> {
        let data: Result<LineRoute, Error> = from_str(&data);
        match data {
            Ok(data) => Ok(data),
            Err(e) => Err(e)
        }
    }

    fn stations_url(&self) -> &str {
        match self {
            Self::District => "Line/District/StopPoints",
            Self::Circle=> "Line/Circle/StopPoints",
            Self::Bakerloo => "Line/Bakerloo/StopPoints",
            Self::Elizabeth => "Line/Bakerloo/StopPoints",
            Self::HammersmithCity => "Line/Hammersmith-City/StopPoints",
            Self::Central => "Line/Central/StopPoints",
            Self::Jubilee => "Line/Jubilee/StopPoints",
            Self::Metropolitan => "Line/Metropolitan/StopPoints",
            Self::Northern => "Line/Northern/StopPoints",
            Self::Piccadilly => "Line/Picadilly/StopPoints",
            Self::Victoria => "Line/Victoria/StopPoints",
            Self::WaterlooCity => "Line/Waterloo-City/StopPoints"
        }
    }

    pub async fn stations(&self) -> Result<Vec<StopPoint>, TflError> {
        let url = format!("{}/{}", ROOT, &self.stations_url());
        let req = self.req().get(url).send().await;
        match req {
            Ok(req) => {
                match req.text().await {
                    Ok(text) => {
                        let data: Vec<StopPoint> = from_str(&text).expect("Query Search");
                        return Ok(data)
                    },
                    Err(e) => return Err(TflError::HttpError(e))
                }
            }
            Err(e) => return Err(TflError::HttpError(e))
        }
    }
}
