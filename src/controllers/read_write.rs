use std::{fs::{self, File}, io::{Seek, SeekFrom, Write}};

use crate::models::named::Named;

fn read_named() -> Option<Named> {
    if let Ok(info) = fs::read_to_string(format!("p5xsummons.json")) {
        if let Ok(named_data) = serde_json::from_str::<Named>(&info) {
            return Some(named_data);
        }
    }
    None
}

pub fn update_named(data: &Named) -> String {
    match &mut read_named() {
        Some(old) => {
            let (std, sb) = data.standard_update(old);
            let (chr, cb) = data.character_update(old);
            let (wea, wb) = data.weapon_update(old);
            let (beg, bb) = data.beginner_update(old);
            if sb || cb || wb || bb {
                // at least one of the above changed
                write_named(old);
                return format!("{std}, {chr}, {wea}, {beg}.");
            }
            return format!("No changes.");
            
        },
        None => return write_named(data),
    }
}

fn write_named(data: &Named) -> String {
    if let Ok(json_data) = serde_json::to_string_pretty(&data) {
        if let Ok(mut file) = File::create(format!("p5xsummons.json"))
        {
            // clear file and set pointer to start
            let _ = file.set_len(0);
            let _ = file.seek(SeekFrom::Start(0));
            // handle result of file write
            return match file.write_all(json_data.as_bytes()) {
                Ok(_) => format!("p5xsummons created."),
                Err(err) => format!("{err:#?}"),
            };
        }
    }
    String::from("")
}