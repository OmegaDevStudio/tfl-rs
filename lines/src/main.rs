mod datastructs;
mod lines;
mod client;
use client::{Client, Request};
use lines::Line;


#[tokio::main]
async fn main() { 
    let stations = Line::Metropolitan.stations().await.unwrap();
    for station in stations {
        println!("{}", station.common_name)
    }
}