use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, Local};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    timestamp: Option<u64>,
    fee: Option<f64>,
    hash: Option<String>,
}

impl Transaction {
    pub fn calculate_hash(&mut self) {
        // let transaction_data = serde_json::to_string(&(self.sender.clone(), self.recipient.clone(), self.amount)).unwrap();
        let transaction_data = self.build_json_hash();
        
        let mut hasher = Sha256::new();
        hasher.update(transaction_data);
        let result = hasher.finalize();

        let hash = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
        
        self.hash = Some(hash.clone());
    }

    pub fn build_json_hash(&self) -> String{
        let json_result = json!({
            "sender": &self.sender,
            "recipient": &self.recipient,
            "amount": self.amount,
            "timestamp": self.timestamp.unwrap_or_default(),
            "fee": self.fee.unwrap_or_default(),
        });
        serde_json::to_string(&json_result).unwrap()
    }

    pub fn build_json_print(&self) -> String {
        let json_result = json!({
            "sender": &self.sender,
            "recipient": &self.recipient,
            "amount": self.amount,
            "timestamp": self.timestamp.unwrap_or_default(),
            "fee": self.fee.unwrap_or_default(),
            "hash": self.hash.clone().unwrap_or_default()
        });
        serde_json::to_string_pretty(&json_result).unwrap()
    }

    pub fn build_json_signature(&self) -> String{
        let json_result = json!({
            "sender": &self.sender,
            "recipient": &self.recipient,
            "amount": self.amount, 
        });
        serde_json::to_string(&json_result).unwrap()
    }

    pub fn current_time_millis() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_secs() * 1000 + u64::from(since_the_epoch.subsec_millis())
    }

    pub fn format_unix_millis(unix_millis: u64) -> String {
        let dt = UNIX_EPOCH + std::time::Duration::from_millis(unix_millis);
        let datetime_utc: DateTime<Utc> = dt.into();
        let datetime_local: DateTime<Local> = dt.into();
    
        // Форматируем время в строку
        let formatted_utc = datetime_utc.format("%Y-%m-%d %H:%M:%S.%3f UTC");
        let formatted_local = datetime_local.format("%Y-%m-%d %H:%M:%S.%3f Local");
    
        format!("UTC: {}, Local: {}", formatted_utc, formatted_local)
    }

    pub fn set_timestamp(&mut self) {
        self.timestamp = Some(Self::current_time_millis());
    }

    pub fn set_fee(&mut self, fee: f64) {
        self.fee = Some(fee);
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone().unwrap_or_default()
    }
    pub fn get_sender(&self) -> String {
        self.sender.clone()
    }

    pub fn get_recipient(&self) -> String {
        self.recipient.clone()
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

}

