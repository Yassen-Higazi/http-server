use crate::request::Request;
use crate::response::Response;
use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;
use std::vec;

mod request;
mod constants;
mod response;
mod thread_pool;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    // let pool = ThreadPool::new(100);

    loop {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            tokio::spawn(async move { handle_connection(&mut stream).await; });

            // pool.execute(move || {
            //     handle_connection(&mut stream);
            // });
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
            response.status = 200;
            response.status_name = String::from("OK");
        } else if request.url.starts_with("/echo") {
            body = request.url.split("/").collect::<Vec<&str>>()[2..].join("");
        } else if request.url == "/user-agent" {
            let agent = request.headers.get("User-Agent");

            if agent.is_some() {
                body = agent.unwrap().to_string();
            }
        } else {
            response.status = 404;
            response.status_name = String::from("Not Found");
        }

        response.set_body(body, None);

        _stream.write_all(&*response.http()).expect("Failed to send Response to client");

        return;
    }

    _stream.write_all(String::from("HTTP/1.1 500 Internal Server Error/\r\n\r\n").as_bytes()).expect("Failed to send Response to client");
}
