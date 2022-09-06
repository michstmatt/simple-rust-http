use crate::http::HttpMessage;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct HttpServer {
    socket: TcpListener,
}

impl HttpServer {
    pub fn new(port: u32) -> Result<Self, String> {
        return Ok(HttpServer {
            socket: TcpListener::bind(format!("127.0.0.1:{}", port))
                .map_err(|_| format!("Could not bind to: {}", port))?,
        });
    }

    pub fn start(&self) -> Result<(), String> {
        for stream in self.socket.incoming() {
            let mut s: TcpStream =
                stream.map_err(|e| format!("Could not get a tcp stream: {}", e.to_string()))?;
            let mut buffer = [0; 1024];
            s.read(&mut buffer)
                .map_err(|e| format!("There was an error reading the stream: {}", e.to_string()))?;
            if buffer.starts_with(b"GET") || buffer.starts_with(b"POST") {
                println!("HTTP message!");
                let message = HttpMessage::from_bytes(&buffer);
                match message {
                    Some(m) => m.to_string(),
                    _ => format!("ERROR: HTTP message is malformed"),
                };
            }
        }
        Ok(())
    }
}
