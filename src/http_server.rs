use crate::options::Options;
use crate::request::Request;
use crate::response::{HttpCode, Response};
use crate::router::Router;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct HttpServer {
    options: Options,
    pub router: Router,
}

impl HttpServer {
    pub fn new(options: Options) -> HttpServer {
        Self {
            options,
            router: Router::new(),
        }
    }

    pub fn listen(self) {
        let listener = TcpListener::bind(format!("{}:{}", &self.options.host, &self.options.port)).unwrap();

        println!("Server is listening on {}:{}", self.options.host, self.options.port);

        // let pool = ThreadPool::new(500);

        loop {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();


                // pool.execute(move || {
                //     HttpServer::handle_connection(&mut stream)
                // })

                let router = self.router.clone();
                let options = self.options.clone();

                tokio::spawn(async move { handle_connection(&mut stream, router, options).await; });
            }
        }
    }
}

async fn handle_connection(_stream: &mut TcpStream, router: Router, options: Options) {
    println!("accepted new connection from {}", _stream.peer_addr().unwrap());

    let mut buffer = vec![0u8; 1024];

    let bits_read = _stream.read(&mut buffer).expect("Could not read client request");

    let request_str = String::from_utf8_lossy(&&buffer[0..bits_read]).to_string();

    let is_http = request_str.contains("HTTP/1.");

    if is_http {
        let mut request = Request::new(request_str, options);

        let (handler, params) = router.get_handler(&request.method, request.url.as_str());

        let mut response = match handler {
            None => {
                let mut res = Response::new("HTTP".to_string(), "1.1".to_string());

                res.status = HttpCode::NotFound;

                res
            }

            Some(handler) => {
                for (key, position) in params {
                    let mut param = String::new();

                    for (i, char) in request.url.chars().enumerate() {
                        if i as i32 >= position {
                            if char == '/' { break; }

                            param.push(char);
                        }
                    }

                    request.params.insert(key.clone(), param);
                }

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

        if let Some(compression) = request.headers.get("Accept-Encoding") {
            if compression.contains("gzip") {
                response.set_header("Content-Encoding".to_string(), "gzip".to_string());
            }
        }

        response.write_to(_stream, None);

        return;
    }

    _stream.write_all(String::from("HTTP/1.1 500 Internal Server Error/\r\n\r\n").as_bytes()).expect("Failed to send Response to client");
}