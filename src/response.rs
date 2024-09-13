use crate::request::Request;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Write;
use std::net::TcpStream;

pub enum HttpCode {
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
}

impl HttpCode {
    pub fn to_status_line(&self) -> &str {
        match self {
            HttpCode::Ok => "200 OK",
            HttpCode::NotFound => "404 Not Found",
            HttpCode::BadRequest => "400 Bad Request",
            HttpCode::InternalServerError => "500 Internal Server Error",
        }
    }
}

#[derive(Debug)]
pub enum ContentType {
    TextPlain,
}

impl ContentType {
    pub fn text(&self) -> &str {
        match self {
            ContentType::TextPlain => "text/plain",
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text().to_string())
    }
}

pub struct Response {
    pub body: String,
    pub status: HttpCode,
    pub protocol: String,
    pub protocol_version: String,
    pub content_type: ContentType,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(protocol: String, protocol_version: String) -> Response {
        let body = String::new();
        let mut headers = HashMap::new();

        headers.insert("Content-Length".to_string(), "0".to_string());
        headers.insert("Content-Type".to_string(), ContentType::TextPlain.to_string());

        Self {
            body,
            headers,
            protocol,
            protocol_version,
            status: HttpCode::Ok,
            content_type: ContentType::TextPlain,
        }
    }

    pub fn write_to(&mut self, stream: &mut TcpStream, body: Option<String>) {
        if body.is_some() {
            self.set_body(body.unwrap(), None);
        }

        stream.write_all(&*self.to_http_format()).expect("Failed to send Response to client");
    }

    pub fn set_body(&mut self, body: String, content_type_option: Option<ContentType>) {
        self.body = body;

        let content_length = self.body.len();
        let content_length_name = String::from("Content-Length");

        self.set_header(content_length_name, content_length.to_string());

        let content_type_name = String::from("Content-Type");
        let content_type = match content_type_option {
            None => {
                ContentType::TextPlain
            }
            Some(ct) => {
                ct
            }
        };

        self.set_header(content_type_name, content_type.to_string());
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

    pub fn to_http_format(&self) -> Vec<u8> {
        let mut res = String::new();

        res.push_str(&format!("{}/{} {}\r\n", self.protocol, self.protocol_version, self.status.to_status_line()));

        for (key, value) in &self.headers {
            res.push_str(&format!("{}: {}\r\n", key, value));
        }

        res.push_str("\r\n");

        res.push_str(&self.body);

        res.into_bytes()
    }
}

impl From<&Request> for Response {
    fn from(req: &Request) -> Self {
        Response::new(req.protocol.clone(), req.protocol_version.clone())
    }
}