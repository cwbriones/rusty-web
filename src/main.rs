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

use app::App;
use hyper::server::Http;

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(App::new())).unwrap();
    println!("Listening on {}", addr);
    server.run().unwrap();
}
