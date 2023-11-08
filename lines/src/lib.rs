mod client;
mod lines;
mod servicetypes;
mod severity;
mod datastructs;


#[cfg(test)]
mod tests {
    use crate::client::Client;
    #[test]
    fn types() {

        let client = Client::new("5bbfdb63ad4a426c8533e708389927ae");
        println!("{:?}", client);
    }
}
