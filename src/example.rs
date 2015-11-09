
#![feature(io)]

use std::io::*;
use std::net::TcpStream;
use std::thread;

fn main() {
//    let addr = "10.0.1.29:5056";
//    let addr = "localhost:21";
    let addr = "irc.freenode.net:6667";
    let mut irc_socket = TcpStream::connect(addr).unwrap();
    let mut reader = BufReader::new(irc_socket.try_clone().unwrap());
    let mut writer = BufWriter::new(irc_socket.try_clone().unwrap());

    let mut buf_line: String = String::new();
    let receive_thread = thread::spawn(move || {
        loop {
            reader.read_line(&mut buf_line);
            if buf_line.len() != 0 {
                println!("{:?}", buf_line);
                buf_line.clear();
            }
//            let mut b = Read::chars(&mut reader);
//            println!("{:?}， ", b.next().unwrap());
        }
    });

//    println!("Input: ");
//    irc_socket.write("PASS a123456\r\n".as_bytes());
//    irc_socket.write("GET / HTTP/1.1\r\n".as_bytes());
//    irc_socket.write("Connection: Close\r\n".as_bytes());
//    irc_socket.write("Host: www.baidu.com\r\n".as_bytes());
//    irc_socket.write("\r\n".as_bytes());
//    irc_socket.write("USER a\r\n\r\n".as_bytes());
//    irc_socket.flush();

//    let mut buf: String = String::new();
//    let stdin = stdin();
//    let mut handle = stdin.lock();
//    let _ = handle.read_to_string(&mut buf);

//    println!("Next shot.");
//    match irc_socket.write("USER a\r\n".as_bytes()) {
//        Ok(_) => (),
//        Err(e) => println!("Err: {:?}", e),
//    }

    let mut cmd: String = String::new();
    let stdin = stdin();
    loop {
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut cmd);
        println!("Command: {}", cmd);

        irc_socket.write(cmd.as_bytes());
        irc_socket.write("\r\n".as_bytes());
        irc_socket.flush();

        cmd.clear();
    }

//    irc.write("GET / HTTP/1.1\r\n".as_bytes());
//    irc.write("Connection: Close\r\n".as_bytes());
//    irc.write("Host: www.baidu.com\r\n".as_bytes());
//    irc.write("\r\n".as_bytes());
//    irc.flush();

//    let stdin = stdin();
//    loop {
//        let data = stdin.lock().lines().next().unwrap().unwrap();
//
//        println!("Command: {}", data);
//
//        match irc.write(data.as_bytes()) {
//            Ok(_) => (),
//            Err(e) => println!("Err: {:?}", e),
//        }
//    }

    println!("Waitting for finished.");
    receive_thread.join();

//    data.push_str("GET / HTTP/1.1\r\n");
//    data.push_str("Connection: Close\r\n");
//    data.push_str("Host: www.baidu.com\r\n");
//    data.push_str("\r\n");

//    match irc.write(data.as_bytes()) {
//        Ok(_) => (),
//        Err(e) => println!("Err: {:?}", e),
//    }
//    irc.write("\r\n".as_bytes());
//    irc.flush();
//    match irc.read_to_string(&mut buf) {
//        Ok(_) => println!("response: {}", buf),
//        Err(e) => println!("Err: {:?}", e),
//    }
}
