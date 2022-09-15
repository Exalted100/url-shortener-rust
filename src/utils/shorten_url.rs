extern crate nanoid;
extern crate redis;
use crate::models::shortener_model::UrlResponse;
use nanoid::nanoid;
use redis::Commands;
use std::sync::{Arc, Mutex};
use rocket::{http::Status, State};

pub type SharedRedisClient = Arc<Mutex<redis::Client>>;

pub fn GetShortUrl(redis_client: &State<SharedRedisClient>, url: &str) -> Result<UrlResponse, &'static str> {
    let mut redis = redis_client.lock().unwrap().get_connection().unwrap();
    let uniqueId = createUniqueId();

    // TODO - check for error
    let _: () = redis.set(&uniqueId, url).unwrap();
    return Ok(UrlResponse {
        // TODO - return a full url
        url: url.to_string(),
        shortenedUrl: uniqueId,
    });
}

pub fn GetTrueUrl(redis_client: &State<SharedRedisClient>, redis_reference: &str) -> String {
    let mut redis = redis_client.lock().unwrap().get_connection().unwrap();

    // TODO - check for error
    println!("{}", redis_reference);
    let true_url: Option<String> = redis.get(redis_reference).unwrap();

    return true_url.unwrap()
}

pub fn createUniqueId() -> String {

    let alphabet: [char; 36] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',  'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

   let id = nanoid!(10, &alphabet);
   // TODO - perform check for when there is a conflict
   return id;
}