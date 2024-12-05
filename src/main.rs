use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Actix-Web!")
}

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u8,
}

#[post("/echo")]
async fn echo(info: web::Json<Info>) -> impl Responder {
    let response = format!("Hello, {}! You are {} years old.", info.name, info.age);
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}