use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub code: u64,
    pub msg: String,
    pub data: ResponseData
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseData {
   pub page: u64,
   pub size: u64,
   pub list: Vec<ResponseSummons>,
   pub pages: u64,
   pub total: u64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseSummons {
    pub id: u128,
    pub cpt: u8,
    pub aas: u32,
    pub aat: u8,
    pub t: u128
}