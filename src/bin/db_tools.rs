
#[macro_use]
extern crate bson;
extern crate mongodb;
extern crate irc;

use bson::Bson;
use irc::Database;
use mongodb::coll::options::FindOptions;

fn main() {
    let mut db = Database::new();
    let mut option = FindOptions::new();
    option.limit = 10;
    option.sort = Some(doc!{"_id" => (-1)});

    let cursor = db.messages(Some(option)).unwrap();

    for c in cursor {
        if c.is_err() {continue;}

        let bson = c.unwrap();

        match bson.get("content") {
            Some(&Bson::String(ref string)) => println!("{}", string),
            _ => {},
        };
    }
}
