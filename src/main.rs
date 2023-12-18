use actix_web::{App, HttpServer, HttpResponse, Responder, web, get, post};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world this is index page")
}

#[post("/home")]
pub async fn home() ->impl Responder {
    HttpResponse::Ok().body("hello this is home page")
}

pub async fn page404 () -> impl Responder {
    HttpResponse::Ok().body("<h1>404 Page not found</h1>")
}

pub async fn app_home() -> impl Responder {
    HttpResponse::Ok().body("this is app home page")
}

pub async fn app_index() ->impl Responder {
    HttpResponse:: Ok().body("this is app index page")
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let url = String::from("127.0.0.1:8000");
    println!("the server is running on {:?} ", url);
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app")
                    .route("/", web::get().to(app_index))
                    .route("/home", web::get().to(app_home))
            )  
            .service(index)
            .service(home)
            .route("/404", web::get().to(page404))
    })
    .bind(url)?
    .run()
    .await
}