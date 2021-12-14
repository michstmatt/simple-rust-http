#[path = "./http.rs"] mod http;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use http::HttpMessage;

pub struct HttpServer {
    port: u32,
    socket: TcpListener,
}

impl HttpServer {
    pub fn new(port: u32) -> Self{
        return HttpServer{
            port: port,
            socket : TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap(),
        }
    }

    pub fn start(&self) {
        for stream in self.socket.incoming() {
            let mut s: TcpStream = stream.unwrap();
            let mut buffer = [0; 1024];
            s.read(&mut buffer).unwrap();
            if buffer.starts_with(b"GET") || buffer.starts_with(b"POST") {
                println!("HTTP message!");
                let message = HttpMessage::from_bytes(&buffer);
                message.to_string();
                let mut outStream = [0; 1024];
                //message.to_bytes(&mut outStream);
                //println!("{}", String::from_utf8_lossy(&outStream[..]));
            }
        }
    }
}
