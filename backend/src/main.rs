#[macro_use]
extern crate rocket;

use rocket::response::status::NotFound;
use rocket::serde::{Serialize,Deserialize, json::Json};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins,AllowedHeaders};
use std::env;
use std::fs::File;
use std::io::BufReader;

// Define a struct to represent a wine item
#[derive(Deserialize, Serialize)]
struct Wine {
    title: String,
    province: Option<String>,
    description: Option<String>, 
    points: Option<u32>,
    country: Option<String>,
    price: Option<f32>,
    winery: Option<String>
}

async fn load_wines(file_path: &String) -> Vec<Wine> {
    let file = File::open(&file_path).expect("file should open read only");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("file should be proper JSON")
}


fn filter_wine(wine: &Wine, query: &str) -> bool {
    wine.title.contains(query) ||
                        wine.province.as_deref().unwrap_or("").contains(query)
                        || wine.description.as_deref().unwrap_or("").contains(query)
                        || wine.country.as_deref().unwrap_or("").contains(query)
                        || wine.winery.as_deref().unwrap_or("").contains(query)
}

fn filter_wines<'a>(wines: &'a [Wine], query: &str) -> Vec<&'a Wine> {
    wines.iter()
         .filter(|wine| filter_wine(wine, query))
         .take(10)
         .collect()
}


#[get("/wines?<q>")]
async fn search_wines(q: Option<String>, wines: &rocket::State<Vec<Wine>>) ->  Result<Json<Vec<&Wine>>, NotFound<String>> {
    if let Some(query) = q {
        let filtered_wines = filter_wines(&wines, &query);
        if filtered_wines.is_empty() {
            Err(NotFound("No wine items found".to_string()))
        } else {
            Ok(Json(filtered_wines))
        }
    } else {
        Err(NotFound("No wine items found".to_string()))
    }
}


#[rocket::main]
async fn main() {
    // get all arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments are passed
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

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
      // panic if there was an error
      .expect("error creating CORS fairing");

    rocket::build()
        .manage(wines)
        .mount("/", routes![search_wines])
        .attach(cors)
        .launch()
        .await
        .expect("Failed to start Rocket server");
}