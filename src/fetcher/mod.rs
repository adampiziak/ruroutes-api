extern crate redis;
extern crate reqwest;

use redis::{Client, Commands};

use std::{thread, time};

pub fn start() {
    thread::spawn(|| {
        // Open connection to Redis Database
        let client = Client::open("redis://localhost:6379").unwrap();
        let conn = client.get_connection().unwrap();

        // Gather data from nextBus API at most 60 times per minute
        loop {
            // Make request to nextBus servers
            let request = reqwest::get("http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a=rutgers&terse");
            let config = request.unwrap().text().unwrap();
            
            // Set config in Redis Database
            let _: () = conn.set("config", config).unwrap();

            // Sleep 1000 milliseconds to set max rate at 60 requests/min
            thread::sleep(time::Duration::from_millis(1000));
        }
    });
}

