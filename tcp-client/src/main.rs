use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut s = TcpStream::connect("localhost:3000").unwrap();
    s.write_all(b"ping from client\n").unwrap();
    let mut buf = [0u8; 1024];
    let n = s.read(&mut buf).unwrap();
    println!("{}", String::from_utf8_lossy(&buf[..n]));
}
