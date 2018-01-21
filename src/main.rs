#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

mod routes;
mod pool;
mod fetcher;

fn main() {
    fetcher::start();
    rocket::ignite()
        .mount("/", routes::create())
        .manage(pool::init_pool())
        .launch();
}
