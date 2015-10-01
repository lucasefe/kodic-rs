use hyper::Client;
use hyper::header::{Headers, Authorization, Basic, ContentType};
use rustc_serialize::{Encodable,json};
use rustc_serialize::json::Json;
use std::io::Read;

fn build_headers<'a>() -> Headers {
    let mut headers = Headers::new();

    headers.set(ContentType::json());
    headers.set(
        Authorization(
            Basic { username: "xbmc".to_owned(), password: Some("xbmc".to_owned()) }
        )
    ); 
    headers
}

fn build_client<'a>() -> Client {
    Client::new()
}

pub fn execute<T:Encodable>(request: T) -> Json {
    let client = build_client();
    let headers = build_headers();
    let body = json::encode(&request).unwrap();

    let mut result = client.post("http://10.0.1.3:8080/jsonrpc")
        .body(&body).headers(headers)
        .send().unwrap();

    let mut buffer = String::new();
    result.read_to_string(&mut buffer).unwrap();
    Json::from_str(&buffer).unwrap()
}
