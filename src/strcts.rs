#![allow(non_camel_case_types)]

use bc_utils_lg::structs::settings::SETTINGS_INDS;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct SETTINGS_S {
    pub api_url: String,
    pub category: String,
    pub update: u64,
    pub th: f64,
    pub limit: f64,
    pub interval: String,
    pub delay_req_sec: u64,
    pub black_list_symbols: Vec<String>,
    pub black_list_coins: Vec<String>,
    pub symbols: Vec<String>,
    pub wait_ms_req: usize,
    pub wait_ms_cycle_req: usize,
    pub indicators: SETTINGS_INDS,
    pub indicators_used: Vec<String>,
}

impl Default for SETTINGS_S {
    fn default() -> Self {
        Self {
            api_url: "https://api-demo.bybit.com".to_string(),
            category: "linear".to_string(),
            update: 0,
            th: 0.005,
            limit: 0.1,
            interval: "1".to_string(),
            delay_req_sec: 5,
            black_list_symbols: vec![],
            black_list_coins: vec![],
            symbols: vec![],
            wait_ms_req: 3000,
            wait_ms_cycle_req: 6000,
            indicators: SETTINGS_INDS::default(),
            indicators_used: Vec::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct SETTINGS {
    pub setdef: SETTINGS_S,
    pub other_api: Vec<FxHashMap<String, String>>,
}
