use pool::RedisConn;
use rocket::Route;
use redis::Commands;

#[get("/")]
fn index() -> &'static str {
    "rutgers link api"
}

#[get("/config")]
fn config(conn: RedisConn) -> String {
    let config: String = conn.get("config").unwrap();

    config
}


pub fn create() -> Vec<Route> {
    routes!(index, config)
}
