#![feature(conservative_impl_trait)]

extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate error_chain;

mod app;
mod errors;
mod responder;
mod router;
mod handler;

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

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let router =
        Router::new()
            .get("/", index)
            .post("/echo", echo);
    let app = App::new(router);

    let server = Http::new().bind(&addr, move || Ok(&app)).unwrap();
    println!("Listening on {}", addr);
    server.run().unwrap();
}
