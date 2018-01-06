#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;

use std::io::Read;

#[get("/")]
fn index() -> String {
    print_route_config()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

fn print_route_config() -> String {
     let mut resp = reqwest::get("http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfi
g&a=rutgers&r=h").unwrap();
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
    content
}
