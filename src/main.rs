#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;

#[get("/api")]
fn api() -> &'static str {
    "Welcome to the Rutgers Link API page."
}

#[get("/")]
fn index() -> String {
    get_config()
}

fn main() {
    rocket::ignite().mount("/", routes![index, api]).launch();
}

fn get_config() -> String {
    let resp = reqwest::get("http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a=rutgers&terse");
    
    let body: String = resp.unwrap().text().unwrap();
    body
}

