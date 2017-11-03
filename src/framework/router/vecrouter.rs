use hyper;
use itertools::{EitherOrBoth,Itertools};

use std::path::{Path,PathBuf};

use super::SubRouter;
use framework::handler::{Handler, BoxHandler, BoxFuture};

struct Node {
    path: PathBuf,
    handler: BoxHandler<BoxFuture<hyper::Response>>,
}

impl Node {
    pub fn matches(&self, target: &str) -> bool {
        let target = Path::new(target);

        for pair in self.path.iter().zip_longest(target) {
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

pub struct VecRouter {
    inner: Vec<Node>,
}

impl SubRouter for VecRouter {
    fn empty() -> Self {
        VecRouter {
            inner: Vec::new(),
        }
    }

    fn insert(&mut self, path: &str, handler: Box<Handler<IntoFuture=BoxFuture<hyper::Response>>>) {
        let node = Node {
            path: PathBuf::from(path),
            handler,
        };
        self.inner.push(node);
    }

    fn route(&self, path: &str) -> Option<&Handler<IntoFuture=BoxFuture<hyper::Response>>> {
        self.inner.iter()
            .find(|&node| node.matches(path))
            .map(|node| &*node.handler)
    }
}
