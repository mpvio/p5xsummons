use crate::{controllers::{get_data::query, read_write::write_items}, models::named::NamedData};

pub mod models;
pub mod controllers;

// TODO
/*
* add file IO
* add file updating (only add summons if their ids are unique)
* if a database is found -> replace item signifiers (aas) with item names
*/
#[tokio::main]
async fn main() {
    let mut all_data: Vec<NamedData> = vec![];
    for i in 1..5 {
        if let Some(data) = query(i, 1).await {
            write_items(&data.summon, &data);
            all_data.push(data);
        }
    }
    for ad in all_data {
        println!("{ad:#?}");
    }
}
