#![allow(non_camel_case_types)]

use serde::{
    Serialize, 
    Deserialize
};
use rustc_hash::FxHashMap;

#[derive(Serialize, Deserialize)]
pub struct SETTINGS_S {
    pub api_url: String,
    pub category: String,
    pub update: u64,
    pub th: f64,
    pub limit: f64,
    pub delay_req_sec: u64,
    pub black_list_symbols: Vec<String>,
    pub black_list_coins: Vec<String>,
    pub symbols: Vec<String>,
    pub wait_sec_req: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SETTINGS {
    pub setdef: SETTINGS_S,
    pub other_api: Vec<FxHashMap<String, String>>
}
