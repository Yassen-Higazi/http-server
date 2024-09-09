use std::collections::HashMap;
use crate::request::Request;

pub struct Response {
    pub status: u16,
    pub body: String,
    pub request: Request,
    pub status_name: String,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(request: &Request) -> Response {
        let body = String::new();
        let headers = HashMap::new();

        Self {
            body,
            headers,
            status: 200,
            request: request.clone(),
            status_name: String::from("OK"),
        }
    }

    pub fn set_body(&mut self, body: String, content_type_option: Option<String>) {
        self.body = body;

        let content_length = self.body.len();
        let content_length_name = String::from("Content-Length");

        self.set_header(content_length_name, content_length.to_string());

        let mut content_type = String::from("text/plain");
        let content_type_name = String::from("Content-Type");

        if !content_type_option.is_none() {
            content_type = content_type_option.unwrap();
        }

        self.set_header(content_type_name, content_type);
    }

    fn set_header(&mut self, header_name: String, header_value: String) {
        // let header = self.headers.get(header_name.as_str());
        //
        // match header {
        //     None => {
        //         self.headers.insert(header_name, header_value);
        //     }
        //     Some(_) => {
        //     }
        // }
        self.headers.insert(header_name, header_value);
    }

    pub fn http(&self) -> Vec<u8> {
        let mut res = String::new();

        res.push_str(&format!("{}/{} {} {}\r\n", self.request.protocol, self.request.protocol_version, self.status, self.status_name));

        for (key, value) in &self.headers {
            res.push_str(&format!("{}: {}\r\n", key, value));
        }

        res.push_str("\r\n");

        res.push_str(&self.body);

        res.into_bytes()
    }
}

impl From<&Request> for  Response {
    fn from(req: &Request) -> Self {
        Response::new(req)
    }
}