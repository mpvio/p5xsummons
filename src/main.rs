use crate::{controllers::{get_data::query, read_write::{read_items, write_items}}, models::named::NamedData};

pub mod models;
pub mod controllers;

// TODO
/*
* add file updating (only add summons if their ids are unique)
* if a database is found -> replace item signifiers (aas) with item names
*/
#[tokio::main]
async fn main() {
    let data = _get_data().await;
    _write_to_files(data);
}

async fn _get_data() -> Vec<NamedData> {
    let mut all_data: Vec<NamedData> = vec![];
    for i in 1..5 {
        if let Some(data) = query(i, 1).await {
            all_data.push(data);
        }
    }
    all_data
}

fn _write_to_files(data: Vec<NamedData>) {
    for d in data {
        write_items(&d.summon, &d);
    }
}

fn _read_showcase() {
    let names : Vec<String> = vec!["standard".into(), "character".into(), "weapon".into(), "beginner".into()];
    _read_from_files(names);
}

fn _read_from_files(names: Vec<String>) {
    for n in names {
        let data = read_items(&n);
        if let Some(d) = data {
            println!("{d:#?}");
        }
    }
}
