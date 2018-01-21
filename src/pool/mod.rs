extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use r2d2_redis::RedisConnectionManager;

// An alias to the type for a pool of Redis connections.
type Pool = r2d2::Pool<RedisConnectionManager>;

//Redis Address
const REDIS_PORT: &'static str = "redis://localhost:6379";

//Initialize a Redis pool
pub fn init_pool() -> r2d2::Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new(REDIS_PORT).unwrap();
    r2d2::Pool::builder()
        .max_size(16)
        .build(manager)
        .unwrap()
}

pub struct RedisConn(pub r2d2::PooledConnection<RedisConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for RedisConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RedisConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(RedisConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for RedisConn {
    type Target = redis::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
