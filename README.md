[![progress-banner](https://backend.codecrafters.io/progress/http-server/b03eacf4-200b-4382-a86f-31a42765ec3e)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

## My take on [codecrafters.io](https://codecrafters.io) Build Your own http server challenge using Rust

> [HTTP](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol) is the
> protocol that powers the web. In this challenge, you'll build a HTTP/1.1 server
> that is capable of serving multiple clients.

**Note**: If you'RE viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

## Run

To run the application simply run

```shell
./run.sh
```

And you can test it by calling "http://localhost:4221"

```shell
 curl -v http://localhost:4221/
```

### new Routes

To define new routes edit the code in the [main file](src/main.rs) and use `server.router` to define new routes

example:

```rust

server.router.get("/echo/:content", | req, res| {
   if let Some(content) = req.params.get("content") {
      res.set_body_string(content.clone(), None);
      res.status = HttpCode::Ok;
   } else {
      res.set_body_string(String::from("{ \"message\": \"Param content is required\" }"), None);
      res.status = HttpCode::BadRequest;
   }
   
   Ok(())
});
```