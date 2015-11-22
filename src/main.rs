
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
    irc.set_nick_name("Vivians");
    irc.set_real_name("Vivians");
    irc.set_password("Vivians");
    irc.append_auto_join_channel("test1");
//    irc.append_auto_join_channel("deepin");

    irc.startup();
}
