use hyper::{Response, StatusCode};
use hyper::header::ContentLength;
use hyper::header::ContentType;

use serde_json;
use serde::Serialize;
use errors::Result;

pub trait Responder {
    fn respond(self) -> Result<Response>;
}

impl<'a> Responder for &'a str {
    fn respond(self) -> Result<Response> {
        self.to_string().respond()
    }
}

impl Responder for () {
    fn respond(self) -> Result<Response> {
        Ok(Response::new().with_status(StatusCode::Ok))
    }
}

impl Responder for String {
    fn respond(self) -> Result<Response> {
        Ok(Response::new()
            .with_status(StatusCode::Ok)
            .with_header(ContentLength(self.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(self))
    }
}

pub struct Json<T>(T);

impl<T> Responder for Json<T>
    where T: Serialize
{
    fn respond(self) -> Result<Response> {
        let json = serde_json::to_string(&self.0)?;
        let resp = json.respond()?.with_header(ContentType::json());
        Ok(resp)
    }
}

pub struct Gzip<T>(T);

impl<T> Responder for Gzip<T>
    where T: Responder
{
    fn respond(self) -> Result<Response> {
        let inner = self.0.respond();
        inner
    }
}

impl Responder for Response {
    fn respond(self) -> Result<Response> {
        Ok(self)
    }
}

pub struct NotFound;

impl Responder for NotFound {
    fn respond(self) -> Result<Response> {
        Ok(Response::new().with_status(StatusCode::NotFound))
    }
}
