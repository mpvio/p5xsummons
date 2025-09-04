use std::io;

use crate::{controllers::manage_auth_key::get_key, models::{named::{NamedData, NamedSummons}, original::{Response, ResponseData}}};

pub async fn query_all(key: Option<String>) -> Vec<NamedData> {
    let mut all_data: Vec<NamedData> = vec![];
    for i in 1..5 {
        if let Some(data) = query(i, 1, &key).await {
            all_data.push(data);
        }
    }
    all_data
}

pub async fn query(gacha_type: u8, page: u8, key: &Option<String>) -> Option<NamedData> {
    let try_key: io::Result<String> = if let Some(k) = key {
        Ok(k.clone())
    } else {
        get_key()
    };
    if let Ok(auth_key) = try_key {
        let _params = [
            ("gachaType", &gacha_type.to_string()), ("authKey", &auth_key), ("page", &page.to_string()), ("size", &String::from("10"))
        ];

        let base = format!("https://euweb.p5xjpupd.com/gacha/getRecords?gachaType={gacha_type}&page={page}&size=10&authKey={auth_key}");

        if let Ok(url) = reqwest::Url::parse(&base) {
            if let Ok(response) = reqwest::get(url).await {
                if response.status() == reqwest::StatusCode::OK {
                    if let Ok(info) = response.json::<Response>().await {
                        let mut named_summons = name_summons(&info.data);
                        if info.data.page < info.data.pages {
                            // append next page(s)
                            if let Some(mut remainder) = Box::pin(query(gacha_type, page+1, &Some(auth_key))).await {
                                named_summons.append(&mut remainder.list);
                            }
                        }
                        let total = named_summons.len();
                        let mut named_data = NamedData {
                            summon: get_banner_type(&gacha_type),
                            list: named_summons,
                            total
                        };
                        // ensure summons are sorted
                        named_data.sort_by_summon_id_desc();
                        return Some(named_data);
                    }
                }
            }
        }
    }
    None
}

fn name_summons(data: &ResponseData) -> Vec<NamedSummons> {
    let mut named_summons: Vec<NamedSummons> = vec![];
    for summon in &data.list {
        let item = match summon.aat {
            3 => "character",
            6 => "persona",
            _ => "weapon"
        }.to_string();
        let banner = get_banner_type(&summon.cpt);
        let named_summon = NamedSummons {
            summon_id: summon.id,
            banner,
            item_id: summon.aas.to_string(),
            item,
            timestamp: summon.t,
        };
        named_summons.push(named_summon);
    }
    named_summons
}

fn get_banner_type(gacha_type: &u8) -> String {
    match gacha_type {
        1 => "standard",
        2 => "character", // limited character
        3 => "weapon", // limited weapon
        _ => "beginner"
    }.to_string()
}