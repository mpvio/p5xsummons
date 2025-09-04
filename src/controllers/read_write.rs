use std::{fs::{self, File}, io::Write};

use crate::models::named::NamedData;

pub fn read_items(item: &String) -> Option<NamedData>{
    if let Ok(info) = fs::read_to_string(format!("{item}.json")) {
        if let Ok(named_data) = serde_json::from_str::<NamedData>(&info) {
            return Some(named_data);
        }
    }
    None
}

pub fn write_items(item: &String, data: &NamedData) -> String {
    if let Ok(json_data) = serde_json::to_string_pretty(&data) {
        if let Ok(mut file) = File::create(format!("{item}.json")) {
            return match file.write_all(json_data.as_bytes()) {
                Ok(_) => format!("{item} updated."),
                Err(err) => format!("{err:#?}"),
            };
        }
    }
    String::from("")
}