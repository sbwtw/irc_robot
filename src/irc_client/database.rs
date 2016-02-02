
use irc_client::bson::Document;
use irc_client::mongodb::Client;
use irc_client::mongodb::ThreadedClient;
use irc_client::mongodb::cursor::Cursor;
use irc_client::mongodb::db::ThreadedDatabase;
use irc_client::mongodb::error::Error;
use irc_client::mongodb::coll::Collection;
use irc_client::mongodb::coll::options::FindOptions;

use irc_client::Message;

pub struct Database {
    //client: Client,
    coll_message: Collection,
}

impl Database {
    pub fn new() -> Database {

        let client = Client::connect("10.0.2.120", 27017).unwrap();
        let db = client.db("irc");
        let coll_message = db.collection("message_history");

        Database {
            //client: client,
            coll_message: coll_message,
        }
    }

    pub fn record_message(&mut self, msg: &Message) {

        //let res = 
        self.coll_message.insert_one(msg.as_bson_doc(), None).unwrap();
        //let res = self.coll_message.update_one(doc!{}, msg.as_bson_doc(), None).unwrap();
    }

    pub fn all_messages(&mut self) -> Result<Cursor, Error> {
        self.messages(None)
    }

    pub fn messages(&mut self, option: Option<FindOptions>) -> Result<Cursor, Error> {
        let mut doc = doc!{"command" => "PRIVMSG"};
        self.coll_message.find(Some(doc), option)
    }
}
