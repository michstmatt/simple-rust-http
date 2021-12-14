mod http;
mod server;

use http::HttpMessage;
use server::HttpServer;

fn main() {
    let server = HttpServer::new(8080);
    server.start();
    println!("hello");
}
