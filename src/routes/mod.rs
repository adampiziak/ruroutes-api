use pool::RedisConn;
use rocket::Route;
use rocket::response::content::Json;
use redis::Commands;

#[get("/")]
fn index() -> &'static str {
    "rutgers link api"
}

#[get("/config")]
fn config(conn: RedisConn) -> Json<String> {
    let config: String = conn.get("config").unwrap();

    Json(config)
}


pub fn create() -> Vec<Route> {
    routes!(index, config)
}
