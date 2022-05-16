use crate::helpers;

use std::io::{BufReader, BufRead, BufWriter};
use std::io::prelude::*;
use std::net::TcpStream;

use crate::dns::{
    DnsResolver
};

pub struct HttpHeader {
    key: String,
    value: String,
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
    pub fn from_bytes(bytes: &[u8]) ->HttpMessage{
        let mut method = String::new();
        let mut path = String::new();
        let mut version = String::new();
        let mut headers = Vec::<HttpHeader>::new();
        let mut body = String::new();
        let reader = BufReader::new(bytes);
        let mut is_header = true;

        for (index, line) in reader.lines().enumerate() {
            let buffer = line.unwrap();

            if index == 0 {
                let mut split = buffer.split(" ");
                method = helpers::ss_get(&mut split);
                path = helpers::ss_get(&mut split);
                version = helpers::ss_get(&mut split);
            }
            else if buffer == "" {
                is_header = false;
            }
            else if is_header {
                let mut split = buffer.split(": ");
                let header = HttpHeader {
                    key: helpers::ss_get(&mut split),
                    value: helpers::ss_get(&mut split),
                };
                headers.push(header);
            }
            else if is_header == false {
                body.push_str(&buffer);
            }
        }
        return HttpMessage{
            method: method,
            path: path,
            version: version,
            headers: headers,
            body: HttpBody{ data: body},
        };
    }

    pub fn to_string(&self) {
        println!("Method: {}", self.method);
        println!("Path: {}", self.path);
        println!("Version: {}", self.version);
        println!("Headers:");
        for header in &self.headers {
            println!("    {} : {}", header.key, header.value);
        }
        println!("Body:{}", self.body.data);
    }
}

impl HttpResponse {
    pub fn from_bytes(bytes: &[u8]) ->HttpResponse{
        let mut version = String::new();
        let mut status = String::new();
        let mut headers = Vec::<HttpHeader>::new();
        let mut body = String::new();
        let reader = BufReader::new(bytes);
        let mut is_header = true;

        for (index, line) in reader.lines().enumerate() {
            let buffer = line.unwrap();

            if index == 0 {
                let mut split = buffer.split(" ");
                version = helpers::ss_get(&mut split);
                status = helpers::ss_get(&mut split);
            }
            else if buffer == "" {
                is_header = false;
            }
            else if is_header {
                let mut split = buffer.split(": ");
                let header = HttpHeader {
                    key: helpers::ss_get(&mut split),
                    value: helpers::ss_get(&mut split),
                };
                headers.push(header);
            }
            else if is_header == false {
                body.push_str(&buffer);
            }

        }
        return HttpResponse{
            version: version,
            status: status,
            headers: headers,
            body: HttpBody{ data: body},
        };
    }

    pub fn to_string(&self) {
        println!("Version: {}", self.version);
        println!("Status Code: {}", self.status);
        println!("Headers:");
        for header in &self.headers {
            println!("    {} : {}", header.key, header.value);
        }
        println!("Body:{}", self.body.data);
    }
}

pub struct HttpClient{}

impl HttpClient{

    pub fn new() -> HttpClient {
        return HttpClient{}
    }

    pub fn get(&self, host: &str, path: &str) -> HttpResponse{
        let message = HttpMessage {
            method: String::from("GET"),
            path: path.to_string(),
            version: String::from("HTTP/1.1"),
            headers: vec![
                HttpHeader{ key: String::from("Host"), value: host.to_string() },
                HttpHeader{ key: String::from("User-Agent"), value: String::from("rustlib") },
                HttpHeader{ key: String::from("Connection"), value: String::from("keep-alive") },],
                body: HttpBody{ data: String::new() },
        };
        return self.send(&host, message);
    }

    fn read(&self, stream: &mut TcpStream) -> HttpResponse{
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let message = HttpResponse::from_bytes(&buffer);
        return message;
    }

    fn send(&self,host: &str, message: HttpMessage) -> HttpResponse{

        let ip = DnsResolver::get_host_by_name(host);
        let mut tcp = TcpStream::connect(ip+":80").unwrap();
        {
            let mut stream = BufWriter::new(&mut tcp);
            stream.write(message.method.as_bytes()).unwrap();
            stream.write(b" ").unwrap();
            stream.write(message.path.as_bytes()).unwrap();
            stream.write(b" ").unwrap();
            stream.write(message.version.as_bytes()).unwrap();
            stream.write(b"\r\n").unwrap();
            for header in &message.headers{
                stream.write(header.key.as_bytes()).unwrap();
                stream.write(b": ").unwrap();
                stream.write(header.value.as_bytes()).unwrap();
                stream.write(b"\r\n").unwrap();
            }
            stream.write(b"\r\n").unwrap();
            stream.flush().unwrap();
        }

        return self.read(&mut tcp);
    }
}
