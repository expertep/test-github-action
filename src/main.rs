use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv; // Import dotenv
use std::env; // Import env

#[get("/")]
async fn hello() -> impl Responder {
    dotenv().ok();  // Load the .env file
    
    // Get the environment variable "ENV", or use a default if not set
    let env = env::var("ENV").unwrap_or_else(|_| "development".to_string());
    let endpoint = env::var("ENDPOINT").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // Return the formatted response including environment details
    HttpResponse::Ok().body(format!(
        "Server is running on environment: {}, endpoint: {}, port: {}",
        env, endpoint, port
    ))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello) // Register the hello handler
    })
    .bind("127.0.0.1:8080")?  // Bind to localhost:8080
    .run()
    .await
}