use actix_web::web::{self, scope};
use actix_web::{web::{
    Data,
    Json,

}, post, HttpResponse } ;

use crate::{models::todo::Todo, repository::database::Database};

#[post("/todos")]
pub async fn create_todo(db: Data<Database>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/api").service(create_todo)
    );
}