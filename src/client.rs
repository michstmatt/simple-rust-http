mod http;

use http::HttpClient;

fn main() {
    let client = HttpClient::new();
    let message = client.Get("172.217.11.228","/");
    message.to_string();
}
