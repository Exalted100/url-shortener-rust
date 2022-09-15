extern crate redis;
use redis::Commands;
use rocket::{http::Status, serde::json::Json, State, response::Redirect, uri};
use crate::models::shortener_model::{UrlRequest, UrlResponse};
use crate::utils::shorten_url::{GetShortUrl, GetTrueUrl, SharedRedisClient};

#[post("/shorten", data = "<new_url_request>")]
pub fn shorten_url(
    redis_client: &State<SharedRedisClient>,
    new_url_request: Json<UrlRequest>,
) -> Result<Json<UrlResponse>, Status> {
    // let mut con = redisCon.get_connection();
    // let uniqueId = createUniqueId();
    let response = GetShortUrl(redis_client, &new_url_request.url);

    // TODO - include ttl functionality
    match response {
        Ok(url) => Ok(Json(url)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<redis_reference>")]
pub fn find_true_url(
    redis_client: &State<SharedRedisClient>,
    redis_reference: String
) -> Redirect {
    // if redis_reference.is_empty() {
    //     return Err(Status::BadRequest);
    // };
    let true_address = GetTrueUrl(redis_client, &redis_reference);

    // TODO - include ttl functionality
    println!("{}", true_address);
    Redirect::to(true_address)
}