use crate::client::Request;
pub enum Mode {
    Bus,
    CableCar,
    Coach,
    Cycle,
    CycleHire,
    DLR,
    ElizabethLine,
    InterchangeKeepSitting,
    InterchangeSecure,
    NationalRail,
    Overground,
    ReplacementBus,
    RiverBus,
    RiverTour,
    Tram,
    Taxi,
    Tube,
    Walking,
}

impl Request for Mode {
    fn url(&self) -> &str {
        // Gathers all Routes for a particular Mode
        match self {
            Self::Bus => "Line/Mode/bus/Route",
            Self::CableCar => "Line/Mode/cable-car/Route",
            Self::Coach => "Line/Mode/coach/Route",
            Self::Cycle => "Line/Mode/cycle/Route",
            Self::CycleHire => "Line/Mode/cycle-hire/Route",
            Self::DLR => "Line/Mode/dlr/Route",
            Self::ElizabethLine => "Line/Mode/elizabeth-line/Route",
            Self::InterchangeKeepSitting => "Line/Mode/interchange-keep-sitting/Route",
            Self::InterchangeSecure => "Line/Mode/interchange-secure/Route",
            Self::NationalRail => "Line/Mode/national-rail/Route",
            Self::Overground => "Line/Mode/overground/Route",
            Self::ReplacementBus => "Line/Mode/replacement-bus/Route",
            Self::RiverBus => "Line/Mode/river-bus/Route",
            Self::RiverTour => "Line/Mode/river-tour/Route",
            Self::Tram => "Line/Mode/tram/Route",
            Self::Taxi => "Line/Mode/taxi/Route",
            Self::Tube => "Line/Mode/tube/Route",
            Self::Walking => "Line/Mode/Walking/Route",
        }
    }
}
