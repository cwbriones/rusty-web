use futures::{IntoFuture, Future};
use hyper;

use super::errors::Error;
use super::responder::Responder;
use super::request::Request;

pub type BoxFuture<I> = Box<Future<Item=I, Error=Error>>;
pub type BoxHandler<I> = Box<Handler<IntoFuture=I>>;

pub trait Handler {
    type IntoFuture: IntoFuture;

    fn handle(&self, req: Request) -> Self::IntoFuture;
}

impl<F, T> Handler for F
    where F: Fn(Request) -> T,
          T: IntoFuture,
          <T as IntoFuture>::Future: 'static,
          <T as IntoFuture>::Item: Responder,
          <T as IntoFuture>::Error: Into<Error> + 'static
{
    type IntoFuture = Box<Future<Item=hyper::Response, Error=Error>>;

    fn handle(&self, req: Request) -> Self::IntoFuture {
        Box::new(self(req)
                 .into_future()
                 .map_err(Into::into)
                 .map(|r| r.respond())
        )
    }
}

