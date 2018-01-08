#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/api")]
fn api() -> &'static str {
    "Welcome to the Rutgers Link API page."
}

#[get("/")]
fn index() -> &'static str {
    "root"
}

fn main() {
    rocket::ignite().mount("/", routes![index, api]).launch();
}
