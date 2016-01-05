
use irc_client::regex::Regex;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
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

        let list: Vec<&str> = msg.split(" ").collect();
        if list.len() <= 2 {
            return Err("parse error.");
        }

        let first = list[0];
        let pass_size;
        if first.starts_with(":") {
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
fn message_test() {
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
