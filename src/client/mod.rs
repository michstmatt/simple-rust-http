use crate::http::HttpClient;
use crate::dns::{
    DnsResolver
};

fn main() {
    let client = HttpClient::new();
    let message = client.get("172.217.11.228","/");
    message.to_string();
}
