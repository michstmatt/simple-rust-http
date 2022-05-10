pub mod helpers;
pub mod http;
pub mod server;
pub mod client;
pub mod dns;

use http::HttpMessage;
use server::HttpServer;
use dns::DnsResolver;

fn main() {
    DnsResolver::get_host_by_name("www.google.com");
    let server = HttpServer::new(8080);
    server.start();
    println!("hello");
}
