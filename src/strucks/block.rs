use super::transaction::Transaction;
use serde_json::json;
use sha2::Sha256;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use sha2::Digest;
use hex::encode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    index: u64,
    timestamp: Option<u64>,
    previous_hash: String,
    hash: Option<String>,
    validator: Option<String>,
    transactions: Vec<Transaction>
}

impl Block {
    pub fn new(index_block: u64, previous_hash_block: String, transactions: Vec<Transaction>) -> Block {
        Self {
            index: index_block,
            timestamp: None,
            previous_hash: previous_hash_block,
            hash: None,
            validator: None,
            transactions: transactions,
        }
    }

    pub fn get_index(&self) -> u64{
        self.index
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone().unwrap_or_default()
    }

    pub fn build_block(&self) {
        
    }

    pub fn calculate_hash(&mut self) {
        let mut hasher = Sha256::new();

        let block_data = self.build_json_hash();
        hasher.update(block_data);
        let result = hasher.finalize();
        let hash = encode(&result);
        self.hash = Some(hash.clone());
    }

    pub fn current_time_millis() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_secs() * 1000 + u64::from(since_the_epoch.subsec_millis())
    }

    pub fn set_timestamp(&mut self) {
        self.timestamp = Some(Self::current_time_millis());
    }

    pub fn build_json_print(&self) -> String {
        // let json_string = serde_json::to_string(&self.transactions).expect("Failed to serialize");

        let json_result = json!({
            "index": self.index,
            "timestamp": self.timestamp.unwrap_or_default(),
            "previous_hash": self.previous_hash.clone(),
            "hash": self.hash.clone().unwrap_or_default(),
            "transactions": self.transactions.len(),
        });
        serde_json::to_string_pretty(&json_result).unwrap()
    }

    pub fn build_json_full_print(&self) -> String {
        // let json_string = serde_json::to_string(&self.transactions).expect("Failed to serialize");

        let json_result = json!({
            "index": self.index,
            "timestamp": self.timestamp.unwrap_or_default(),
            "previous_hash": self.previous_hash.clone(),
            "hash": self.hash.clone().unwrap_or_default(),
            "transactions": self.transactions,
        });
        serde_json::to_string_pretty(&json_result).unwrap()
    }

    pub fn build_json_hash(&self) -> String {
        let json_result = json!({
            "index": self.index,
            "timestamp": self.timestamp.unwrap_or_default(),
            "previous_hash": self.previous_hash.clone(),
            "hash": self.hash.clone().unwrap_or_default(),
            "transactions": self.transactions,
        });
        serde_json::to_string(&json_result).unwrap()
    }

    // pub fn get_timestamp(&self) {
    //     self.timestamp
    // }

    // pub fn get_previous_hash(&self) {
    //     self.previous_hash
    // }

    // pub fn get_hash(&self) {
    //     self.hash
    // }

    // pub fn get_validator(&self) -> String{
    //     self.validator
    // }

    pub fn get_transactions(&self) -> Vec<Transaction>{
        self.transactions.clone()
    }

    // pub fn calculate_hash(&self) -> u64 {
        
    // }
    
}