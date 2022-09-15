mod api; 
mod models;
mod utils;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
extern crate redis;
use redis::Commands;
use std::sync::{Arc, Mutex};

use api::shortener_api::{shorten_url, find_true_url};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and rocket")))
}
// fn hello() -> Result<Json<String>, Status> {
//     Ok(Json(String::from("Hello from rust and rocket")))
// }

#[launch]
fn rocket() -> _ {
    let redis_client = redis::Client::open("redis://127.0.0.1").unwrap();
    rocket::build()
        .manage(Arc::new(Mutex::new(redis_client)))
        .mount("/", routes![hello])
        .mount("/", routes![shorten_url])
        .mount("/", routes![find_true_url])
}