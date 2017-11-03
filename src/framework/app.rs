use diesel::pg::PgConnection;
use futures::{self,Future,IntoFuture};
use hyper::{self,StatusCode};
use hyper::server::Service;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

use super::handler::Handler;
use super::request::Request;
use super::errors::Error;

use std::sync::Arc;

pub struct App<H: Handler> {
    handler: Arc<H>,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl<H: Handler> Clone for App<H> {
    fn clone(&self) -> Self {
        App {
            handler: self.handler.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl<H: Handler> App<H> {
    pub fn new(handler: H, pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        let handler = Arc::new(handler);
        App {
            handler,
            pool,
        }
    }
}

impl<H: Handler> Service for App<H>
    where
        H: Handler + 'static,
        <<H as Handler>::IntoFuture as IntoFuture>::Error: Into<Error> + 'static
{
    type Request = hyper::Request;
    type Response = hyper::Response;
    type Error = hyper::Error;

    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let req = Request::new(req, self.pool.clone());
        let future = self.handler.handle(req).into_future();

        Box::new(future.map_err(Into::into).or_else(translate_error))
    }
}

fn translate_error(error: Error) -> impl Future<Item=hyper::Response, Error=hyper::Error> {
    futures::future::ok(
        hyper::Response::new()
            .with_status(StatusCode::InternalServerError)
            .with_body(error.description().to_string())
    )
}
