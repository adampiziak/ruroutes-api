#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;

use std::io::Read;

#[get("/")]
fn index() -> &'static str {
   " rutgers link"
}

fn main() {
    let mut resp = reqwest::get("https://www.rust-lang.org").unwrap();
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
    println!("{}", content);
    rocket::ignite().mount("/", routes![index]).launch();
}

fn get_routes() -> Vec<String> {
    vec![]
}
