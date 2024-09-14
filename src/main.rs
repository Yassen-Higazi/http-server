use crate::response::{ContentType, HttpCode};
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
    let server = HttpServer::new();

    server.router
        .get("/", |req, res| {
            res.set_body_string("Hello, World!".to_string(), None);

            res.status = HttpCode::Ok;

            Ok(())
        })

        .get("/user-agent", |req, res| {
            let mut body = String::new();

            let agent = req.headers.get("User-Agent");

            if agent.is_some() {
                body = agent.unwrap().to_string();
            }

            res.set_body_string(body, None);

            res.status = HttpCode::Ok;

            Ok(())
        })

        .get("/echo/:content", |req, res| {
            if let Some(content) = req.params.get("content") {
                res.set_body_string(content.clone(), None);
                res.status = HttpCode::Ok;
            } else {
                res.set_body_string(String::from("{ \"message\": \"Param content is required\" }"), None);
                res.status = HttpCode::BadRequest;
            }

            Ok(())
        })

        .get("/files/:filename", |req, res| {
            if let Some(filename) = req.params.get("filename") {
                let file_path = PathBuf::from(&req.server_options.files_directory).join(filename);

                println!("{:?}", file_path);

                match std::fs::read_to_string(file_path) {
                    Ok(content) => {
                        res.status = HttpCode::Ok;
                        res.set_body_string(content, Some(ContentType::OctetStream));
                    }

                    Err(err) => {
                        match err.kind() {
                            ErrorKind::NotFound => {
                                res.status = HttpCode::NotFound;

                                let message = format!("File not found: {}", filename);

                                let content = String::from("{ \"message\": \"".to_string() + message.as_str() + "\" }");

                                res.set_json_body(content);
                            }
                            _ => {
                                eprintln!("{:?}", err);

                                res.status = HttpCode::InternalServerError;

                                let content = String::from("{ \"message\": \"Internal Server Error\" }".to_string());

                                res.set_json_body(content);
                            }
                        }
                    }
                };
            } else {
                res.set_body_string(String::from("{ \"message\": \"Param filename is required\" }"), None);
                res.status = HttpCode::BadRequest;
            }

            Ok(())
        })

        .post("/files/:filename", |req, res| {
            if let Some(filename) = req.params.get("filename") {
                let file_path = PathBuf::from(&req.server_options.files_directory).join(filename);

                println!("{:?}", file_path);

                let result = OpenOptions::new().write(true).create(true).open(file_path);

                match result {
                    Ok(mut file) => {
                        match file.write(&req.body.as_bytes()) {
                            Ok(_) => {
                                res.status = HttpCode::Created;
                            }
                            Err(err) => {
                                eprintln!("{:?}", err);

                                res.status = HttpCode::InternalServerError;

                                let content = String::from("{ \"message\": \"Internal Server Error\" }".to_string());

                                res.set_json_body(content);
                            }
                        }
                    }

                    Err(err) => {
                        println!("{:?}", err);

                        match err.kind() {
                            _ => {
                                eprintln!("{:?}", err);

                                res.status = HttpCode::InternalServerError;

                                let content = String::from("{ \"message\": \"Internal Server Error\" }".to_string());

                                res.set_json_body(content);
                            }
                        }
                    }
                };
            } else {
                res.set_body_string(String::from("{ \"message\": \"Param filename is required\" }"), None);
                res.status = HttpCode::BadRequest;
            }

            Ok(())
        });

    server.listen();
}

