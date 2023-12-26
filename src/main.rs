use std::sync::mpsc;
use std::sync::{Arc, Mutex};
mod strucks;
mod cryptography;
mod server;
mod blockchain;
mod processor_transactions;

use processor_transactions::Processor;
use blockchain::Blockchain;



fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    {
        let mut locked_blockchain = blockchain.lock().unwrap();
        let _ = locked_blockchain.load();
    }

    let (sender, recipient) = mpsc::channel();

    // let mut proccesor_module = Processor {
    //     blockchain: Blockchain::new(),
    //     queue: recipient,
    //     state: false,
    // };
    let mut proccesor_module = Processor::new(blockchain.clone(), recipient);
    

    let processor_thread = std::thread::spawn(move || {
        proccesor_module.run();
    });

    let server_thread = std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(server::server::run_server(sender, blockchain))
            .unwrap();
    });
    
    println!("Main Modules Start... OK");
    server_thread.join().unwrap();
    processor_thread.join().unwrap();
}