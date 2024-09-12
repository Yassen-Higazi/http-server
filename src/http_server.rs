use regex::Regex;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::request::Request;
use crate::response::{HttpCode, Response};
use crate::router::{RequestHandler, Router};

pub struct HttpServer {
    pub port: u16,
    pub host: String,
    router: Router,
}

impl HttpServer {
    pub fn new(host: &str, port: u16) -> HttpServer {
        Self {
            port,
            router: Router::new(),
            host: host.to_string(),
        }
    }

    pub fn define_route(&self, path: &str, handle: RequestHandler) -> &Self {
        self.router.define_route(path.to_string(), handle);

        self
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

                let router = self.router.clone();

                tokio::spawn(async move { handle_connection(&mut stream, router).await; });
            }
        }
    }
}

async fn handle_connection(_stream: &mut TcpStream, router: Router) {
    println!("accepted new connection from {}", _stream.peer_addr().unwrap());
    let params_regex = Regex::new(r":([a-z0-9_]+):").unwrap();

    let mut buffer = vec![0u8; 1024];

    _stream.read(&mut buffer).expect("Could not read client request");

    let request_str = String::from_utf8_lossy(&buffer).to_string();

    let is_http = request_str.contains("HTTP/1.");

    if is_http {
        let mut request = Request::new(request_str);

        let names = request.url.split(":").filter(|c| !c.contains("/")).collect::<Vec<&str>>();
        
        println!("Names: {:?}", names);

        for (i, value) in params_regex.captures_iter(&*request.url).enumerate() {
            println!("Values: {}-{:?}", i, value);
            request.params.insert(names.get(i).unwrap().to_string(), value[1].trim().to_string());
        }

        println!("Request: {:?}", request.params);

        let handler = router.get_handler(request.url.as_str());

        let mut response = match handler {
            None => {
                let mut res = Response::new("HTTP".to_string(), "1.1".to_string());

                res.status = HttpCode::NotFound;

                res
            }

            Some(handler) => {
                let result = handler(&request);

                match result {
                    Ok(res) => {
                        res
                    }

                    Err(err) => {
                        eprintln!("{}", err);

                        let mut res = Response::new("HTTP".to_string(), "1.1".to_string());

                        res.status = HttpCode::InternalServerError;

                        res
                    }
                }
            }
        };

        response.write_to(_stream, None);

        return;
    }

    _stream.write_all(String::from("HTTP/1.1 500 Internal Server Error/\r\n\r\n").as_bytes()).expect("Failed to send Response to client");
}