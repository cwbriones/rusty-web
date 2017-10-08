use std::ascii::AsciiExt;

use futures::{self, Future, IntoFuture, Stream};
use hyper::{self, Request, Response, StatusCode, Method};
use hyper::header::{ContentLength};

use handler::{Handler, BoxHandler};
use responder::{self, Responder};
use errors::{Error, Result};

type BoxFuture<I> = Box<Future<Item=I, Error=Error>>;

pub struct Router;

impl Router {
    fn route(&self, method: &Method, path: &str) -> BoxHandler<BoxFuture<hyper::Response>> {
        use self::Method::*;

        match (method.clone(), path) {
            (Get,  "/") => Box::new(index),
            (Post, "/echo") => Box::new(echo),
            _ => Box::new(not_found),
        }
    }
}

impl Handler for Router {
    type IntoFuture = Box<Future<Item=hyper::Response, Error=hyper::Error>>;

    fn handle(&self, req: hyper::Request) -> Self::IntoFuture {
        let handler = self.route(req.method(), req.path());
        Box::new(handler.handle(req).or_else(translate_error))
    }
}

fn translate_error(error: Error) -> impl Future<Item=Response, Error=hyper::Error> {
    futures::future::ok(
        Response::new()
            .with_status(StatusCode::InternalServerError)
            .with_body(error.description().to_string())
    )
}

fn index(_req: Request) -> Result<&'static str> {
    Ok("Hello, World!")
}

fn not_found(_req: Request) -> Result<responder::NotFound> {
    Ok(responder::NotFound)
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
