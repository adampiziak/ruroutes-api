extern crate redis;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use redis::{Client, Commands};

use std::{thread, time};

pub fn start() {
    thread::spawn(|| {
        // Open connection to Redis Database
        let client = Client::open("redis://localhost:6379").unwrap();
        let conn = client.get_connection().unwrap();
        
        // Gather data from nextBus API at most 60 times per minute
        loop {
            println!("\nFetching new data...");
            
            // let config = request_config();
            // let predictions = request_predictions();
            let list = request_routes();

            for route in list.route {
                println!("{}", route.title);
                thread::sleep(time::Duration::from_millis(50));
            }
            
            // Set data in Redis Database
            // let _: () = conn.set("config", config).unwrap();
            // let _: () = conn.set("predictions", predictions).unwrap();

            // Sleep 1000 milliseconds to set max rate at 60 requests/min
            thread::sleep(time::Duration::from_millis(2000));
        }
    });
}

#[derive(Serialize, Deserialize)]
struct RouteList {
    route: Vec<Route>,
    copyright: String
}

#[derive(Serialize, Deserialize)]
struct Route {
    title: String,
    tag: String
}

fn request_routes() -> RouteList {
    let start = time::Instant::now();
    let request = reqwest::get("http://webservices.nextbus.com/service/publicJSONFeed?command=routeList&a=rutgers");
    println!("duration: {:?}", start.elapsed());
    let routes = request.unwrap().text().unwrap();


    let list: RouteList = serde_json::from_str(&routes).expect("JSON deserialization FAILED");



    list
}

fn request_config() -> String {
    // Make request to nextBus servers
    let request = reqwest::get("http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a=rutgers&terse");
    let config = request.unwrap().text().unwrap();

    config
 }

fn request_predictions() -> String {
    let request = reqwest::get("http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a=rutgers&terse");
    let predictions = request.unwrap().text().unwrap();
    
    predictions
}
        
