use rocket::response::status::NotFound;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::State;

// Define a struct to represent a wine item
#[derive(Deserialize, Serialize)]
pub struct Wine {
    pub title: String,
    pub province: Option<String>,
    pub description: Option<String>, 
    pub points: Option<u32>,
    pub country: Option<String>,
    pub price: Option<f32>,
    pub winery: Option<String>,
}

pub fn filter_wine(wine: &Wine, query: &str) -> bool {
    wine.title.contains(query) ||
    wine.province.as_deref().unwrap_or("").contains(query) ||
    wine.description.as_deref().unwrap_or("").contains(query) ||
    wine.country.as_deref().unwrap_or("").contains(query) ||
    wine.winery.as_deref().unwrap_or("").contains(query)
}

pub fn filter_wines<'a>(wines: &'a [Wine], query: &str) -> Vec<&'a Wine> {
    wines.iter()
         .filter(|wine| filter_wine(wine, query))
         .take(10)
         .collect()
}

#[get("/wines?<q>")]
pub async fn search_wines(q: Option<String>, wines: &State<Vec<Wine>>) -> Result<Json<Vec<&Wine>>, NotFound<String>> {
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
