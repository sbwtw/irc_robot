
#![feature(lookup_host)]
#![feature(ip_addr)]

#[macro_use]
extern crate bson;
extern crate mio;

mod irc_client;

use irc_client::IRCClient;

use mio::{Token, EventLoop, EventSet, PollOpt};

use std::net::{SocketAddr};

const IRC_CLIENT: Token = Token(0);

fn main() {
    let server = "irc.freenode.net";
    let port = 6667;
    let host = std::net::lookup_host(server).unwrap().next().unwrap().unwrap();
    let addr = SocketAddr::new(host.ip(), port);
    //irc.set_addr(addr);
    //irc.append_auto_join_channel("deepin");
    //irc.startup();

    //let addr: SocketAddr = "91.217.189.42:6667".parse().unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    let mut irc = IRCClient::new(&addr);
    irc.set_token(IRC_CLIENT);
    irc.set_nick_name("Vivians");
    irc.set_real_name("Vivians");
    irc.set_password("Vivians");
    irc.append_auto_join_channel("test1");
    irc.startup();

    let _ = event_loop.register(irc.socket(), IRC_CLIENT, EventSet::readable(), PollOpt::oneshot());
    let _ = event_loop.run(&mut irc);
}
