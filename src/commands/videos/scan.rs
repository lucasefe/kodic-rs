use std::fmt;
use rustc_serialize::json::Json;

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct Params {
	directory: String
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Request {
    id: u64,
    jsonrpc: String,
    method: String,
    params: Params,
}

#[derive(Debug)]
pub struct Response {
    data: String
}

impl Request {
    pub fn new() -> Request {
        Request {
            jsonrpc: "2.0".to_string(),
            method:  "VideoLibrary.Scan".to_string(),
            id: 2,
            params: Params {
            	directory: "".to_string()
            }
        }
    }
}

impl Response {
    pub fn new(json: Json) -> Response {
        Response {
            data: json.to_string()
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
