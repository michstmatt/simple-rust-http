use crate::dns::DnsResolver;
use crate::http::HttpClient;

fn main() {
    let client = HttpClient::new();
    let message = client.get("172.217.11.228", "/");
    println!(
        "{}",
        match message {
            Ok(m) => m.to_string(),
            Err(e) => e,
        }
    );
}
