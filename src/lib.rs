
#![feature(plugin)]

#![plugin(clippy)]

#[macro_use]
extern crate bson;
#[macro_use] 
extern crate log;
extern crate mio;
extern crate regex;
extern crate hyper;
extern crate mongodb;
extern crate rand;
extern crate image;
extern crate rustc_serialize;

pub use self::irc_client::IRCClient;
pub use self::message::Message;
pub use self::resolv_url::url;
pub use self::database::Database;

mod irc_client;
mod message;
mod resolv_url;
mod database;
