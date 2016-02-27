
extern crate rand;
extern crate mio;
extern crate env_logger;

use database::Database;
use message::Message;
use resolv_url::url;

use std::io::*;
use std::net::SocketAddr;
use std::sync::mpsc::channel;

use self::rand::{thread_rng, Rng};
use self::mio::{Token, Handler, EventLoop, EventSet, PollOpt};
use self::mio::tcp::TcpStream;

pub struct IRCClient {
    irc_socket: TcpStream,
    irc_nick_name: String,
    irc_real_name: String,
    irc_password: String,
    irc_database: Database,
    auto_join_channels: Vec<String>,
    mio_token: Option<Token>,
    remaining: Vec<String>,
}

impl IRCClient {
    pub fn new(addr: &SocketAddr) -> IRCClient {

        let stream = TcpStream::connect(addr).unwrap();

        IRCClient {
            irc_socket: stream,
            irc_nick_name: "IRCRobot".to_owned(),
            irc_real_name: "IRCRobot".to_owned(),
            irc_password: "*".to_owned(),
            irc_database: Database::new(),
            auto_join_channels: Vec::new(),
            mio_token: None,
            remaining: Vec::new(),
        }
    }

    pub fn socket(&self) -> &TcpStream {
        &self.irc_socket
    }

    pub fn set_token(&mut self, token: Token) {
        self.mio_token = Some(token);
    }

    pub fn append_auto_join_channel(&mut self, channel: &str) {
        self.auto_join_channels.push(channel.to_owned());
    }

    pub fn set_nick_name(&mut self, name: &str) {
        self.irc_nick_name = name.to_owned();
    }

    pub fn set_real_name(&mut self, name: &str) {
        self.irc_real_name = name.to_owned();
    }

    pub fn set_password(&mut self, pwd: &str) {
        self.irc_password = pwd.to_owned();
    }

    pub fn startup(&mut self) {
        // login
        let command = &format!("PASS {}", &self.irc_password);
        self.command(command);
        let command = &format!("NICK {}", &self.irc_nick_name);
        self.command(command);
        let command = &format!("USER {} 8 * :{}", &self.irc_nick_name,
                                                  &self.irc_real_name);
        self.command(command);

        //self.join_channels();
    }

    // execute a command
    fn command(&mut self, command: &str) {
        info!("Command: {}", command);

        let buf = command.trim().to_owned() + "\n";
        self.remaining.push(buf);
    }

    // process message
    fn process(&mut self, msg: &Message) {

        match msg.command() {
            "PING" => self.process_ping(msg),
            "JOIN" => self.process_join(msg),
            "PRIVMSG" => self.process_privmsg(msg),
            "NOTICE" => self.process_notice(msg),
            "433" => self.process_433(),
            _ => debug!("Msg not handled: {}", msg),
        }
    }

    fn join_channels(&mut self) {
        let channels = self.auto_join_channels.clone();

        for channel in channels {
            self.join(&channel);
        }
    }

    fn join(&mut self, channel: &str) {
        let msg = "JOIN #".to_owned() + channel;
        self.command(&msg);
    }

    // PRIVMSG
    fn privmsg(&mut self, target: &str, msg: &str) {
        let msg = "PRIVMSG ".to_owned() + target + " :" + msg;
        self.command(&msg);
    }

    // process JOIN command
    fn process_join(&mut self, _msg: &Message) {
        //println!("Process JOIN command");
        //let nick = msg.nickname();

        //let tips = vec!["hi", "hello", "how are you", "hey", "welcome"];
        //let smile = vec![":)", "xD", ":D", ":P", "-_-", ":-/", ":-\\"];
        //let mut rng = rand::StdRng::new().unwrap();

        //if nick != self.irc_nick_name && nick != "ChanServ" {
            //self.privmsg(msg.channel(), &format!("{}, {} {}", rng.choose(&tips).unwrap(), nick, rng.choose(&smile).unwrap()));
        //}
    }

    // process PING command
    fn process_ping(&mut self, msg: &Message) {
        let command = "PONG ".to_owned() + msg.servername();
        self.command(&command);
    }

    fn process_notice(&mut self, msg: &Message) {
        debug!("Process NOTICE command");

        let from = msg.nickname();

        if from == "NickServ" {
            self.join_channels();
        }
    }

    fn process_privmsg(&mut self, msg: &Message) {
        debug!("Process PRIVMSG command");

        let content = msg.content();

        if let Some(res) = url::resolv_url(content) {
            self.privmsg(msg.channel(), &format!("{}, {}", msg.nickname(), &res[..]));
        }

        if content.contains(&self.irc_nick_name) {
            let content = content.replace(&self.irc_nick_name, msg.nickname());
            self.privmsg(msg.channel(), &content);
        }
    }

    fn process_433(&mut self) {
        debug!("Process 433");

        let new_nick = format!("{}{}", self.irc_nick_name, thread_rng().gen_range(0, 100));
        self.irc_nick_name = new_nick;

        let msg = format!("NICK {}", self.irc_nick_name);
        self.command(&msg);
    }

    fn ready_read(&mut self) {
        let mut buf: String = String::new();
        let _ = self.irc_socket.read_to_string(&mut buf);

        let list: Vec<&str> = buf.split('\n').collect();

        for item in list {
            if let Ok(msg) = item.parse::<Message>() {
                self.irc_database.record_message(&msg);
                self.process(&msg);
            }
        }
    }

    fn ready_write(&mut self) {
        let _ = self.irc_socket.write_all(self.remaining[0].as_bytes());
        let _ = self.irc_socket.flush();
        self.remaining.remove(0);
    }
}

impl Handler for IRCClient {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<IRCClient>, token: Token, events: EventSet) {

        if token != self.mio_token.unwrap() || events.is_error() {
            return;
        }

        if events.is_writable() && !self.remaining.is_empty() {
            self.ready_write();
        }

        if events.is_readable() {
            self.ready_read();
        }

        let event_set;

        if self.remaining.is_empty() {
            event_set = EventSet::readable();
        } else {
            event_set = EventSet::readable() | EventSet::writable();
        }

        let _ = event_loop.reregister(&self.irc_socket, self.mio_token.unwrap(), event_set, PollOpt::oneshot());
    }
}
