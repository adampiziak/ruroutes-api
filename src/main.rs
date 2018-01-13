#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;

mod api_data;

// use std::time::Duration;
// use std::sync::Mutex;           
// use std::thread;
use rocket::State;
use rocket::response::Redirect;
use api_data::ApiData;


#[get("/api")]
fn api(api: State<ApiData>) -> String {
    get_config(api)
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/api")
}

fn main() {
    let mut api_state = api_data::new();
    rocket::ignite()
        .manage(api_state)
        .mount("/", routes![index, api])
        .launch();
}

fn get_config(state: State<ApiData>) -> String {
    let mut config = String::new();
    if state.config.read().unwrap().len() < 1 {
        let resp = reqwest::get("http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a=rutgers&terse");
        config = resp.unwrap().text().unwrap();
        let mut n = state.config.write().unwrap();
        *n = config.clone();
        println!("Returning new");
    } else {
        println!("Returning old");
        return state.config.read().unwrap().clone();
    }
    config
}

