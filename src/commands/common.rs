use std::fmt;
use rustc_serialize::json::Json;

#[derive(Debug)]
pub struct Response {
    data: String
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
