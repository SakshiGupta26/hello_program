 //cargo new name
// //cd name
// //cargo run
// //cargo add serde csv rocket
// //cargo install cargo-watch
// //cargo clean
// //cargo watch
use rocket::*;
mod tasks;

use tasks::*;
use rocket::serde::json::Json;

#[get("/tasks")]
fn fetch_tasks() -> Json<Vec<Task>> {
    let tasks = load_tasks();
    Json(tasks)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetch_tasks])
}

