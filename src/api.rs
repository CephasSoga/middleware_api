use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

// Define input and output structures
#[derive(Deserialize)]
struct InputData {
    values: Vec<f64>,
}

#[derive(Serialize)]
struct ProcessedData {
    mean: f64,
    sum: f64,
    count: usize,
}

// Process function (business logic)
fn process_data(data: InputData) -> ProcessedData {
    let sum: f64 = data.values.iter().sum();
    let count = data.values.len();
    let mean = if count > 0 { sum / count as f64 } else { 0.0 };

    ProcessedData { mean, sum, count }
}

// API Handlers
async fn process_endpoint(input: web::Json<InputData>) -> impl Responder {
    let result = process_data(input.into_inner());
    HttpResponse::Ok().json(result)
}

async fn health_endpoint() -> impl Responder {
    HttpResponse::Ok().body("API is healthy")
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/process", web::post().to(process_endpoint))
            .route("/health", web::get().to(health_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
