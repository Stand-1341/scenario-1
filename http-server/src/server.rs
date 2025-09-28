use crate::http::{HttpRequest, HttpResponse};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::router::Router;

pub struct Server {
    addr: String,
    router: Router,
}

impl Server {
    pub fn new(addr: String, router: Router) -> Self {
        Self { addr, router }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        eprintln!("HTTP server listening on http://{}", self.addr);
        for stream in listener.incoming() {
            match stream {
                Ok(mut s) => {
                    let _ = self.handle_client(&mut s);
                }
                Err(_) => {}
            }
        }
        Ok(())
    }

    fn handle_client(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let mut buf = vec![0u8; 8192];
        let mut read_total = 0usize;
        loop {
            let n = stream.read(&mut buf[read_total..])?;
            if n == 0 {
                break;
            }
            read_total += n;
            if read_total >= 4 && &buf[read_total - 4..read_total] == b"\r\n\r\n" {
                break;
            }
            if read_total == buf.len() {
                buf.resize(buf.len() * 2, 0);
            }
        }
        let text = String::from_utf8_lossy(&buf[..read_total]).into_owned();
        let mut req = HttpRequest::from(text.as_str());
        if let Some(cl) = req
            .header("Content-Length")
            .and_then(|v| v.parse::<usize>().ok())
        {
            let have = req.msg_body.len();
            if cl > have {
                let need = cl - have;
                let mut more = vec![0u8; need];
                stream.read_exact(&mut more)?;
                req.msg_body.push_str(&String::from_utf8_lossy(&more));
            }
        }
        let resp: HttpResponse = self.router.route(&req);
        stream.write_all(resp.to_string().as_bytes())?;
        Ok(())
    }
}
