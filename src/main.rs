use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");

                let reply = String::from("HTTP/1.1 200 OK\r\n\r\n");

                _stream.write_all(reply.as_bytes()).expect("Failed to send Response to client");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
