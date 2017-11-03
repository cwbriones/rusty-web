mod vecrouter;

use std::collections::HashMap;

use futures::Future;
use hyper::{self, Response, StatusCode, Method};

use super::errors::{Error, Result};
use super::handler::{Handler, BoxFuture};
use super::request::Request;

use self::vecrouter::VecRouter;

pub trait SubRouter {
    fn empty() -> Self;
    fn insert(&mut self, path: &str, handler: Box<Handler<IntoFuture=BoxFuture<hyper::Response>>>);
    fn route(&self, path: &str) -> Option<&Handler<IntoFuture=BoxFuture<hyper::Response>>>;
}

pub struct Router<R> {
    subrouters: HashMap<Method, R>,
    not_found: Box<Handler<IntoFuture=BoxFuture<hyper::Response>>>,
}

impl Router<VecRouter> {
    pub fn new() -> Self {
        Router {
            subrouters: HashMap::new(),
            not_found: Box::new(default_not_found),
        }
    }
}

macro_rules! method_helper {
    ($name:ident, $method:expr) => {
        pub fn $name<H>(self, path: &str, handler: H) -> Self
            where
                H: Handler<IntoFuture=BoxFuture<hyper::Response>> + 'static
        {
            self.method($method, path, handler)
        }
    }
}

impl<R: SubRouter> Router<R> {
    pub fn method<H>(mut self, method: Method, path: &str, handler: H) -> Self
        where
            H: Handler<IntoFuture=BoxFuture<hyper::Response>> + 'static
    {
        let handler: Box<Handler<IntoFuture=BoxFuture<_>>> = Box::new(handler);

        {
            let subrouter = &mut self.subrouters.entry(method).or_insert_with(|| SubRouter::empty());
            subrouter.insert(path, handler);
        }

        self
    }

    pub fn not_found<H>(mut self, handler: H) -> Self
        where
            H: Handler<IntoFuture=BoxFuture<hyper::Response>> + 'static
    {
        self.not_found = Box::new(handler);
        self
    }

    method_helper!(get, Method::Get);
    method_helper!(post, Method::Post);
    method_helper!(put, Method::Put);
    method_helper!(patch, Method::Patch);
    method_helper!(delete, Method::Delete);

    fn route(&self, method: &Method, path: &str) -> &Handler<IntoFuture=BoxFuture<hyper::Response>> {
        self.subrouters
            .get(method)
            .and_then(|tree| tree.route(path))
            .unwrap_or(&*self.not_found)
    }
}

impl<R: SubRouter> Handler for Router<R> {
    type IntoFuture = Box<Future<Item=hyper::Response, Error=Error>>;

    fn handle(&self, req: Request) -> Self::IntoFuture {
        let handler = self.route(req.method(), req.path());
        Box::new(handler.handle(req))
    }
}

fn default_not_found(req: Request) -> Result<hyper::Response> {
    let body = "Path not found: ".to_owned() + req.path();
    Ok(Response::new()
        .with_status(StatusCode::NotFound)
        .with_body(body)
    )
}
