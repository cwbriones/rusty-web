use diesel::pg::PgConnection;
use futures::{Stream, Future};
use hyper;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use serde::de::DeserializeOwned;
use serde_json;

use super::errors::Error;

// FIXME:
// I don't mind tying this to these third-party crates, but the pool
// functionality shouldn't need to be baked-in to the request.
//
// Can we alter this while still maintaining a consistent error type?

pub struct Request {
    inner: hyper::Request,
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Request {
    pub fn new(inner: hyper::Request, pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Request { inner, pool }
    }

    pub fn body(self) -> hyper::Body {
        self.inner.body()
    }

    pub fn parse_json<T: DeserializeOwned>(self) -> impl Future<Item=T, Error=Error>
    {
        self.inner.body()
            .concat2()
            .from_err::<Error>()
            .and_then(move |chunk| {
                serde_json::from_slice::<T>(&chunk).map_err(Into::into)
            })
    }

    pub fn method(&self) -> &hyper::Method {
        self.inner.method()
    }

    pub fn path(&self) -> &str {
        self.inner.path()
    }

    pub fn pool(&self) -> &Pool<ConnectionManager<PgConnection>> {
        &self.pool
    }
}
