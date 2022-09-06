use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};
use std::net::TcpStream;

use crate::dns::DnsResolver;

pub struct HttpHeader {
    key: String,
    value: String,
}

impl HttpHeader {
    pub fn to_string(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }
}

pub struct HttpBody {
    data: String,
}

pub struct HttpMessage {
    method: String,
    path: String,
    version: String,
    headers: Vec<HttpHeader>,
    body: HttpBody,
}

pub struct HttpResponse {
    version: String,
    status: String,
    headers: Vec<HttpHeader>,
    body: HttpBody,
}

impl HttpMessage {
    pub fn from_bytes(bytes: &[u8]) -> Option<HttpMessage> {
        let mut method: String = String::new();
        let mut path: String = String::new();
        let mut version: String = String::new();
        let mut headers = Vec::<HttpHeader>::new();
        let mut body = String::new();
        let reader = BufReader::new(bytes);
        let mut is_header = true;

        for (index, line) in reader.lines().enumerate() {
            let buffer = line.unwrap();

            if index == 0 {
                let mut split = buffer.split(" ");
                method = split.next()?.to_string();
                path = split.next()?.to_string();
                version = split.next()?.to_string();
            } else if buffer == "" {
                is_header = false;
            } else if is_header {
                let mut split = buffer.split(": ");
                let header = HttpHeader {
                    key: split.next()?.to_string(),
                    value: split.next()?.to_string(),
                };
                headers.push(header);
            } else if is_header == false {
                body.push_str(&buffer);
            }
        }
        return Some(HttpMessage {
            method: method,
            path: path,
            version: version,
            headers: headers,
            body: HttpBody { data: body },
        });
    }

    pub fn to_string(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(|h| format!("\t{} : {}", h.key, h.value))
            .reduce(|current, last| format!("{}\r\n{}", last, current))
            .expect("There was an error building the headers");
        format!(
            "Method: {}\r\n Path: {}\r\n Version: {}\r\n Headers:\r\n{}\r\n Body: {}",
            self.method, self.path, self.version, headers, self.body.data
        )
    }
}

impl HttpResponse {
    pub fn from_bytes(bytes: &[u8]) -> Option<HttpResponse> {
        let mut version: String = String::new();
        let mut status: String = String::new();
        let mut headers = Vec::<HttpHeader>::new();
        let mut body = String::new();
        let reader = BufReader::new(bytes);
        let mut is_header = true;

        for (index, line) in reader.lines().enumerate() {
            let buffer = line.unwrap();

            if index == 0 {
                let mut split = buffer.split(" ");
                version = split.next()?.to_string();
                status = split.next()?.to_string();
            } else if buffer == "" {
                is_header = false;
            } else if is_header {
                let mut split = buffer.split(": ");
                let header = HttpHeader {
                    key: split.next()?.to_string(),
                    value: split.next()?.to_string(),
                };
                headers.push(header);
            } else if is_header == false {
                body.push_str(&buffer);
            }
        }
        return Some(HttpResponse {
            version: version,
            status: status,
            headers: headers,
            body: HttpBody { data: body },
        });
    }

    pub fn to_string(&self) -> String {
        let response = format!(
            "Version: {}\r\nStatus Code: {}\r\nHeaders:\r\n\t",
            self.version, self.status
        );
        let headers = self
            .headers
            .iter()
            .map(|h| h.to_string())
            .reduce(|current, last| format!("{}\r\n\t{}", last, current))
            .unwrap();
        return response + &headers + &format!("\r\nBody:\r\n{}", self.body.data);
    }
}

fn stream_write(stream: &mut BufWriter<&mut TcpStream>, bytes: &[u8]) -> Result<(), String> {
    stream.write(bytes).map_err(|e| {
        format!(
            "There was an error writing to the tcp stream {}",
            e.to_string()
        )
    })?;
    Ok(())
}

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> HttpClient {
        return HttpClient {};
    }

    pub fn get(&self, host: &str, path: &str) -> Result<HttpResponse, String> {
        let message = HttpMessage {
            method: String::from("GET"),
            path: path.to_string(),
            version: String::from("HTTP/1.1"),
            headers: vec![
                HttpHeader {
                    key: String::from("Host"),
                    value: host.to_string(),
                },
                HttpHeader {
                    key: String::from("User-Agent"),
                    value: String::from("rustlib"),
                },
                HttpHeader {
                    key: String::from("Connection"),
                    value: String::from("keep-alive"),
                },
            ],
            body: HttpBody {
                data: String::new(),
            },
        };
        let ip = DnsResolver::get_host_by_name(host)?;
        return self.send(ip, message);
    }

    fn read(&self, stream: &mut TcpStream) -> Result<HttpResponse, String> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).map_err(|e| {
            format!(
                "There was an error reading from the stream buffer {}",
                e.to_string()
            )
        })?;
        let message = HttpResponse::from_bytes(&buffer);
        return message.ok_or_else(|| "Failed to read http response".to_string());
    }

    fn send(&self, ip: String, message: HttpMessage) -> Result<HttpResponse, String> {
        let mut tcp = TcpStream::connect(ip + ":80").map_err(|e| e.to_string())?;
        {
            let mut stream = BufWriter::new(&mut tcp);
            stream_write(&mut stream, message.method.as_bytes())?;
            stream_write(&mut stream, b" ")?;
            stream_write(&mut stream, message.path.as_bytes())?;
            stream_write(&mut stream, b" ")?;
            stream_write(&mut stream, message.version.as_bytes())?;
            stream_write(&mut stream, b"\r\n")?;
            for header in &message.headers {
                stream_write(&mut stream, header.key.as_bytes())?;
                stream_write(&mut stream, b": ")?;
                stream_write(&mut stream, header.value.as_bytes())?;
                stream_write(&mut stream, b"\r\n")?;
            }
            stream_write(&mut stream, b"\r\n")?;
            stream.flush().map_err(|e| {
                format!("There was an error flushing tcp buffer: {}", e.to_string())
            })?;
        }

        return self.read(&mut tcp);
    }
}
