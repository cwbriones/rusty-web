use std::collections::HashMap;

use futures::Future;
use hyper::{self, Response, StatusCode, Method};

use errors::{Error, Result};
use handler::{Handler, BoxFuture};
use request::Request;

use self::vecrouter::VecRouter;

mod vecrouter;
// mod tree;

pub struct Router<'p> {
    trees: HashMap<Method, VecRouter<'p>>,
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
            trees: HashMap::new(),
        }
    }

    pub fn method<H>(mut self, method: Method, path: &'p str, handler: H) -> Self
        where
            H: Handler<IntoFuture=BoxFuture<hyper::Response>> + 'static
    {
        let handler: Box<Handler<IntoFuture=BoxFuture<_>>> = Box::new(handler);

        {
            let subrouter = &mut self.trees.entry(method).or_insert_with(|| VecRouter::new());
            subrouter.insert(path, handler);
        }
        // match self.trees.entry(method) {
        //     Entry::Vacant(entry) => { entry.insert(TreeNode::new(path, handler)); },
        //     Entry::Occupied(mut tree) => tree.get_mut().insert(path, handler),
        // }

        self
    }

    method_helper!(get, Method::Get);
    method_helper!(post, Method::Post);
    method_helper!(put, Method::Put);
    method_helper!(patch, Method::Patch);
    method_helper!(delete, Method::Delete);

    fn route(&self, method: &Method, path: &str) -> &Handler<IntoFuture=BoxFuture<hyper::Response>> {
        self.trees
            .get(method)
            .and_then(|tree| tree.route(path))
            .unwrap_or(&not_found)
    }
}

impl<'p> Handler for Router<'p> {
    type IntoFuture = Box<Future<Item=hyper::Response, Error=Error>>;

    fn handle(&self, req: Request) -> Self::IntoFuture {
        let handler = self.route(req.method(), req.path());
        Box::new(handler.handle(req))
    }
}

fn not_found(req: Request) -> Result<hyper::Response> {
    let body = "Path not found: ".to_owned() + req.path();
    Ok(Response::new()
        .with_status(StatusCode::NotFound)
        .with_body(body)
    )
}
