use crate::request::Request;
use crate::response::{HttpCode, Response};
use std::fmt::Display;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct HttpServer {
    pub port: u16,
    pub host: String,
}

impl HttpServer {
    pub fn new(host: &str, port: u16) -> HttpServer {
        Self {
            port,
            host: host.to_string(),
        }
    }

    pub fn listen(self) {
        let listener = TcpListener::bind(format!("{}:{}", &self.host, &self.port)).unwrap();

        println!("Server is listening on {}:{}", self.host, self.port);

        // let pool = ThreadPool::new(500);

        loop {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();


                // pool.execute(move || {
                //     HttpServer::handle_connection(&mut stream)
                // })

                tokio::spawn(async move { handle_connection(&mut stream).await; });
            }
        }
    }
}

async fn handle_connection(_stream: &mut TcpStream) {
    println!("accepted new connection from {}", _stream.peer_addr().unwrap());

    let mut buffer = vec![0u8; 1024];

    _stream.read(&mut buffer).expect("Could not read client request");

    let request_str = String::from_utf8_lossy(&buffer).to_string();

    let is_http = request_str.contains("HTTP/1.");

    if is_http {
        let request = Request::new(request_str);
        let mut response = Response::from(&request);

        let mut body = String::from("");

        if request.url == "/" {
            response.status = HttpCode::Ok
        } else if request.url.starts_with("/echo") {
            body = request.url.split("/").collect::<Vec<&str>>()[2..].join("");
        } else if request.url == "/user-agent" {
            let agent = request.headers.get("User-Agent");

            if agent.is_some() {
                body = agent.unwrap().to_string();
            }
        } else {
            response.status = HttpCode::NotFound;
        }

        response.write_to(_stream, Some(body));

        return;
    }

    _stream.write_all(String::from("HTTP/1.1 500 Internal Server Error/\r\n\r\n").as_bytes()).expect("Failed to send Response to client");
}