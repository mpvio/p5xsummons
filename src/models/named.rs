use std::collections::HashSet;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use serde::de::Error;
use serde_json::Value;
use serde::Deserialize as deser;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Named {
    pub standard: Option<NamedData>,
    pub character: Option<NamedData>,
    pub weapon: Option<NamedData>,
    pub beginner: Option<NamedData>
}

impl Named {
    pub fn standard_update(&self, other: &mut Named) -> (String, bool) {
        if let Some(self_data) = &self.standard {
            match &mut other.standard {
                Some(data) => {
                    if self_data.update_needed(data) {
                        update_helper(data, self_data);
                        return (String::from("standard updated"), true);
                    }
                    return (String::from("standard unchanged"), false);
                },
                None => {},
            }
        }
        return (String::from("standard created"), true);
    }

    pub fn character_update(&self, other: &mut Named) -> (String, bool) {
        if let Some(self_data) = &self.character {
            match &mut other.character {
                Some(data) => {
                    if self_data.update_needed(data) {
                        update_helper(data, self_data);
                        return (String::from("character updated"), true);
                    }
                    return (String::from("character unchanged"), false);
                },
                None => {},
            }
        }
        return (String::from("character created"), true);
    }

    pub fn weapon_update(&self, other: &mut Named) -> (String, bool) {
        if let Some(self_data) = &self.weapon {
            match &mut other.weapon {
                Some(data) => {
                    if self_data.update_needed(data) {
                        update_helper(data, self_data);
                        return (String::from("weapon updated"), true);
                    }
                    return (String::from("weapon unchanged"), false);
                },
                None => {},
            }
        }
        return (String::from("weapon created"), true);
    }

    pub fn beginner_update(&self, other: &mut Named) -> (String, bool) {
        if let Some(self_data) = &self.beginner {
            match &mut other.beginner {
                Some(data) => {
                    if self_data.update_needed(data) {
                        update_helper(data, self_data);
                        return (String::from("beginner updated"), true);
                    }
                    return (String::from("beginner unchanged"), false);
                },
                None => {},
            }
        }
        return (String::from("beginner created"), true);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedData {
    pub list: Vec<NamedSummons>,
    pub total: usize
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedSummons {
    #[serde(rename = "summonId")]
    pub summon_id: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "itemId", deserialize_with = "deserialize_item_id")]
    pub item_id: u32,
    pub item: String, // character/ persona/ weapon
    pub timestamp: u128
}

impl NamedData {
    pub fn sort_by_summon_id_asc(&mut self) {
        self.list.sort_by(|a, b| a.summon_id.cmp(&b.summon_id));
    }

    pub fn sort_by_summon_id_desc(&mut self) {
        self.list.sort_by(|a, b| b.summon_id.cmp(&a.summon_id));
    }

    pub fn update_needed(&self, other: &NamedData) -> bool {
        if self.list.is_empty() && other.list.is_empty() {
            return false;
        }
        match (self.list.first(), other.list.first()) {
            (Some(current), Some(prev)) => {
                current.summon_id != prev.summon_id
            },
            _ => true
        }
    }
}

fn deserialize_item_id<'de, D>(deserializer: D) -> Result<u32, D::Error> where D: serde::Deserializer<'de> {
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Number(n) => Ok(n.as_u64().unwrap_or_default() as u32),
        Value::String(s) => s.parse::<u32>().map_err(Error::custom),
        _ => Ok(0)
    }
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