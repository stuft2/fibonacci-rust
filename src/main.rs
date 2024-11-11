// src/main.rs

use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use std::time::Instant;

// Function to calculate Fibonacci number
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let tmp = a + b;
            a = b;
            b = tmp;
        }
        b
    }
}

// Asynchronous handler for the Fibonacci endpoint
#[get("/fibonacci")]
async fn fibonacci_handler(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    // Parse "n" parameter from the query string, default to 40 if not provided
    let n = query.get("n").and_then(|n| n.parse::<u64>().ok()).unwrap_or(40);

    let start_time = Instant::now();

    // Calculate Fibonacci in a blocking way (synchronously) within a thread
    let result = web::block(move || fibonacci(n)).await.unwrap_or(0);

    let elapsed = start_time.elapsed().as_millis();

    // Respond with JSON data
    HttpResponse::Ok().json(serde_json::json!({
        "number": n,
        "result": result,
        "elapsed": format!("{}ms", elapsed),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Fibonacci server at http://localhost:8080");

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .service(fibonacci_handler) // Register the Fibonacci handler
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
