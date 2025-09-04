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
    pub id: u128,
    pub cpt: String,
    pub aas: u16,
    pub aat: String,
    pub t: u128
}