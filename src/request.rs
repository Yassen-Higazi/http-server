use crate::constants::CRLF;
use crate::options::Options;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
}

impl HTTPMethod {
    fn text(&self) -> &'static str {
        match self {
            HTTPMethod::GET => "GET",
            HTTPMethod::PUT => "PUT",
            HTTPMethod::POST => "POST",
            HTTPMethod::HEAD => "HEAD",
            HTTPMethod::PATCH => "PATCH",
            HTTPMethod::DELETE => "DELETE",
            HTTPMethod::OPTIONS => "OPTIONS",
        }
    }
}

impl Display for HTTPMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text().to_string())
    }
}

impl From<&str> for HTTPMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => HTTPMethod::GET,
            "PUT" => HTTPMethod::PUT,
            "POST" => HTTPMethod::POST,
            "HEAD" => HTTPMethod::HEAD,
            "PATCH" => HTTPMethod::PATCH,
            "DELETE" => HTTPMethod::DELETE,
            "OPTIONS" => HTTPMethod::OPTIONS,
            _ => panic!("Invalid HTTP method"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub protocol: String,
    pub method: HTTPMethod,
    pub host: Option<String>,
    pub server_options: Options,
    pub protocol_version: String,
    pub query: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub headers: HashMap<String, String>,

    pub body: String,
}

impl Request {
    pub fn new(request: String, options: Options) -> Self {
        let params: HashMap<String, String> = HashMap::new();
        let mut headers: HashMap<String, String> = HashMap::new();

        let request_parts = request.split(CRLF).collect::<Vec<&str>>();

        // parse first part
        let first_line = request_parts[0];

        let (method, url, query, protocol, protocol_version, host) = Self::parse_first_line(first_line);

        // parse headers
        let mut i = 1;

        while i < request_parts.len() && request_parts[i] != "" {
            let header_parts: Vec<&str> = request_parts[i].split(": ").collect();

            let header_name = header_parts[0].to_string();

            let header_value = header_parts[1].to_string();

            headers.insert(header_name, header_value);

            i += 1;
        }

        // parse body
        let body_str: &str = request_parts[i + 1];

        let body = body_str.trim().to_string();

        let mut final_host = match headers.get("Host") {
            None => None,
            Some(host) => Some(host.clone()),
        };

        if !host.is_empty() {
            final_host = Some(host);
        };

        Self {
            url,
            body,
            query,
            params,
            headers,
            protocol,
            protocol_version,
            host: final_host,
            server_options: options,
            method: HTTPMethod::from(method.as_str()),
        }
    }

    fn parse_first_line(first_line: &str) -> (String, String, HashMap<String, String>, String, String, String) {
        let first_line_parts = first_line.split_whitespace().collect::<Vec<&str>>();

        let method = first_line_parts[0].trim().to_string();

        let (url, query, host) = Self::parse_request_target(&first_line_parts[1]);

        let (protocol, protocol_version) = Self::parse_protocol(&first_line_parts);

        (method, url, query, protocol, protocol_version, host)
    }

    fn parse_protocol(first_line_parts: &Vec<&str>) -> (String, String) {
        let protocol_and_version = first_line_parts[2].trim().split('/').collect::<Vec<&str>>();

        let protocol = protocol_and_version[0].to_string();

        let protocol_version = protocol_and_version[1].to_string();

        (protocol, protocol_version)
    }

    fn parse_request_target(request_target: &str) -> (String, HashMap<String, String>, String) {
        let host = String::new();

        let mut query: HashMap<String, String> = HashMap::new();

        let full_url = request_target.trim().to_string();

        let url_splits = full_url.split('?').collect::<Vec<&str>>();

        let url = url_splits[0].to_string();

        // TODO: handle absolute form
        // let is_absolute_form = url.starts_with("http://");

        if url_splits.len() > 1 {
            let query_string = url_splits[1].to_string();

            if !query_string.is_empty() {
                for param in query_string.split('&') {
                    let parts = param.split('=').collect::<Vec<&str>>();

                    if parts.len() == 2 {
                        let key = parts[0].to_string();
                        let value = parts[1].to_string();
                        query.insert(key, value);
                    }
                }
            }
        }

        (url, query, host)
    }
}