use http_server::HttpServer;
use std::io::{Read, Write};

mod request;
mod constants;
mod response;
mod thread_pool;
mod http_server;
mod router;

#[tokio::main]
async fn main() {
    let server = HttpServer::new("localhost", 4221);

    server.listen();
}

