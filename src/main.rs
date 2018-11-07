#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;

extern crate r2d2;

extern crate r2d2_redis;
extern crate redis;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method};
use std::io::Cursor;


mod data_fetcher;
mod api_worker;
mod routes;
mod pool;
mod model;
mod lookup_table;

fn main() {
    data_fetcher::start(); // Fetch data from nextBus API
    api_worker::start();   // Restructure and simplify data in Redis store
    rocket::ignite()       // Create server, mount routes, manage redis pool and start!
        .attach(CORS())
        .mount("/", routes::create())
        .manage(pool::init_pool())
        .launch();
}

struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:8080"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}

