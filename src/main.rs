#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate reqwest;

mod api_data;

use std::time::Duration;
use std::sync::Mutex;
use std::thread;
use rocket::State;
use api_data::ApiData;

#[get("/api")]
fn api(api: State<ApiData>) -> String {
    let data = api.config.clone();
    data
}

#[get("/")]
fn index() -> String {
    String::from("/")
}

fn main() {
    gather_api_data();
    rocket::ignite().manage(api_data::new()).mount("/", routes![index, api]).launch();
}

fn gather_api_data(api: State<ApiData>) {
   let config_data = Mutex::new(String::new());
   thread::spawn(move || {
       loop {
           let new_data = get_config();
           let mut config = config_data.lock().unwrap();
           *config = new_data.clone();
           thread::sleep(Duration::from_millis(5000));
       }
   });
}

fn get_config() -> String {
    let resp = reqwest::get("http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a=rutgers&terse");
    
    let body: String = resp.unwrap().text().unwrap();
    body
}

