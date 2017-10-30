use futures;
use hyper;
use hyper::StatusCode;
use hyper::server::Service;
use futures::Future;
use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

use super::router::Router;
use super::handler::Handler;
use super::request::Request;
use super::errors::Error;

pub struct App<'r> {
    router: Router<'r>,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl<'r> App<'r> {
    pub fn new(router: Router<'r>, pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        App {
            router,
            pool,
        }
    }
}

impl<'a> Service for &'a App<'a> {
    type Request = hyper::Request;
    type Response = hyper::Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let req = Request::new(req, self.pool.clone());
        Box::new(self.router.handle(req).or_else(translate_error))
    }
}

fn translate_error(error: Error) -> impl Future<Item=hyper::Response, Error=hyper::Error> {
    futures::future::ok(
        hyper::Response::new()
            .with_status(StatusCode::InternalServerError)
            .with_body(error.description().to_string())
    )
}
