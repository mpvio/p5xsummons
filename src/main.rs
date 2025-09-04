use crate::{controllers::get_data::query, models::named::NamedData};

pub mod models;
pub mod controllers;
#[tokio::main]
async fn main() {
    let mut all_data: Vec<NamedData> = vec![];
    for i in 1..5 {
        if let Some(data) = query(i, 1).await {
            all_data.push(data);
        }
    }
    for ad in all_data {
        println!("{ad:#?}");
    }
}
