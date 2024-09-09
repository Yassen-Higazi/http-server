use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::vec;
use crate::request::Request;
use crate::response::Response;

mod request;
mod constants;
mod response;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");

                let mut buffer = vec![0u8; 1024];

                _stream.read(&mut buffer).expect("Could not read client request");

                let request_str = String::from_utf8_lossy(&buffer).to_string();

                let request = Request::new(request_str);
                let mut response = Response::from(&request);

                let mut body = String::from("");

                if request.url == "/" {
                    response.status = 200;
                    response.status_name = String::from("OK");
                } else if request.url.starts_with("/echo")  {
                    body = request.url.split("/").collect::<Vec<&str>>()[2..].join("");
                } else {
                    response.status = 404;
                    response.status_name = String::from("Not Found");
                }

                response.set_body(body, None);

                _stream.write_all(&*response.http()).expect("Failed to send Response to client");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
