use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Named {
    pub data: NamedData
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
    pub item: String,
    pub timestamp: u128
}