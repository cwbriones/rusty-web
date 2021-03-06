#![feature(conservative_impl_trait)]

extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate mime;
extern crate itertools;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;

mod schema;
mod models;
mod handlers;

mod framework;

use hyper::server::Http;
use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;

use framework::App;
use framework::Router;

use std::env;

fn initialize_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(&database_url[..]);
    r2d2::Pool::new(config, manager).expect("Failed to create pool.")
}

fn main() {
    dotenv().ok();
    let addr = "127.0.0.1:8080".parse().unwrap();
    let router =
        Router::new()
            .get("/", handlers::index)
            .post("/todos", handlers::todos::create)
            .get("/todos", handlers::todos::list);

    let pool = initialize_pool();
    let app = App::new(router, pool);

    let server = Http::new().bind(&addr, move || Ok(app.clone())).unwrap();
    println!("Listening on {}", addr);
    server.run().unwrap();
}
