#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use rocket_contrib::Json;

#[derive(Serialize)]
struct Timestamp {
    time: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/time")]
fn time_now() -> Json<Timestamp> {
    let now = Utc::now();
    let timestamp = Timestamp { time: now.to_rfc3339() };

    Json(timestamp)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, time_now])
        .launch();
}
