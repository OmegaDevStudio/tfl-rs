use serde::{Serialize, Deserialize};
use serde_json::Value;

// For Line/Search/<query> - Slightly different to below
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuerySearch {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub input: String,
    pub search_matches: Vec<SearchMatch>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchMatch {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub line_id: String,
    pub mode: String, 
    pub line_name: String,
    pub line_route_section: Vec<LineRouteSection>,
    pub matched_route_sections: Vec<Value>,
    pub matched_stops: Vec<Value>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub vehicle_destination_text: String
}

// For /Line/<id>/Route - For some reason it's slightly different to above
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LineRoute {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub id: String,
    pub name: String,
    pub mode_name: String,
    pub disruptions: Vec<Value>,
    pub line_statuses: Vec<Value>,
    pub route_sections: Vec<RouteSection>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RouteSection {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub name: String,
    pub direction: String,
    pub origination_name: String,
    pub destination_name: String,
    pub service_type: String,
}

// Line/<id>/StopPoints
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StopPoint {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub modes: Vec<String>,
    pub ics_code: String,
    pub stop_type: String,
    pub station_naptan: String,
    pub lines: Vec<StopPointLine>,
    pub line_group: Vec<StopPointLineGroup>,
    pub line_mode_groups: Vec<StopPointLineModeGroup>,
    pub status: bool,
    pub id: String,
    pub common_name: String,
    pub place_type: String,
    pub additional_properties: Vec<AdditionalProperty>,
    pub children: Vec<StopPointChildren>,
    pub lat: f32,
    pub lon: f32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StopPointLine {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field_2: String,
    pub crowding: Value,
    pub route_type: String,
    pub status: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StopPointLineGroup {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub station_atco_code: String,
    pub line_identifier: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StopPointLineModeGroup {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub mode_name: String,
    pub line_identifier: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalProperty {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub category: String,
    pub key: String,
    pub source_system_key: String,
    pub value: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StopPointChildren {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub naptan_id: String,
    pub modes: Vec<String>,
    pub ics_code: String,
    pub station_naptan: String,
    pub lines: Vec<StopPointLine>,
    pub line_group: Vec<StopPointLineGroup>,
    pub line_mode_groups: Vec<StopPointLineModeGroup>,
    pub status: bool,
    pub id: String,
    pub common_name: String,
    pub place_type: String,
    pub additional_properties: Vec<AdditionalProperty>,
    pub children: Vec<StopPointChildren>,
    pub lat: f32,
    pub lon: f32
}