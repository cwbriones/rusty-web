use hyper::{Response, StatusCode};
use hyper::header::ContentLength;
use hyper::header::ContentType;

use serde_json;
use serde::Serialize;

use super::errors::{Error,ErrorKind,Result};

pub trait Responder {
    fn respond(self) -> Response;
}

impl Responder for Error {
    fn respond(self) -> Response {
        let status = match *self.kind() {
            ErrorKind::Serde(_) => StatusCode::BadRequest,
            _ => StatusCode::InternalServerError,
        };
        status.respond()
    }
}

impl<'a> Responder for &'a str {
    fn respond(self) -> Response {
        self.to_string().respond()
    }
}

impl Responder for () {
    fn respond(self) -> Response {
        Response::new().with_status(StatusCode::Ok)
    }
}

impl Responder for String {
    fn respond(self) -> Response {
        Response::new()
            .with_status(StatusCode::Ok)
            .with_header(ContentLength(self.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(self)
    }
}

impl Responder for Response {
    fn respond(self) -> Response {
        self
    }
}

impl Responder for StatusCode {
    fn respond(self) -> Response {
        let reason = self.canonical_reason().unwrap_or("");
        let resp = Response::new()
            .with_status(self)
            .with_body(reason);
        resp
    }
}

pub trait TryResponder {
    fn try_respond(self) -> Result<Response>;
}

pub struct Json<T>(pub T);

impl<T> TryResponder for Json<T>
    where T: Serialize
{
    fn try_respond(self) -> Result<Response> {
        let json = serde_json::to_string(&self.0)?;
        let resp = json.respond().with_header(ContentType::json());
        Ok(resp)
    }
}

impl<T: TryResponder> Responder for T
{
    fn respond(self) -> Response {
        self.try_respond()
            .unwrap_or_else(Responder::respond)
    }
}
