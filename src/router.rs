use futures::{self, Future};
use hyper::{self, Request, Response, StatusCode, Method};

use errors::{Error, Result};
use handler::{Handler, BoxHandler};
use responder;

type BoxFuture<I> = Box<Future<Item=I, Error=Error>>;

pub struct Router<'p> {
    routes: Vec<Route<'p>>
}

pub struct Route<'p> {
    method: Method,
    path: &'p str,
    handler: BoxHandler<BoxFuture<hyper::Response>>,
}

impl<'p> Route<'p> {
    pub fn new(method: Method, path: &'p str, handler: BoxHandler<BoxFuture<hyper::Response>>) -> Self {
        Route {
            method,
            path,
            handler,
        }
    }
}

macro_rules! method_helper {
    ($name:ident, $method:expr) => {
        pub fn $name<H>(self, path: &'p str, handler: H) -> Self
            where
                H: Handler<IntoFuture=BoxFuture<hyper::Response>> + 'static
        {
            self.method($method, path, handler)
        }
    }
}

impl<'p> Router<'p> {
    pub fn new() -> Self {
        Router {
            routes: Vec::new()
        }
    }

    pub fn method<H>(mut self, method: Method, path: &'p str, handler: H) -> Self
        where
            H: Handler<IntoFuture=BoxFuture<hyper::Response>> + 'static
    {
        let handler: Box<Handler<IntoFuture=BoxFuture<_>>> = Box::new(handler);
        let route = Route::new(method, path, handler);
        self.routes.push(route);
        self
    }

    method_helper!(get, Method::Get);
    method_helper!(post, Method::Post);
    method_helper!(put, Method::Put);
    method_helper!(patch, Method::Patch);
    method_helper!(delete, Method::Delete);

    fn route(&self, method: &Method, path: &str) -> &Handler<IntoFuture=BoxFuture<hyper::Response>> {
        let mut path_exists = false;
        for route in &self.routes {
            if route.path == path {
                path_exists = true;
                if route.method == *method {
                    return &*route.handler;
                }
            }
        }

        if path_exists {
            &method_not_allowed
        } else {
            &not_found
        }
    }
}

impl<'p> Handler for Router<'p> {
    type IntoFuture = Box<Future<Item=hyper::Response, Error=hyper::Error>>;

    fn handle(&self, req: hyper::Request) -> Self::IntoFuture {
        let handler = self.route(req.method(), req.path());
        Box::new(handler.handle(req).or_else(translate_error))
    }
}

fn not_found(req: Request) -> Result<hyper::Response> {
    let body = "Path not found: ".to_owned() + req.path();
    Ok(Response::new()
        .with_status(StatusCode::NotFound)
        .with_body(body)
    )
}

fn method_not_allowed(_req: Request) -> Result<hyper::Response> {
    Ok(Response::new()
        .with_status(StatusCode::MethodNotAllowed)
    )
}

fn translate_error(error: Error) -> impl Future<Item=Response, Error=hyper::Error> {
    futures::future::ok(
        Response::new()
            .with_status(StatusCode::InternalServerError)
            .with_body(error.description().to_string())
    )
}
