mod datastructs;
mod lines;
mod client;

use client::{Client, Request};
use lines::Line;
use datastructs::{DataStruct, QuerySearch, SearchMatch, Response};

fn main() {
    let line = Line::Bakerloo;
        let resp = Client::new("abcd1234").line(&line.line()).fetch().unwrap();
        match resp {
            Response::Single(data) => {
                if let DataStruct::LineRoute(data) = data {
                    println!("{:#?}", data.type_field);
                } else {
                    assert!(false, "{:?}", data)
                }
            }
            Response::Multiple(data) => {
                for data in data {
                    println!("{:#?}", data);
                }
            }
        }
        
    
}
