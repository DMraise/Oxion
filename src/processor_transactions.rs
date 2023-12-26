use std::sync::mpsc;
use crate::strucks::transaction::Transaction;
use crate::Blockchain;
use std::sync::{Mutex, Arc};
pub struct Processor {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub queue: mpsc::Receiver<Transaction>,
    pub state: bool,
}

impl Processor {
    pub fn new(blockchain_main: Arc<Mutex<Blockchain>>, recipient: mpsc::Receiver<Transaction>) -> Processor{
        Self {
            blockchain: blockchain_main,
            queue: recipient,
            state: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.queue.recv() {
                Ok(transaction) => {
                    let locked_blockchain = &mut self.blockchain.lock().unwrap();
                    locked_blockchain.transactions.push(transaction.clone());

                    if locked_blockchain.transactions.len() >= 100{
                        self.state = true;
                    }
                } Err(_) => {
                    break;
                }
            }      
            
            if self.state {
                let locked_blockchain = &mut self.blockchain.lock().unwrap();
                locked_blockchain.create_block();
                self.state = false;
            }
            
        }
    }
}