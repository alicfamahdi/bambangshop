
use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
use crate::model::notification::Notification;

#[derive (Debug, Clone, Deserialize, Serialize)] 
#[serde (crate = "rocket::serde")]

pub struct Subscriber {
    pub url: String,
    pub name: String,
}


// 1. Implement update method in Subscriber model in src/model/subscriber.rs. This method
// will send a notification to a Subscriber based on the provided URL, using HTTP POST request. We use tokio library to make async functions usable as sync functions, and the reqwest external library to make HTTP POST requests.
impl Subscriber {
    #[tokio::main]
    pub async fn update(&self, payload: Notification) {
        REQWEST CLIENT
            .post(&self.url)
            .header("Content-Type", "JSON")
            .body(to_string(&payload).unwrap())
            .send().await.ok();
        log::warn_! ("Sent {} notification of: [{}] {}, to: {}",
            payload.status, payload.product_type, payload.product_title, self.url);
    }
}