use actix_web::{App, HttpServer, HttpResponse, web, Responder};


pub struct AppDetail{
    app_name: String,
}

pub async fn index(data : web::Data<AppDetail>) -> impl Responder {
    let response = "the app name is".to_string() +  &data.app_name;
    // let response = String::from("value");
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let url = String::from("127.0.0.1:8000");
    println!("The Server is running on {:?}", url);

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppDetail{
                app_name: "actix web server".to_string(),
            }))
            .route("/", web::get().to(index))
    })
    .bind(url)?
    .run()
    .await
}