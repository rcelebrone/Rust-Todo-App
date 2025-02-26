#![feature(decl_macro)]

mod todo;
mod utils;

use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use todo::{
    todo_model::Todo,
    todo_repository::{TodoRepository, TodoRepositoryImpl},
};
use utils::{uuid_params::UuidParam, request_model::CreateTodoRequest,};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index(
    repo: &State<TodoRepository>,
) -> String {
    let mut repo_mut = repo.inner().clone();
    match repo_mut.create("new todo") {
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

#[post("/", format = "json", data = "<create_todo_request>")]
fn create(
    create_todo_request: Json<CreateTodoRequest>,
    repo: &State<TodoRepository>,
) -> String {
    let mut repo_mut = repo.inner().clone();
    match repo_mut.create(&create_todo_request.text) {
        Ok(todo) => format!("Created todo: {}", todo.id),
        Err(_) => String::from("Error creating todo"),
    }
}


#[launch]
async fn rocket() -> _ {
    rocket::build()
      .mount("/", routes![index, get_by_id, create])
      .manage(TodoRepository::default()) // Provide the repository as a managed state
}
