mod client;
mod lines;
mod datastructs;

#[cfg(test)]
mod tests {
    use crate::{client::Client, datastructs::DataStruct};
    #[test]
    fn search_test() {
        let resp = Client::new("abcd1234").query("205").fetch().unwrap();
        if let DataStruct::QuerySearch(data) = resp {
            assert!("Tfl.Api.Presentation.Entities.RouteSearchResponse, Tfl.Api.Presentation.Entities" == data.type_field, "Type Returned: {}", data.type_field)
        }
        
    }
}
