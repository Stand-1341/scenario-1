use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("listening on 127.0.0.1:3000");
    for incoming in listener.incoming() {
        let mut stream = incoming.unwrap();
        println!("connection established from {}", stream.peer_addr().unwrap());
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).unwrap();
        if n > 0 {
            println!("received: {}", String::from_utf8_lossy(&buf[..n]));
        }
        stream.write_all(b"hello\n").unwrap();
    }
}
