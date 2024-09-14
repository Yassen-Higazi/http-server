use crate::options::Options;
use crate::response::{ContentType, HttpCode, Response};
use clap::Parser;
use http_server::HttpServer;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
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

    server.router
        .get("/", |r| {
            let mut response = Response::from(r);

            response.set_body_string("Hello, World!".to_string(), None);

            response.status = HttpCode::Ok;

            Ok(response)
        })

        .get("/user-agent", |r| {
            let mut body = String::new();

            let agent = r.headers.get("User-Agent");

            if agent.is_some() {
                body = agent.unwrap().to_string();
            }

            let mut response = Response::from(r);

            response.set_body_string(body, None);

            response.status = HttpCode::Ok;

            Ok(response)
        })

        .get("/echo/:content", |req| {
            let mut response = Response::from(req);

            if let Some(content) = req.params.get("content") {
                response.set_body_string(content.clone(), None);
                response.status = HttpCode::Ok;
            } else {
                response.set_body_string(String::from("{ \"message\": \"Param content is required\" }"), None);
                response.status = HttpCode::BadRequest;
            }

            Ok(response)
        })

        .get("/files/:filename", |req| {
            let mut response = Response::from(req);

            if let Some(filename) = req.params.get("filename") {
                let file_path = PathBuf::from(&req.server_options.files_directory).join(filename);

                println!("{:?}", file_path);

                match std::fs::read_to_string(file_path) {
                    Ok(content) => {
                        response.status = HttpCode::Ok;
                        response.set_body_string(content, Some(ContentType::OctetStream));
                    }

                    Err(err) => {
                        match err.kind() {
                            ErrorKind::NotFound => {
                                response.status = HttpCode::NotFound;

                                let message = format!("File not found: {}", filename);

                                let content = String::from("{ \"message\": \"".to_string() + message.as_str() + "\" }");

                                response.set_json_body(content);
                            }
                            _ => {
                                eprintln!("{:?}", err);

                                response.status = HttpCode::InternalServerError;

                                let content = String::from("{ \"message\": \"Internal Server Error\" }".to_string());

                                response.set_json_body(content);
                            }
                        }
                    }
                };
            } else {
                response.set_body_string(String::from("{ \"message\": \"Param filename is required\" }"), None);
                response.status = HttpCode::BadRequest;
            }

            Ok(response)
        })

        .post("/files/:filename", |req| {
            let mut response = Response::from(req);

            if let Some(filename) = req.params.get("filename") {
                let file_path = PathBuf::from(&req.server_options.files_directory).join(filename);

                println!("{:?}", file_path);

                let result = OpenOptions::new().write(true).create(true).open(file_path);

                match result {
                    Ok(mut file) => {
                        match file.write(&req.body.as_bytes()) {
                            Ok(_) => {
                                response.status = HttpCode::Created;
                            }
                            Err(err) => {
                                eprintln!("{:?}", err);

                                response.status = HttpCode::InternalServerError;

                                let content = String::from("{ \"message\": \"Internal Server Error\" }".to_string());

                                response.set_json_body(content);
                            }
                        }
                    }

                    Err(err) => {
                        println!("{:?}", err);

                        match err.kind() {
                            _ => {
                                eprintln!("{:?}", err);

                                response.status = HttpCode::InternalServerError;

                                let content = String::from("{ \"message\": \"Internal Server Error\" }".to_string());

                                response.set_json_body(content);
                            }
                        }
                    }
                };
            } else {
                response.set_body_string(String::from("{ \"message\": \"Param filename is required\" }"), None);
                response.status = HttpCode::BadRequest;
            }

            Ok(response)
        });

    server.listen();
}

