use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::vec;

mod request;
mod constants;

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

                let request = request::Request::new(request_str);

                let mut reply = String::from("HTTP/1.1 200 OK\r\n\r\n");

                if request.url != "/" {
                    reply = String::from("HTTP/1.1 404 Not Found\r\n\r\n");
                }

                _stream.write_all(reply.as_bytes()).expect("Failed to send Response to client");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
