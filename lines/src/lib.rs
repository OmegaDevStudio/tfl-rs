mod client;
mod lines;
mod datastructs;

#[cfg(test)]
mod tests {
    

    use crate::{client::Client, datastructs::{Response, DataStruct}, lines::Line};
    #[test]
    fn search_test() {
        // Does test for search/query
        let resp = Client::new("abcd1234").query("ok").fetch().unwrap();
        if let Response::Single(data) = resp {
            if let DataStruct::QuerySearch(data) = data {
                assert!("Tfl.Api.Presentation.Entities.RouteSearchResponse, Tfl.Api.Presentation.Entities"  == data.type_field, "Type Returned: {}", data.type_field)
            } else {
                assert!(false, "{:?}", data)
            }
        }
    }

    #[test]
    fn version_test() {
        let resp = Client::new("abcd1234").version().fetch().unwrap();
        if let Response::Single(data) = resp {
            if let DataStruct::Version(data) = data {
                assert!("Tfl.Api.Common.ApiVersionInfo, Tfl.Api.Common" == data.type_field, "Type Returned: {}", data.type_field)
            } else {
                assert!(false, "{:?}", data)
            }
        }
        
    }

    #[test]
    fn line_test() {
        let line = Line::Jubilee;
        let resp = Client::new("abcd1234").line(&line.line()).fetch().unwrap();
        if let Response::Multiple(data) = resp {
            for data in data {
            if let DataStruct::LineRoute(data) = data {
                assert!("Tfl.Api.Presentation.Entities.Line, Tfl.Api.Presentation.Entities" == data.type_field, "Type Returned: {}", data.type_field)
            } else {
                assert!(false, "{:?}", data)
            }}
        } else {
            assert!(false, "{:?}", resp)
        }
    }

    #[test]
    fn route_test() {
        let line = Line::Bakerloo;
        let resp = Client::new("abcd1234").route(&line.route()).fetch().unwrap();
        if let Response::Single(data) = resp {
            if let DataStruct::LineRoute(data) = data {
                assert!("Tfl.Api.Presentation.Entities.Line, Tfl.Api.Presentation.Entities"  == data.type_field, "Type Returned: {}", data.type_field)
            } else {
                assert!(false, "{:?}", data)
            }
        }
    }

    #[test]
    fn station_test() {
        let line = Line::Bakerloo;
        let resp = Client::new("abcd1234").route(&line.stations()).fetch().unwrap();
        if let Response::Multiple(data) = resp {
            for data in data {
                if let DataStruct::StopPoint(data) = data {
                    
                    assert!("Tfl.Api.Presentation.Entities.StopPoint, Tfl.Api.Presentation.Entities" == data.type_field, "Type Returned: {}", data.type_field)
                } else {
                    assert!(false, "{:#?}", data)
                }
            }
        }
    }
    
}
