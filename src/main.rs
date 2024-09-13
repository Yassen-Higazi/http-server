use crate::response::{HttpCode, Response};
use http_server::HttpServer;

mod request;
mod constants;
mod response;
mod thread_pool;
mod http_server;
mod router;

#[tokio::main]
async fn main() {
    let server = HttpServer::new("localhost", 4221);

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
        });

    server.listen();
}

