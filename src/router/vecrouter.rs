use hyper;
use itertools::{EitherOrBoth,Itertools};

use std::path::Path;

use handler::{Handler, BoxHandler, BoxFuture};

struct Node<'p> {
    path: &'p str,
    handler: BoxHandler<BoxFuture<hyper::Response>>,
}

impl<'p> Node<'p> {
    pub fn matches(&self, target: &str) -> bool {
        let path = Path::new(self.path);
        let target = Path::new(target);

        for pair in path.iter().zip_longest(target) {
            if let EitherOrBoth::Both(p, t) = pair {
                if p != t {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

pub struct VecRouter<'p> {
    inner: Vec<Node<'p>>,
}

impl<'p> VecRouter<'p> {
    pub fn new() -> Self {
        VecRouter {
            inner: Vec::new(),
        }
    }

    pub fn insert(&mut self, path: &'p str, handler: Box<Handler<IntoFuture=BoxFuture<hyper::Response>>>) {
        let node = Node {
            path,
            handler,
        };
        self.inner.push(node);
    }

    pub fn route(&self, path: &str) -> Option<&Handler<IntoFuture=BoxFuture<hyper::Response>>> {
        self.inner.iter()
            .find(|&node| node.matches(path))
            .map(|node| &*node.handler)
    }
}
