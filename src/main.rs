use actix_web:: {get, web, HttpResponse, HttpServer, Responder, App};
use serde::Serialize;

mod api;
mod models;
mod repository;

#[derive(Debug, Serialize)]
pub struct Response {
    pub status: String,
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response{
        status: "200".to_string(),
        message : String::from("Server is running fine"),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() ->impl Responder {
    let response = Response {
        status: "200".to_string(),
        message: String::from("The requested respurce is not available"),
    };
    HttpResponse::Ok().json(response)
}


#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let url =  "127.0.0.1:8000".to_string();
    println!("Server is running on {url}");

    let todo_db = repository::database::Database::new();
    let app_data = web::Data::new(todo_db);

    HttpServer::new( move ||{
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(healthcheck)
            .default_service(
                web::route().to(not_found)
            )
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(url)?
    .run()
    .await
}