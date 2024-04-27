use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
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
    // Additional methods for TransactionData
    pub fn new(id: String, from: String, to: String, amount: f64, requirements: String, description: String, signature: String) -> Self {
        Self {
            id,
            from,
            to,
            amount,
            requirements,
            description,
            signature,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub data: TransactionData,
}

impl Transaction {
    pub fn new(data: TransactionData) -> Self {
        Self { data }
    }
}
