use std::fs::read_to_string;


use crate::models::{named::{NamedData, NamedSummons}, original::{Response, ResponseData}};

pub async fn query(gacha_type: u8, page: u8) -> Option<NamedData> {
    // PRINT
    // println!("{gacha_type}, {page}");
    if let Ok(auth_key) = read_to_string("authKey.txt") {
        let _params = [
            ("gachaType", &gacha_type.to_string()), ("authKey", &auth_key), ("page", &page.to_string()), ("size", &String::from("10"))
        ];

        let base = format!("https://euweb.p5xjpupd.com/gacha/getRecords?gachaType={gacha_type}&page={page}&size=10&authKey={auth_key}");

        if let Ok(url) = reqwest::Url::parse(&base) {
            if let Ok(response) = reqwest::get(url).await {
                if response.status() == reqwest::StatusCode::OK {
                    if let Ok(info) = response.json::<Response>().await {
                        let mut named_summons = name_summons(&info.data);
                        // PRINT
                        // println!("{} < {}?", info.data.page, info.data.pages);
                        if info.data.page < info.data.pages {
                            // append next page(s)
                            if let Some(mut remainder) = Box::pin(query(gacha_type, page+1)).await {
                                named_summons.append(&mut remainder.list);
                            }
                        }
                        let total = named_summons.len();
                        let named_data = NamedData {
                            summon: get_banner_type(&gacha_type),
                            list: named_summons,
                            total
                        };
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
            id: summon.id,
            cpt: banner,
            aas: summon.aas,
            aat: item,
            t: summon.t,
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