use std::fmt;
use rustc_serialize::json::Json;

static MAJOR_KEY: &'static [ &'static str ] = &["result", "version", "major"];
static MINOR_KEY: &'static [ &'static str ] = &["result", "version", "minor"];
static PATCH_KEY: &'static [ &'static str ] = &["result", "version", "patch"];

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Request {
    id: u64,
    jsonrpc: String,
    method: String,
}

#[derive(Debug)]
pub struct Response {
    major: u64,
    minor: u64,
    patch: u64
}

impl Request {
    pub fn new() -> Request {
        Request {
            jsonrpc: "2.0".to_string(),
            method:  "JSONRPC.Version".to_string(),
            id: 1
        }
    }
}

impl Response {
    pub fn new(json: Json) -> Response {
        Response {
            major: json.find_path(&MAJOR_KEY).unwrap().as_u64().unwrap(),
            minor: json.find_path(&MINOR_KEY).unwrap().as_u64().unwrap(),
            patch: json.find_path(&PATCH_KEY).unwrap().as_u64().unwrap()
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
