use rocksdb::{DB, Options, Error};
use crate::strucks::transaction::Transaction;
use crate::strucks::block::Block;
use crate::strucks::wallet::Wallet;

pub struct Blockchain {
    pub database_b: Option<DB>,
    pub database_s: Option<DB>,

    pub index_last_block: u64,
    pub previous_hash_block: Option<String>,
    pub transactions: Vec<Transaction>,
}


impl Blockchain {
    pub fn new() -> Self {
        Self {
            database_b: None,
            database_s: None,
            index_last_block: 0,
            previous_hash_block: None,
            transactions: Vec::new(),
        }
    }

    fn load_database_blockchain(&mut self) -> Result<(), Error> {
        let mut options = Options::default();
        options.create_if_missing(true);

        let path = "B:/CoreRUST/BlockchainDatabase";

        let db_state = DB::open_default(path)?;
        self.database_b = Some(db_state);
        Ok(())
    }

    fn load_database_state(&mut self) -> Result<(), Error> {
        let mut options = Options::default();
        options.create_if_missing(true);

        let path = "B:/CoreRUST/DatabaseState";

        let db_state = DB::open_default(path)?;
        self.database_s = Some(db_state);
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.load_database_blockchain() {
            Ok(_) => {},
            Err(err) => eprintln!("Error load database blockchain: {}", err),
        }

        match self.load_database_state() {
            Ok(_) => {},
            Err(err) => eprintln!("Error load database state: {}", err),
        }

        match self.get_last_block() {
            Some(block) => {
                self.index_last_block = block.get_index();
                self.previous_hash_block = Some(block.get_hash());
                println!("{}", block.build_json_print());
            }
            None => {
                println!("Начальный блок не загружен.");
            }
        };
        Ok(())
    }



    pub fn get_last_block(&self) -> Option<Block> {
        if let Some(ref database) = self.database_b {
            let mut iterator = database.iterator(rocksdb::IteratorMode::Start);
            let mut max_key: u64 = 0;

            while let Some(Ok(item_option)) = iterator.next() {
                if let (key, _) = item_option {
                    let key_u64 = String::from_utf8_lossy(key.as_ref()).parse::<u64>().expect("Не удалось извлечь значение ключа.");
                    if max_key < key_u64 {
                        max_key = key_u64.clone();
                    }
                }
            }

            match database.get(max_key.to_string().as_bytes()) {
                Ok(Some(value)) => {
                    let last_block: Block = serde_json::from_slice(&value).unwrap();
                    return Some(last_block);
                },
                Ok(None) => {
                    println!("Блок не найден.")
                },
                Err(_) => {
                    println!("Ошибка получения блока.")
                },
            }
        } else {
            eprintln!("Error: Database not load.")
        }
        return None;
    }

    pub fn get_block(&self, key: u64) -> Option<Block> {
        if let Some(ref db) = self.database_b {
            match db.get(key.to_string().as_bytes()) {
                Ok(Some(value)) => {
                    let block: Block = serde_json::from_slice(&value).unwrap();
                    // println!("Значение: {}", block.build_json_full_print());
                    return Some(block);
                }

                Ok(None) => {
                    println!("Значение не найдено");
                }

                Err(e) => {
                    println!("Ошибка при получении значения: {:?}", e);
                }
            }
        } else {
            println!("Не удалось записать данные. Соединение с БД не установлено.");
        }
        return None;
    }

    pub fn create_block(&mut self) {
        self.index_last_block += 1;

        let mut block = Block::new(self.index_last_block.clone(), self.previous_hash_block.clone().unwrap(), self.transactions.clone());
        let _ = &block.set_timestamp();
        let _ = &block.calculate_hash();
        let _ = self.write_blockchain(self.index_last_block, block.build_json_hash().clone());

        for transaction in block.get_transactions() {
            let sender = transaction.get_sender();
            let recipient = transaction.get_recipient();
            let amount = transaction.get_amount();
            self.update_state(sender, recipient, amount);
        }

        let data = &block.build_json_print();
        println!("{}", data);

        // let mut vector = self.transactions.lock().unwrap();
        // vector.clear();
        // vector.shrink_to_fit();
        self.transactions.clear();
        self.transactions.shrink_to_fit();
        
        self.previous_hash_block = Some(block.get_hash());
    }

    pub fn write_blockchain(&self, key: u64, value: String) {
        if let Some(ref db) = self.database_b {
            db.put(key.to_string().as_bytes(), value.as_bytes()).unwrap();
        } else {
            println!("Не удалось записать данные. Соединение с БД не установлено.");
        }
    }

    pub fn delete_blockchain(&self, key: u64) {
        if let Some(ref db) = self.database_b {
            db.delete(key.to_string().as_bytes()).unwrap();
        } else {
            println!("Не удалось записать данные. Соединение с БД не установлено.");
        }
    }





    pub fn update_state(&mut self, sender: String, recipient: String, amount: f64) -> Option<f64> {
        if let Some(ref db) = self.database_s {
            let mut sender_object: Wallet;
            let mut recipient_object: Wallet;

            match db.get(sender.as_bytes()) {
                Ok(Some(value)) => {
                    sender_object = serde_json::from_slice(&value).unwrap();
                }
                Ok(None) => {
                    sender_object = Wallet::new(sender.clone());
                }
                Err(e) => {
                    println!("Не удалось получить значение. {}", e);
                    return None
                }
            }

            match db.get(recipient.as_bytes()) {
                Ok(Some(value_recipient)) => {
                    recipient_object = serde_json::from_slice(&value_recipient).unwrap();
                }
                Ok(None) => {
                    recipient_object = Wallet::new(recipient.clone());
                }
                Err(e) => {
                    println!("Не удалось получить значение. {}", e);
                    return None
                }
            }

            sender_object.subtraction(amount);
            recipient_object.addition(amount);

            let sender_object_string = serde_json::to_string(&sender_object).unwrap();
            let recipient_object_string = serde_json::to_string(&recipient_object).unwrap();
            
            db.put(sender.as_bytes(), sender_object_string.as_bytes()).unwrap();
            db.put(recipient.as_bytes(), recipient_object_string.as_bytes()).unwrap();
            return Some(amount);
        } else {
            println!("Не удалось записать данные. Соединение с БД не установлено.");
        }
        None
    }

    pub fn get_balance_account(&self, public_key: String) -> Option<f64> {
        if let Some(ref db) = self.database_s {
            match db.get(public_key.as_bytes()) {
                Ok(Some(value)) => {
                    let wallet: Wallet = serde_json::from_slice(&value).unwrap();
                    return Some(wallet.get_balance());
                }
                Ok(None) => {
                    println!("Значение не найдено");
                }

                Err(e) => {
                    println!("Ошибка при получении значения: {:?}", e);
                }
            }
        }  
        Some(0.0)
    }
}
