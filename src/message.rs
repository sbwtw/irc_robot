
use bson::*;
use regex::Regex;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

pub struct Message {
    raw_message: String,
    prefix: String,
    command: String,
    params: String,
}

impl Message {
    pub fn new(msg: &str) -> Result<Message, &'static str> {
        let mut prefix: String = String::new();
        let command: String;

        let list: Vec<&str> = msg.split(' ').collect();
        if list.len() < 2 {
            return Err("parse error.");
        }

        let first = list[0];
        let pass_size;
        if first.starts_with(':') {
            prefix = list[0].to_owned();
            command = list[1].to_owned();
            pass_size = prefix.len() + command.len() + 2;
        } else {
            command = list[0].to_owned();
            pass_size = command.len() + 1;
        }
        let (_, params) = msg.split_at(pass_size);

        Ok(Message{
            raw_message: msg.to_owned(),
            prefix: prefix,
            command: command,
            params: params.trim().to_owned(),
        })
    }

    pub fn as_bson_doc(&self) -> Document {
        let mut doc = doc!{"raw" => (self.raw_message.clone())};

        if !self.channel().is_empty() {
            doc.insert("channel".to_owned(), Bson::String(self.channel().to_owned()));
        }

        if !self.nickname().is_empty() {
            doc.insert("nick".to_owned(), Bson::String(self.nickname().to_owned()));
        }

        if !self.content().is_empty() {
            doc.insert("content".to_owned(), Bson::String(self.content().to_owned()));
        }

        if !self.params().is_empty() {
            doc.insert("params".to_owned(), Bson::String(self.params().to_owned()));
        }

        if !self.command().is_empty() {
            doc.insert("command".to_owned(), Bson::String(self.command().to_owned()));
        }

        doc
    }

    pub fn is_maybe_bot(&self) -> bool {
        let nick = self.nickname();

        nick.contains("bot") || nick.contains("bridge")

        //if nick.contains("bot") || nick.contains("bridge") {
            //true
        //} else {
            //false
        //}
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn params(&self) -> &str {
        &self.params
    }

    pub fn servername(&self) -> &str {
        if self.prefix.is_empty() {
            &self.params[1..]
        } else {
            &self.prefix[1..]
        }
    }

    pub fn nickname(&self) -> &str {
        let (nick, _, _) = self.resolv_prefix();
        nick
    }

    pub fn resolv_prefix(&self) -> (&str, &str, &str) {
        let re = Regex::new(r"^:([-\w\d]+)!(\S+)@(\S+)$").unwrap();
        if let Some(t) = re.captures(&self.prefix) {
            (t.at(1).unwrap(), t.at(2).unwrap(), t.at(3).unwrap())
        } else {
            ("", "", "")
        }
    }

    pub fn channel(&self) -> &str {
        let re = Regex::new(r"^(#[\w\d]+).*?").unwrap();
        if let Some(t) = re.captures(&self.params) {
            t.at(1).unwrap()
        } else {
            ""
        }
    }

    pub fn content(&self) -> &str {
        let re = Regex::new(r"^#[\w\d]+ :(.+)$").unwrap();
        if let Some(t) = re.captures(&self.params) {
            t.at(1).unwrap()
        } else {
            ""
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for Message {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nraw_message: {}", self.raw_message).unwrap();
        writeln!(f, "prefix: {}", self.prefix).unwrap();
        writeln!(f, "command: {}", self.command()).unwrap();
        writeln!(f, "params: {}", self.params())
    }
}

impl FromStr for Message {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        Message::new(s)
    }
}

#[test]
fn test() {
    let msg = Message::new(":username1!~username1@8.8.8.8 JOIN #test1\r\n").unwrap();
    assert_eq!(msg.command(), "JOIN");
    assert_eq!(msg.params(), "#test1");
    assert_eq!(msg.resolv_prefix(), ("username1", "~username1", "8.8.8.8"));

    let msg = Message::new(":sbw!~Thunderbi@8.8.8.8 PRIVMSG #test1 :123\r\n").unwrap();
    assert_eq!(msg.command(), "PRIVMSG");
    assert_eq!(msg.channel(), "#test1");
    assert_eq!(msg.content(), "123");

    let msg = Message::new("PING :hitchcock.freenode.net\r\n").unwrap();
    assert_eq!(msg.servername(), "hitchcock.freenode.net");
    assert_eq!(msg.resolv_prefix(), ("", "", ""));

    let msg = Message::new(":sbw!~Thunderbi@1.1.1.1 PRIVMSG #test1 :b, a \r\n").unwrap();
    assert_eq!(msg.content(), "b, a");

    let msg: Message = ":wilhelm.freenode.net 433 * username1 :Nickname is already in use.".parse().unwrap();
    assert_eq!(msg.command(), "433");
}

#[test]
fn test_brige_bot() {
    let msg = ":somebot!~a@1.1.1.1 PRIVMSG #test1 :some-bot realname: realcontent".parse::<Message>().unwrap();
    assert_eq!(msg.is_maybe_bot(), true);

    let msg = ":somebridge!~a@1.1.1.1 PRIVMSG #test1 :some-bot realname: realcontent".parse::<Message>().unwrap();
    assert_eq!(msg.is_maybe_bot(), true);
}
