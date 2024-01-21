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




impl Line {
    pub fn convert(&self, data: String) -> Result<LineRoute, Error> {
        let data: Result<LineRoute, Error> = from_str(&data);
        match data {
            Ok(data) => Ok(data),
            Err(e) => Err(e)
        }
    }

    pub fn line(&self) -> &str {
        match self {
            Self::District => "Line/District",
            Self::Circle => "Line/Circle",
            Self::Elizabeth => "Line/Elizabeth",
            Self::Bakerloo => "Line/Bakerloo",
            Self::Central => "Line/Central",
            Self::Jubilee => "Line/Jubilee",
            Self::Metropolitan => "Line/Metropolitan",
            Self::HammersmithCity => "Line/Hammersmith-City",
            Self::Northern => "Line/Northern",
            Self::Piccadilly => "Line/Picadilly",
            Self::Victoria => "Line/Victoria",
            Self::WaterlooCity => "Line/Waterloo-City"
        }
    }
    pub fn route(&self) -> &str {
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

    pub fn stations(&self) -> &str {
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
}
