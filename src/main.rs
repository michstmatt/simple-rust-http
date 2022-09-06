pub mod client;
pub mod dns;
pub mod http;
pub mod server;

use http::HttpClient;

fn main() {
    let host = "google.com";
    let client = HttpClient::new();
    let response = client.get(&host, "/");

    println!("Response from:{}", host);
    println!(
        "{}",
        match response {
            Ok(r) => r.to_string(),
            Err(e) => format!("Error!: {}", e),
        }
    );
}
