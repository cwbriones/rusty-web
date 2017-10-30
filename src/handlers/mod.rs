use request::Request;
use errors::Result;

pub mod todos;

pub fn index(_req: Request) -> Result<&'static str> {
    Ok("Hello, World!")
}
