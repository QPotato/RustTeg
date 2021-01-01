#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
// #[macro_use] extern crate serde_derive;
extern crate rusteg;

// #[cfg(test)] mod tests;

use std::sync::Mutex;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use rusteg::{start, Game};


// // TODO: This example can be improved by using `route` with multiple HTTP verbs.
// #[post("/<id>", format = "json", data = "<message>")]
// fn new(id: ID, message: Json<Message>, map: State<'_, MessageMap>) -> JsonValue {
//     let mut hashmap = map.lock().expect("map lock.");
//     if hashmap.contains_key(&id) {
//         json!({
//             "status": "error",
//             "reason": "ID exists. Try put."
//         })
//     } else {
//         hashmap.insert(id, message.0.contents);
//         json!({ "status": "ok" })
//     }
// }

#[get("/game", format = "json")]
fn get(game: State<'_, Game>) -> JsonValue {
    json!(game.inner())
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    let game = start();
    rocket::ignite()
        .mount("/", routes![get])
        .register(catchers![not_found])
        .manage(game)
}

fn main() {
    rocket().launch();
}