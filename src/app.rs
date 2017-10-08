use hyper;
use router::Router;
use hyper::server::Service;
use futures::Future;

use handler::Handler;

pub struct App<'r> {
    router: Router<'r>,
}

impl<'r> App<'r> {
    pub fn new(router: Router<'r>) -> Self {
        App {
            router
        }
    }
}

impl<'a> Service for &'a App<'a> {
    type Request = hyper::Request;
    type Response = hyper::Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.
        self.router.handle(req)
    }
}
