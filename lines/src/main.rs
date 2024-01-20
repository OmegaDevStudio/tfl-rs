mod datastructs;
mod lines;
mod client;

use client::{Client, Request};
use lines::Line;
use datastructs::{DataStruct, QuerySearch, SearchMatch};

fn main() { 
    let client = Client::new("abcd");
    let res: DataStruct = client.query("205").fetch().unwrap();

    if let DataStruct::QuerySearch(data) = res {
        for matched in data.search_matches {
            println!("{}", matched.line_name);
        }
    }

}