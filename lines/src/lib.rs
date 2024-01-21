mod client;
mod lines;
mod datastructs;

#[cfg(test)]
mod tests {
    use crate::{client::Client, datastructs::DataStruct};
    #[test]
    fn search_test() {
        // Does test for search/query
        let resp = Client::new("abcd1234").query("ok").fetch().unwrap();
        if let DataStruct::QuerySearch(data) = resp {
            assert!("Tfl.Api.Presentation.Entities.RouteSearchResponse, Tfl.Api.Presentation.Entities" == data.type_field, "Type Returned: {}", data.type_field)
        }
    }

    #[test]
    fn version_test() {
        let resp = Client::new("abcd1234").version().fetch().unwrap();
        if let DataStruct::Version(data) = resp {
            assert!("Tfl.Api.Common.ApiVersionInfo, Tfl.Api.Common" == data.type_field, "Type Returned: {}", data.type_field)
        }
    }
}
