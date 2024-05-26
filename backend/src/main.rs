#[macro_use]
extern crate rocket;

mod api;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use std::env;
use std::fs::File;
use std::io::BufReader;
use api::{Wine, search_wines};
use log::{info};

// Load wines into memory from a JSON file
async fn load_wines(file_path: &String) -> Vec<Wine> {
    let file = File::open(&file_path).expect("file should open read only");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("file should be proper JSON")
}

#[rocket::main]
async fn main() {
    // Initialize the logger
    //env_logger::init();

    // Get all arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments are passed
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    info!("Loading wines from file: {}", file_path);

    // Load wines into memory
    let wines = load_wines(file_path).await;

    // Configure CORS
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error creating CORS fairing");

    rocket::build()
        .manage(wines)
        .mount("/", routes![search_wines])
        .attach(cors)
        .launch()
        .await
        .expect("Failed to start Rocket server");
}
