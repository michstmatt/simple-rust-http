pub mod helpers;
pub mod http;
pub mod server;
pub mod client;
pub mod dns;

use http::{
    HttpClient
};

fn main() {
    let host = "google.com";
    let client = HttpClient::new();
    let response = client.get(&host , "/");
    
    println!("Response from:{}", host);
    response.to_string();
}
