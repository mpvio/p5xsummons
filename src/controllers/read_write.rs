use std::{collections::HashSet, fs::{self, File}, io::Write};

use crate::models::named::{NamedData, NamedSummons};

pub fn read_items(item: &String) -> Option<NamedData>{
    if let Ok(info) = fs::read_to_string(format!("{item}.json")) {
        if let Ok(named_data) = serde_json::from_str::<NamedData>(&info) {
            return Some(named_data);
        }
    }
    None
}

pub fn update_items(item: &String, data: &NamedData) -> String {
    match &mut read_items(item) {
        Some(old) => {
            if old.total == data.total {
                return String::from(format!("{item} unchanged."));
            }
            // update old list with new data and write to file
            update_helper(old, data);
            write_items(item, old)
        },
        None => {
            // there is no old file, so just write directly to file
            write_items(item, data)
        },
    }
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

fn update_helper(old: &mut NamedData, data: &NamedData) {
    // get summon_ids from old
    let existing_ids: HashSet<u128> = old.list
    .iter()
    .map(|summon| summon.summon_id)
    .collect();
    // filter data.list to get ONLY new summons
    let new_summons: Vec<NamedSummons> = data.list
    .iter()
    .filter(|summon| !existing_ids.contains(&summon.summon_id))
    .cloned()
    .collect();
    // add new summons to old list
    old.list.extend(new_summons);
    old.total = old.list.len();
    // sort to move new summons to top of list
    old.sort_by_summon_id_desc();
}