use actix_web::web::{self, scope};
use actix_web::{web::{
    Data,
    Json,

}, post, get, put, delete, HttpResponse } ;

use crate::{models::todo::Todo, repository::database::Database};

#[post("/todos")]
pub async fn create_todo(db: Data<Database>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(db: Data<Database>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo = db.get_todo_by_id(&id);
    match todo {
        Some(todo)=>HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Requested data not found"),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id (db:  Data<Database>, id: web::Path<String>, updateed_todo: Json<Todo>) -> HttpResponse {
    let todo = db.update_todo_by_id(&id, updateed_todo.clone());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("The ID doesnot exist"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo = db.delete_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body(format!("The id {} not found", id)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/api")
        .service(create_todo)
        .service(get_todos)
        .service(get_todo_by_id)
        .service(update_todo_by_id)
        .service(delete_todo_by_id)
    );
}