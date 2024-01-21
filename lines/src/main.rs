mod datastructs;
mod lines;
mod client;

use client::{Client, Request};
use lines::Line;
use datastructs::{DataStruct, QuerySearch, SearchMatch};

fn main() { 
    let client = Client::new("abcd");
    let res: DataStruct = client.route(Line::Central.line()).fetch().unwrap();

    match res {
        DataStruct::QuerySearch(data) => {
            for matched in data.search_matches {
                println!("{}", matched.line_name);
            }
        },

        DataStruct::Version(data) => {
            println!("{}", data.type_field)
        },

        DataStruct::LineRoute(data) => println!("{}", data.type_field),

        _ => ()
    }

}