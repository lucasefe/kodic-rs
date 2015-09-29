extern crate hyper;
extern crate rustc_serialize;

use std::env;
use std::process;
use std::io::Read;
use rustc_serialize::json;
use hyper::header::{Headers, Authorization, Basic, ContentType};
use hyper::Client;

static VALID_COMMANDS: &'static [ &'static str ] = &["version"];

#[derive(RustcDecodable, RustcEncodable)]
struct RequestBody {
  jsonrpc: String,
  method: String,
  id: u8
}

// TODO: Check for proper error code to exit with. 
fn exit_with_error(message: &str) {
    println!("ERR: {}", message);
    process::exit(1);
}

fn run(command: &str) {
    if !&VALID_COMMANDS.contains(&command) {
        exit_with_error(&format!("Invalid command: {}", command));
    }
    
    println!("Executing command: {}", command);
    
    let body = RequestBody {
        jsonrpc: "2.0".to_string(), 
        method:  "JSONRPC.Version".to_string(),
        id: 1
    };

    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set(
        Authorization(
            Basic { 
                username: "xbmc".to_owned(), 
                password: Some("xbmc".to_owned())
            }
        )
    );
    
    let json_body = json::encode(&body).unwrap();
    let mut res = client.post("http://10.0.1.3:8080/jsonrpc")
        .body(&json_body)
        .headers(headers)
        .send()
        .unwrap();

    let mut buffer = String::new();
    res.read_to_string(&mut buffer).unwrap();
    println!("{}", buffer);
    println!("{}", res.status_raw().0);
}

// parse action
// build
// execute
// parse response
// render

fn main() {
    match env::args().nth(1) {
        Some(command)   => run(&command),
        None            => exit_with_error("you need to provide a command"),
    }
}
