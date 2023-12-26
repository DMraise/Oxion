use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wallet {
    pub public_key: String,
    balance: f64,
}

impl Wallet {
    pub fn new(public_key: String) -> Wallet {
        Self {
            public_key: public_key,
            balance: 0.0
        }
    }

    pub fn subtraction(&mut self, amount: f64) {
        self.balance -= amount;
    }

    pub fn addition(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn get_json(&self) -> String{
        let json_result = json!({
            "public_key": &self.public_key,
            "balance": &self.balance,
        });
        serde_json::to_string(&json_result).unwrap()
    }
}