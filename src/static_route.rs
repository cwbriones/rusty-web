use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::io::BufReader;

use hyper;
use hyper::{Response, StatusCode};
use hyper::header::{ContentType, ContentLength};
use futures::{self, Future};

use request::Request;
use handler::Handler;
use errors::{Error, Result};

pub struct StaticRouter {
    root: PathBuf,
}

impl StaticRouter {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        let root = root.into().canonicalize().expect("Path must be valid");
        assert!(root.exists() && root.is_dir());
        println!("Creating static route at {:?}", root);
        StaticRouter {
            root
        }
    }

    fn read_file(&self, req_path: &str) -> Result<Response> {
        // Create the path
        let mut path = PathBuf::new();
        path.push(&self.root);
        path.push(req_path);
        path.canonicalize()?;

        let content_type = path.extension()
            .and_then(::std::ffi::OsStr::to_str)
            .and_then(get_content_type);

        // TODO: Turn not found into 404
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        let mut file = BufReader::new(file);
        let mut body = String::with_capacity(metadata.len() as usize);

        // TODO: This needs to stream and not be sent all at once
        file.read_to_string(&mut body)?;

        let resp = Response::new()
            .with_header(ContentLength(body.len() as u64))
            .with_body(body);
        if let Some(content_type) = content_type {
            Ok(resp.with_header(content_type))
        } else {
            Ok(resp)
        }
    }
}

impl Handler for StaticRouter {
    type IntoFuture = Box<Future<Item=hyper::Response, Error=Error>>;

    fn handle(&self, req: Request) -> Self::IntoFuture {
        let future = futures::future::result(self.read_file(req.path()));
        Box::new(future)
    }
}

fn get_content_type(ext: &str) -> Option<ContentType> {
    match ext {
        "jpeg" | "jpg" => Some(ContentType::jpeg()),
        "png" => Some(ContentType::png()),
        "txt" => Some(ContentType::plaintext()),
        "html" => Some(ContentType::html()),
        "xml" => Some(ContentType::xml()),
        _ => None,
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
