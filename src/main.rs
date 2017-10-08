#![feature(conservative_impl_trait)]

extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate mime;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

mod app;
mod errors;
mod responder;
mod router;
mod handler;
mod static_route;

use std::ascii::AsciiExt;

use futures::{Future, Stream};
use hyper::server::Http;
use hyper::{Request, Response};
use hyper::header::{ContentLength};

use app::App;
use errors::{Result};
use router::Router;

fn index(_req: Request) -> Result<&'static str> {
    Ok("Hello, World!")
}

fn echo(req: Request) -> impl Future<Item=Response, Error=hyper::Error> {
    let body = req.body().map(|chunk| {
        let uppered = chunk.iter()
            .map(|byte| byte.to_ascii_uppercase())
            .collect::<Vec<u8>>();
        ::hyper::Chunk::from(uppered)
    }).concat2();

    body.map(|data| {
        Response::new()
           .with_header(ContentLength(data.len() as u64))
           .with_body(data)
    })
}

fn json(_req: Request) -> Result<responder::Json<Foo>> {
    let foo = Foo { one: "one".into(), two: 2 };
    Ok(responder::Json(foo))
}

#[derive(Serialize, Deserialize)]
pub struct Foo {
    pub one: String,
    pub two: usize,
}

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let static_router = static_route::StaticRouter::new("./static");
    let router =
        Router::new()
            .get("/", index)
            .post("/echo", echo)
            .get("/json", json)
            .get("/static", static_router);

    let app = App::new(router);

    let server = Http::new().bind(&addr, move || Ok(&app)).unwrap();
    println!("Listening on {}", addr);
    server.run().unwrap();
}
