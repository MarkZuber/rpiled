#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use core::{DisplayTextRequest, DisplayTextResponse};
use rocket_contrib::json::Json;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/", data = "<payload>")]
fn display_text(payload: Json<DisplayTextRequest>) -> Json<DisplayTextResponse> {
    rledlib::circles::run();
    Json(DisplayTextResponse {
        message: format!("We would be displaying text: {}", payload.0.text),
    })
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![hello])
        .mount("/displaytext", routes![display_text])
        .launch();
}
