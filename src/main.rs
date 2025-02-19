#![feature(proc_macro_hygiene, decl_macro)]

mod todo;
mod utils;

use axum::http::response;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use todo::{
    todo_model::Todo,
    todo_repository::{TodoRepository, TodoRepositoryImpl},
};
use utils::uuid_params::UuidParam;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    let mut repo = TodoRepository::default();
    let todo = TodoRepositoryImpl::create(&mut repo, "New Todo");
    match todo {
        Ok(todo) => format!("Created todo: {}", todo.id),
        Err(_) => String::from("Error creating todo"),
    }
}

#[get("/<id>")]
fn get_by_id(
    id: UuidParam,
    repo: &State<TodoRepository>,
) -> Result<Json<Todo>, NotFound<String>> {
    match TodoRepositoryImpl::get(repo.inner(), &id.0) {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(NotFound("Todo not found".to_string())),
    }
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
      .mount("/", routes![index, get_by_id])
      .manage(TodoRepository::default())
      .launch()
      .await
    {
        println!("Rocket failed to launch: {}", e);
    }
}