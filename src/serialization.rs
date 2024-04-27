// src/serialization.rs

use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub requirements: String,
    pub description: String,
    pub signature: String,
}

impl TransactionData {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self)
    }

    pub fn from_json(json_str: &str) -> Result<Self> {
        serde_json::from_str(json_str)
    }
}
