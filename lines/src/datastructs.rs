use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[enum_dispatch]
pub trait JsonTrait {
    fn some(&self) -> &str { 
        "foo"
    }
}

// Line/Search/<query>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySearch {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub input: String,
    pub search_matches: Vec<SearchMatch>,
}

// Version
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub label: String,
    pub timestamp: String,
    pub version: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchMatch {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub line_id: String,
    pub mode: String,
    pub line_name: String,
    pub line_route_section: Vec<LineRouteSection>,
    pub matched_route_sections: Vec<Value>,
    pub matched_stops: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineRouteSection {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub route_id: i64,
    pub direction: String,
    pub destination: String,
    pub from_station: String,
    pub to_station: String,
    pub service_type: String,
    pub vehicle_destination_text: String,
}

// Line/<line>/Route
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineRoute {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub id: String,
    pub name: String,
    pub mode_name: String,
    pub disruptions: Vec<Value>,
    pub created: String,
    pub modified: String,
    pub line_statuses: Vec<Value>,
    pub route_sections: Vec<RouteSection>,
    pub service_types: Vec<ServiceType>,
    pub crowding: Crowding,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteSection {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub name: String,
    pub direction: String,
    pub origination_name: String,
    pub destination_name: String,
    pub originator: String,
    pub destination: String,
    pub service_type: String,
    pub valid_to: String,
    pub valid_from: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceType {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub name: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crowding {
    #[serde(rename = "$type")]
    pub type_field: String,
}

// Line/<line>/StopPoints
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPoint {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub naptan_id: String,
    pub modes: Vec<String>,
    pub ics_code: String,
    pub stop_type: String,
    pub station_naptan: String,
    pub lines: Vec<Line>,
    pub line_group: Vec<LineGroup>,
    pub line_mode_groups: Vec<LineModeGroup>,
    pub status: bool,
    pub id: String,
    pub common_name: String,
    pub place_type: String,
    pub additional_properties: Vec<AdditionalProperty>,
    pub children: Vec<Children>,
    pub lat: f64,
    pub lon: f64,
    pub hub_naptan_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub id: String,
    pub name: String,
    pub uri: String,
    #[serde(rename = "type")]
    pub type_field2: String,
    pub crowding: Crowding,
    pub route_type: String,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineGroup {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub naptan_id_reference: Option<String>,
    pub station_atco_code: String,
    pub line_identifier: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineModeGroup {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub mode_name: String,
    pub line_identifier: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProperty {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub category: String,
    pub key: String,
    pub source_system_key: String,
    pub value: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub naptan_id: String,
    pub modes: Vec<Value>,
    pub ics_code: String,
    pub station_naptan: String,
    pub lines: Vec<Value>,
    pub line_group: Vec<Value>,
    pub line_mode_groups: Vec<Value>,
    pub status: bool,
    pub id: String,
    pub common_name: String,
    pub place_type: String,
    pub additional_properties: Vec<Value>,
    pub children: Vec<Value>,
    pub lat: f64,
    pub lon: f64,
    pub hub_naptan_code: Option<String>,
    pub indicator: Option<String>,
    pub stop_letter: Option<String>,
}

// Line/<name>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineData {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub id: String,
    pub name: String,
    pub mode_name: String,
    pub disruptions: Vec<Value>,
    pub created: String,
    pub modified: String,
    pub line_statuses: Vec<Value>,
    pub route_sections: Vec<Value>,
    pub service_types: Vec<ServiceType>,
    pub crowding: Crowding,
}



impl JsonTrait for Version {}

impl JsonTrait for QuerySearch {}
impl JsonTrait for SearchMatch {}
impl JsonTrait for LineRouteSection {}

impl JsonTrait for LineRoute {}
impl JsonTrait for RouteSection {}
impl JsonTrait for ServiceType {}
impl JsonTrait for Crowding {}

impl JsonTrait for StopPoint {}
impl JsonTrait for Line {}
impl JsonTrait for LineGroup {}
impl JsonTrait for LineModeGroup {}
impl JsonTrait for AdditionalProperty {}
impl JsonTrait for Children {}

impl JsonTrait for LineData {}

#[derive(Debug, Deserialize, Serialize)]
#[enum_dispatch(JsonTrait)]
pub enum DataStruct {
    Version(Version),
    QuerySearch(QuerySearch),
    SearchMatch(SearchMatch),
    LineRouteSection(LineRouteSection),
    LineRoute(LineRoute),
    RouteSection(RouteSection),
    ServiceType(ServiceType),
    Crowding(Crowding),
    StopPoint(StopPoint),
    Line(Line),
    LineGroup(LineGroup),
    LineModeGroup(LineModeGroup),
    AdditionalProperty(AdditionalProperty),
    Children(Children),
    LineData(LineData)

}


