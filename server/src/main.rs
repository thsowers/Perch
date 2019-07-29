#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use perch::search;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::Value;

#[get("/<query>", format = "json")]
fn get(query: String) -> Json<Value> {
    Json(search::search_as_json(query))
}

#[post("/buildIndex")]
fn build_index() {
    perch::build_search_index::write_persistent_index();
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![get, build_index])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
