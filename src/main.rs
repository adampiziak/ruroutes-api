#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;

use std::io::Read;

#[get("/")]
fn index() -> &' str {
    "root"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
