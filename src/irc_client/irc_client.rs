
extern crate rand;
extern crate mio;

use irc_client::Message;
use irc_client::url;

use std::io::*;
use std::net::SocketAddr;
use std::thread;
use std::sync::mpsc::channel;

use self::rand::{thread_rng, Rng};
use self::mio::{Token, Handler, EventLoop, EventSet, PollOpt};
use self::mio::tcp::TcpStream;

pub struct IRCClient {
    irc_socket: TcpStream,
    irc_nick_name: String,
    irc_real_name: String,
    irc_password: String,
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
        //println!("Startup");

        //self.connect();

        //// waitting for server response
        //let (socket_tx, socket_rx) = channel();
        //let mut reader = match self.irc_socket.as_mut() {
            //Some(x) => BufReader::new(x.try_clone().unwrap()),
            //None => panic!("Socket Error"),
        //};
        //let receive_thread = thread::spawn(move || {
            //loop {
                //let mut buf_line: String = String::new();
                //let _ = reader.read_line(&mut buf_line);
                //if buf_line.len() == 0 {
                    //continue;
                //}

                //let _ = socket_tx.send(buf_line).unwrap();
            //}
        //});

        //// waitting for user command
        //let (command_tx, command_rx) = channel();
        //let command_thread = thread::spawn(move || {
            //let stdin = stdin();
            //loop {
                //let mut command: String = String::new();
                //let mut handle = stdin.lock();
                //let _ = handle.read_line(&mut command);

                //let _ = command_tx.send(command);
            //}
        //});

        //// login
        let command = &format!("PASS {}", &self.irc_password);
        self.command(command);
        let command = &format!("NICK {}", &self.irc_nick_name);
        self.command(command);
        let command = &format!("USER {} 8 * :{}", &self.irc_nick_name,
                                                  &self.irc_real_name);
        self.command(command);

        //'msg: loop {
            //// check command
            //let command = command_rx.try_recv();
            //if command.is_ok() {
                //let command: String = command.unwrap();

                //// quit
                //if command == "exit\n" ||
                   //command == "EXIT\n" {
                    //break 'msg;
                //}

                //self.command(&command);
            //}

            //// check socket
            //let response = socket_rx.try_recv();
            //if response.is_ok() {
                //let response = response.unwrap();
                //self.process(&response);
            //}
        //}

        //println!("Waitting for thread finished.");
        //let _ = receive_thread.join();
        //let _ = command_thread.join();
    }

    // execute a command
    fn command(&mut self, command: &str) {
        println!("Command: {:?}", command);

        let buf = command.trim().to_owned() + "\n";
        self.remaining.push(buf);
    }

    // process message
    fn process(&mut self, msg: &Message) {
        println!("Process: {}", msg);

        match msg.command() {
            "PING" => self.process_ping(msg),
            "JOIN" => self.process_join(msg),
            "PRIVMSG" => self.process_privmsg(msg),
            "NOTICE" => self.process_notice(msg),
            "433" => self.process_433(),
            _ => println!("Msg not handled: {}", msg),
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
    fn process_join(&mut self, msg: &Message) {
        println!("Process JOIN command");
        let nick = msg.nickname();

        if nick != self.irc_nick_name {
            self.privmsg(msg.channel(), &format!("hi, {} :-)", nick));
        }
    }

    // process PING command
    fn process_ping(&mut self, msg: &Message) {
        println!("Process PING command");
        let command = "PONG ".to_owned() + msg.servername();
        self.command(&command);
    }

    fn process_notice(&mut self, msg: &Message) {
        println!("Process NOTICE command");

        let from = msg.nickname();

        if from == "NickServ" {
            self.join_channels();
        }
    }

    fn process_privmsg(&mut self, msg: &Message) {
        println!("Process PRIVMSG command");

        let content = msg.content();

        if let Some(res) = url::resolv_url(content) {
            self.privmsg(msg.channel(), &res[..]);
        }
    }

    fn process_433(&mut self) {
        println!("Process 433");

        let new_nick = format!("{}{}", self.irc_nick_name, thread_rng().gen_range(0, 100));
        self.irc_nick_name = new_nick;

        let msg = format!("NICK {}", self.irc_nick_name);
        self.command(&msg);
    }
}

impl Handler for IRCClient {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<IRCClient>, token: Token, events: EventSet) {
        //println!("{:?}", token);
        //println!("{:?}", events);
        //println!("{:?}", self.irc_socket);

        if events.is_writable() && self.remaining.len() != 0 {
            self.irc_socket.write_all(self.remaining[0].as_bytes());
            self.remaining.remove(0);
        }

        if events.is_readable() {
            let mut buf: String = String::new();
            self.irc_socket.read_to_string(&mut buf);

            println!("read: {}", buf);

            if let Ok(msg) = buf.parse::<Message>() {
                self.process(&msg);
            }
        }

        {
            //let s = self.irc_socket;
            //let t = self.mio_token;

            event_loop.reregister(&self.irc_socket, self.mio_token.unwrap(), EventSet::all(), PollOpt::all());
        }
    }
}
