use crate::smart_contract::SmartContract;
use crate::transaction::Transaction;
use serde_json;
use std::error::Error;


pub struct Block {
    pub transactions: Vec<Transaction>,
    // ... other fields
}

impl Block {
    /// Validates all transactions in the block.
    pub fn validate_transactions(&self) -> bool {
        for tx in &self.transactions {
            if !SmartContract::is_valid_transaction(tx) {
                return false;
            }
        }
        true
    }

    /// Converts transactions to JSON format.
    pub fn transactions_to_json(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut json_transactions = Vec::new();

        for tx in &self.transactions {
            let json_tx = serde_json::to_string(tx)?;
            json_transactions.push(json_tx);
        }

        Ok(json_transactions)
    }
}