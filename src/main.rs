
#![feature(lookup_host)]
#![feature(ip_addr)]

mod irc_client;

use irc_client::IRCClient;
use std::net::SocketAddr;

fn main() {
    let server = "irc.freenode.net";
    let port = 6667;
    let host = std::net::lookup_host(server).unwrap().next().unwrap().unwrap();
    let addr = SocketAddr::new(host.ip(), port);
    let mut irc = IRCClient::new();
    irc.set_addr(addr);
    irc.set_nick_name("username1");
    irc.set_real_name("username");
    irc.set_password("mypass");

    irc.startup();
}
