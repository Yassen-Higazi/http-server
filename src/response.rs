use crate::request::Request;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Write;
use std::net::TcpStream;

pub enum HttpCode {
    Ok,
    Created,
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
            HttpCode::Created => "201 Created",
        }
    }
}

#[derive(Debug)]
pub enum ContentType {
    Json,
    TextPlain,
    Multipart,
    UrlEncoded,
    OctetStream,
}

impl ContentType {
    pub fn text(&self) -> &str {
        match self {
            ContentType::Multipart => "multipart ",
            ContentType::TextPlain => "text/plain",
            ContentType::Json => "application/json",
            ContentType::OctetStream => "application/octet-stream",
            ContentType::UrlEncoded => "application/x-www-form-urlencoded",
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text().to_string())
    }
}

pub struct Response {
    pub body: Vec<u8>,
    pub status: HttpCode,
    pub protocol: String,
    pub protocol_version: String,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(protocol: String, protocol_version: String) -> Response {
        let body = Vec::new();
        let mut headers = HashMap::new();

        headers.insert("Content-Length".to_string(), "0".to_string());
        headers.insert("Content-Type".to_string(), ContentType::TextPlain.to_string());

        Self {
            body,
            headers,
            protocol,
            protocol_version,
            status: HttpCode::Ok,
        }
    }

    pub fn write_to(&mut self, stream: &mut TcpStream, body: Option<Vec<u8>>) {
        if body.is_some() {
            self.set_body(body.unwrap(), None);
        }

        stream.write_all(&*self.to_http_format()).expect("Failed to send Response to client");
    }

    pub fn set_json_body(&mut self, body: String) {
        self.set_body_string(body, Some(ContentType::Json))
    }

    pub fn set_body_string(&mut self, body: String, content_type: Option<ContentType>) {
        self.set_body(body.as_bytes().to_owned(), content_type)
    }

    pub fn set_body(&mut self, body: Vec<u8>, content_type_option: Option<ContentType>) {
        self.body = body;

        let content_length = self.body.len();
        let content_length_name = String::from("Content-Length");

        self.set_header(content_length_name, content_length.to_string());

        let content_type = content_type_option.unwrap_or_else(|| ContentType::TextPlain);

        self.set_content_type(content_type);
    }

    pub fn set_content_type(&mut self, content_type: ContentType) {
        let content_type_name = String::from("Content-Type");

        self.set_header(content_type_name, content_type.to_string());
    }

    pub fn set_header(&mut self, header_name: String, header_value: String) {
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
        let mut res: Vec<u8> = Vec::new();

        res.extend_from_slice(&format!("{}/{} {}\r\n", self.protocol, self.protocol_version, self.status.to_status_line()).as_bytes());

        for (key, value) in &self.headers {
            res.extend_from_slice(&format!("{}: {}\r\n", key, value).as_bytes());
        }

        res.extend_from_slice("\r\n".as_bytes());

        res.extend(&self.body);

        res
    }
}

impl From<&Request> for Response {
    fn from(req: &Request) -> Self {
        Response::new(req.protocol.clone(), req.protocol_version.clone())
    }
}