use redis::{Client, Connection, Commands};
use std::{thread, time};
use reqwest;
use serde_json;
//use std::error::Error;

use api_worker;

static REQUEST_ROUTES_LIST: &'static str = "http://webservices.nextbus.com/service/publicJSONFeed?command=routeList&a=rutgers";
static REQUEST_PREDICTIONS: &'static str = "http://webservices.nextbus.com/service/publicJSONFeed?a=rutgers&command=predictionsForMultiStops";

pub fn start() {
    initialize();
    spawn_1sec_updater();
    spawn_30sec_updater();
    spawn_5min_updater();
}

// Fetches data that needs to updated semi-frequently
fn spawn_30sec_updater() {
    thread::spawn(|| {
        let conn = get_redis_connection();

        loop {
            update_predictions(&conn);
            sleep(30000);
        }
    });
}

// Fetches data thats needs to be updated very frequently (location of buses)
fn spawn_1sec_updater() {
    thread::spawn(|| {

        loop {
            sleep(1000);
        }
    });
}

fn spawn_5min_updater() {
    thread::spawn(|| {
        let conn = get_redis_connection();

        loop {
            update_routes_list(&conn);
            update_route_config(&conn);
            sleep(1000*60*5);
        }
    });
}

fn initialize() {
    update_route_config(&get_redis_connection());
    api_worker::create_predictions_query();
    api_worker::store_route_order();
}
// Return new Redis connection
fn get_redis_connection() -> Connection {
    let client = Client::open("redis://localhost:6379").unwrap();
    client.get_connection().expect("Unable to open redis connection")
}

fn sleep(duration: u64) {
    thread::sleep(time::Duration::from_millis(duration));
}

#[derive(Serialize, Deserialize)]
struct RouteList {
    route: Vec<RouteName>
}

#[derive(Serialize, Deserialize)]
struct RouteName {
    title: String,
    tag: String
}

// Will fetch list of routes from NextBus API, and update Redis store
fn update_routes_list(conn: &Connection)  {
    let request = reqwest::get(REQUEST_ROUTES_LIST);
    let routes = request.unwrap().text().unwrap();

    // Create struct with same structure as json
    let new_list: RouteList = serde_json::from_str(&routes).unwrap();

    // Extract tag names
    let mut new_routes: Vec<String> = Vec::new();
    for route in new_list.route {
        new_routes.push(route.tag);
    }
    
    // Retrieve old routes from redis
    let old_routes: Vec<String> = conn.lrange("route_names", 0, -1).unwrap();
    
    // Delete invalid routes in Redis
    for route in &old_routes {
        if !new_routes.contains(&route) {
            let _: () = conn.lrem("route_names", 0, route).unwrap();
        }
    }
    
    // Add new routes to Redis
    for route in new_routes {
        if !old_routes.contains(&route) {
            let _: () = conn.lpush("route_names", route).unwrap();
        }
    }
}

fn update_predictions(conn: &Connection) {
    let parameters: String = match conn.get("schedule_query") {
        Ok(n) => n,
        Err(e) => {
            println!("{:?}", e);
            return;
        },
    };

    let query = format!("{}{}", REQUEST_PREDICTIONS, parameters);
    let mut response = match reqwest::get(&query) {
        Ok(r) => r,
        Err(e) => {
            println!("{:?}", e);
            return;
        },

    };

    let predictions = match response.text() {
        Ok(t) => t,
        Err(e) => {
            println!("{:?}", e);
            return;
        },

    };
   
    let _: () = conn.set("raw_predictions", predictions).expect("unable to set raw_predictions");
    api_worker::process_raw_predictions();
}

fn update_route_config(conn: &Connection) {
    // Make request to nextBus servers
    let request = reqwest::get("http://webservices.nextbus.com/service/publicJSONFeed?command=routeConfig&a=rutgers&terse");
    let config: String = request.unwrap().text().unwrap();

    let _: () = conn.set("config", config).unwrap();
    api_worker::create_predictions_query();
    api_worker::store_route_order();
    api_worker::store_stop_locations();
 }
