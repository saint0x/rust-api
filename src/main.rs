mod block;
mod transaction;
mod serialization;
mod smart_contract;

use block::Block;
use transaction::{Transaction, TransactionData};

fn main() {
    // Create a new blockchain
    let mut blockchain = Blockchain::new();

    // Create some transactions
    let transaction1_data = TransactionData::new(
        "id1".to_string(),
        "Alice".to_string(),
        "Bob".to_string(),
        10.0,
        "Requirement 1".to_string(),
        "Description 1".to_string(),
        "Signature 1".to_string(),
    );

    let transaction2_data = TransactionData::new(
        "id2".to_string(),
        "Bob".to_string(),
        "Alice".to_string(),
        5.0,
        "Requirement 2".to_string(),
        "Description 2".to_string(),
        "Signature 2".to_string(),
    );

    let transaction1 = Transaction::new(transaction1_data);
    let transaction2 = Transaction::new(transaction2_data);

    // Add transactions to the blockchain
    blockchain.add_block(vec![transaction1.clone(), transaction2.clone()]);

    // Print the blockchain
    println!("Blockchain: {:?}", blockchain);

    // Serialize transactions to JSON
    let transaction_data1 = transaction1.data.clone();
    let transaction_data2 = transaction2.data.clone();

    let json_str1 = serde_json::to_string(&transaction_data1).unwrap();
    let json_str2 = serde_json::to_string(&transaction_data2).unwrap();

    println!("Transaction 1 JSON: {}", json_str1);
    println!("Transaction 2 JSON: {}", json_str2);

    // Deserialize transactions from JSON
    let parsed_transaction_data1: TransactionData = serde_json::from_str(&json_str1).unwrap();
    let parsed_transaction_data2: TransactionData = serde_json::from_str(&json_str2).unwrap();

    let parsed_transaction1 = Transaction::new(parsed_transaction_data1);
    let parsed_transaction2 = Transaction::new(parsed_transaction_data2);

    println!("Parsed Transaction 1: {:?}", parsed_transaction1);
    println!("Parsed Transaction 2: {:?}", parsed_transaction2);
}
