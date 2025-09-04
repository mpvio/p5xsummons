use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Named {
    pub standard: NamedData,
    pub character: NamedData,
    pub weapon: NamedData,
    pub beginner: NamedData
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedData {
    pub summon: String,
    pub list: Vec<NamedSummons>,
    pub total: usize
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedSummons {
    #[serde(rename = "summonId")]
    pub summon_id: u128,
    pub banner: String, // banner type, remove?
    #[serde(rename = "itemId")]
    pub item_id: String,
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
        match (self.list.first(), other.list.first()) {
            (Some(current), Some(prev)) => {
                current.summon_id != prev.summon_id
            },
            _ => true
        }
    }
}