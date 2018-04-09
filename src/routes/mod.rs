use pool::RedisConn;
use rocket::Route;
//use rocket::response::content::Json;
use redis::Commands;
// use std::thread::sleep;
//use std::time::{Duration, Instant};
use rocket::response::content;


#[get("/")]
fn index() -> &'static str {
    NOTE
}

#[get("/config")]
fn config(conn: RedisConn) -> String {
    //let start = Instant::now();
    let _: () = conn.set("length", NOTE.len()).unwrap();
    let list: Vec<String> = conn.lrange("route_names", 0, -1).unwrap();
    let config = list[0].clone();
    //let config: String = conn.get("route_names").unwrap();
    //let duration = start.elapsed();
    // println!("Duration: {:?}", duration);

    
    config
}

#[get("/predictions")]
fn predictions(conn: RedisConn) -> content::Json<String> {
    let pred: String = conn.get("predictions").unwrap();

    content::Json(pred)
}

pub fn create() -> Vec<Route> {
    routes!(index, config, predictions)
}

const NOTE: &'static str = "afda";
