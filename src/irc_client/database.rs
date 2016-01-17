
#[macro_use(bson, doc)]

use irc_client::mongodb::Client;

pub struct Database {
    addr: String,
}

impl Database {
    pub fn new() {
        //let client = Client::with_uri("mongodb://10.0.1.29:27017/")
            //.ok().expect("Failed to connect.");
    }
}
