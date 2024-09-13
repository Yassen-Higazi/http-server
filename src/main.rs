use crate::options::Options;
use crate::response::{ContentType, HttpCode, Response};
use clap::Parser;
use http_server::HttpServer;
use std::io::ErrorKind;
use std::path::PathBuf;

mod request;
mod constants;
mod response;
mod thread_pool;
mod http_server;
mod router;
mod options;

#[tokio::main]
async fn main() {
    let args = Options::parse();

    let server = HttpServer::new(args);

    server
        .define_route("/", |r| {
            let mut response = Response::from(r);

            response.set_body("Hello, World!".to_string(), None);

            response.status = HttpCode::Ok;

            Ok(response)
        })

        .define_route("/user-agent", |r| {
            let mut body = String::new();

            let agent = r.headers.get("User-Agent");

            if agent.is_some() {
                body = agent.unwrap().to_string();
            }

            let mut response = Response::from(r);

            response.set_body(body, None);

            response.status = HttpCode::Ok;

            Ok(response)
        })

        .define_route("/echo/:content", |req| {
            let mut response = Response::from(req);

            if let Some(content) = req.params.get("content") {
                response.set_body(content.clone(), None);
                response.status = HttpCode::Ok;
            } else {
                response.set_body(String::from("{ \"message\": \"Param content is required\" }"), None);
                response.status = HttpCode::BadRequest;
            }

            Ok(response)
        })

        .define_route("/files/:filename", |req| {
            let mut response = Response::from(req);

            if let Some(filename) = req.params.get("filename") {
                let file_path = PathBuf::from(&req.options.files_directory).join(filename);

                println!("{:?}", file_path);

                let content = match std::fs::read_to_string(file_path) {
                    Ok(content) => {
                        response.status = HttpCode::Ok;
                        content
                    }

                    Err(err) => {
                        match err.kind() {
                            ErrorKind::NotFound => {
                                response.status = HttpCode::NotFound;

                                let message = format!("File not found: {}", filename);

                                String::from("{ \"message\": \"".to_string() + message.as_str() + "\" }")
                            }
                            _ => {
                                eprintln!("{:?}", err);

                                response.status = HttpCode::InternalServerError;
                                String::from("{ \"message\": \"Internal Server Error\" }".to_string())
                            }
                        }
                    }
                };

                response.set_body(content, Some(ContentType::OctetStream));
            } else {
                response.set_body(String::from("{ \"message\": \"Param filename is required\" }"), None);
                response.status = HttpCode::BadRequest;
            }

            Ok(response)
        });

    server.listen();
}

