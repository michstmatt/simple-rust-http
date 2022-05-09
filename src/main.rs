pub mod helpers;
pub mod http;
pub mod server;
pub mod client;
pub mod dns;

use http::HttpMessage;
use server::HttpServer;
use dns::DnsResolver;

fn main() {
    let fmt = DnsResolver::change_dns_name("www.google.com");
    println!("{}", fmt);
    let server = HttpServer::new(8080);
    server.start();
    println!("hello");
}
