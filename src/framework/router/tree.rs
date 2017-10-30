use handler::{Handler, BoxHandler, BoxFuture};
use hyper;

pub(super) struct TreeNode<'p> {
    path: &'p str,
    handler: Option<BoxHandler<BoxFuture<hyper::Response>>>,
    children: Vec<TreeNode<'p>>, // Children are sorted by priority, which is the number of children
    priority: u32,
}

impl<'p> TreeNode<'p> {
    pub fn new(path: &'p str, handler: BoxHandler<BoxFuture<hyper::Response>>) -> Self {
        TreeNode {
            path,
            handler: Some(handler),
            children: Vec::new(),
            priority: 0,
        }
    }

    pub fn insert(&mut self, path: &'p str, handler: BoxHandler<BoxFuture<hyper::Response>>) {
        self.priority += 1;

        let prefix = get_prefix(self.path, path);

        if prefix.len() == 1 {
            // They only share '/'
        }
        if prefix.len() < self.path.len() {
            // split this node
        }
        // Add a new child directly
        let rest = &path[self.path.len()..];
        self.insert_child(rest, handler); 
    }

    fn insert_child(&mut self, mut path: &'p str, handler: BoxHandler<BoxFuture<hyper::Response>>) {
        let mut node = self;

        loop {
            if path == "/" || path == "" {
                // The path is already the root, we should insert here
                let child = TreeNode::new(path, handler);
                node.children.push(child);
                return;
            }
            path = &path[node.path.len()..];
        }
    }

    pub fn get(&self, path: &str) -> Option<&Handler<IntoFuture=BoxFuture<hyper::Response>>> {
        if path == "/" {
            // We're at the correct node
            return match self.handler {
                Some(ref h) => Some(&**h),
                None => None,
            };
        }
        if !path.starts_with(self.path) {
            return None;
        }
        let rest = &path[self.path.len()..];

        if let Some(c) = rest.chars().next() {
            for child in &self.children {
                if child.path.starts_with(c) {
                    return child.get(rest);
                }
            }
        }
        None
    }

    fn increment_priority(&mut self, idx: usize) {
        assert!(!self.children.is_empty());

        let priority = {
            let child = &mut self.children[idx];
            child.priority += 1;
            child.priority
        };

        for j in idx..0 {
            if self.children[j].priority >= priority {
                break;
            }
            self.children.swap(j, j + 1);
        }
    }
}

impl<'p> ::std::fmt::Debug for TreeNode<'p> {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let handler = if let Some(_) = self.handler {
            "YES"
        } else {
            "NO"
        };
        write!(formatter, "TreeNode(path='{}', priority={}, handler={}, children=[", self.path, self.priority, handler)?;
        for child in &self.children {
            write!(formatter, "{:?}, ", child)?;
        }
        write!(formatter, "])")
    }
}

fn get_prefix<'p>(shorter: &'p str, longer: &'p str) -> &'p str {
    if shorter.len() > longer.len() {
        return get_prefix(longer, shorter);
    }
    let mut idx = 0;
    for ((i, a), (_, b)) in shorter.char_indices().zip(longer.char_indices()) {
        if a != b {
            break;
        }
        idx = i;
    }
    &shorter[..idx]
}

// alternate construction
//
// pub enum Tree<'p> {
//     Leaf(LeafNode<'p>),
//     Inner(InnerNode<'p>),
// }
//
// impl<'p> Tree<'p> {
//     fn priority(&self) -> u32 {
//         match *self {
//             Tree::Leaf(ref n) => n.priority,
//             Tree::Inner(ref n) => n.priority,
//         }
//     }
//
//     fn path(&self) -> &Path {
//         match *self {
//             Tree::Leaf(ref n) => n.path,
//             Tree::Inner(ref n) => n.path,
//         }
//     }
//
//     fn children(&self) -> Option<&[Tree]> {
//         match *self {
//             Tree::Leaf(ref n) => None,
//             Tree::Inner(ref n) => Some(&n.children[..]),
//         }
//     }
// }
//
// pub struct LeafNode<'p> {
//     path: &'p Path,
//     handler: BoxHandler<BoxFuture<hyper::Response>>,
//     priority: u32,
// }
//
// pub struct InnerNode<'p> {
//     path: &'p Path,
//     handler: Option<BoxHandler<BoxFuture<hyper::Response>>>,
//     priority: u32,
//     children: Vec<Tree<'p>>,
// }
