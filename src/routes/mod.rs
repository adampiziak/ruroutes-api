use pool::RedisConn;
use rocket::Route;
//use rocket::Request;
use rocket::response::content;
//use rocket::response::Failure;
//use rocket::http::Status;
use redis::Commands;
// use std::thread::sleep;
//use std::time::{Duration, Instant};



#[get("/")]
fn index() -> &'static str {
    "hey"
}

#[get("/config")]
fn config(conn: RedisConn) -> content::Json<String> {
    let config: String = conn.get("config").unwrap();
   
    content::Json(config)
}

#[get("/predictions")]
fn predictions(conn: RedisConn) -> content::Json<String> {
    let pred: String = conn.get("route_predictions").unwrap();

    content::Json(pred)
}

#[get("/stops")]
fn stops(conn: RedisConn) -> content::Json<String> {
    let pred: String = conn.get("stop_predictions").unwrap();

    content::Json(pred)
}

pub fn create() -> Vec<Route> {
    routes!(index, config, predictions, stops)
}
