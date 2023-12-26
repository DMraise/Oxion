use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use serde::{Serialize, Deserialize};
use actix_web::guard;
use crate::cryptography::ecdsa::ECDSA;
use crate::strucks::transaction::Transaction;
use crate::blockchain::Blockchain;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;

#[derive(Serialize)]
struct MessageKeys {
    public_key: String,
    secret_key: String
}

#[derive(Serialize)]
struct MessageBuild {
    data: MessageKeys,
    code: i32
}

#[derive(Deserialize)]
struct ParamsGet {
    public_key: String
}

struct ServerState {
    blockchain: Arc<Mutex<Blockchain>>,
    queue: mpsc::Sender<Transaction>,
}

async fn index() -> impl Responder {
    "Hello, World!"
}

async fn gen_keys() -> HttpResponse {
    let (public_key, private_key) = ECDSA::generate_keys();

    let keys = MessageKeys {
        public_key: public_key.to_string(),
        secret_key: private_key.to_string()
    };

    let json = MessageBuild {
        data: keys,
        code: 0
    };

    let json_data = serde_json::to_string(&json).unwrap();
    HttpResponse::Ok().body(json_data)
}

async fn get() -> impl Responder {
    "Hello, Python!"
}

// async fn get_block(blockchain_data: web::Data<ServerState>) -> impl Responder {
//     let _blockchain = blockchain_data.blockchain.lock().unwrap();
//     blockchain_data.blockchain.get_block();
// }

async fn get_balance(s: web::Data<ServerState>, params: web::Query<ParamsGet>) -> impl Responder {
    let locked_blockchain = &s.blockchain.lock().unwrap();
    let result;
    match locked_blockchain.get_balance_account(params.public_key.clone()) {
        Some(data) => result = format!("Balance {} is {}", params.public_key, data),
        None => result = format!("Balance {} is None", params.public_key),
    }
    return result;
} 

async fn sign_data(data: web::Json<Transaction>, request: HttpRequest, blockchain_data: web::Data<ServerState>) -> HttpResponse {
    let _blockchain = blockchain_data.blockchain.lock().unwrap();

    let mut transaction = data.0;
    let signature: String;

    let headers = request.headers();

    if let Some(signature_h) = headers.get("SIGNATURE") {
        signature = signature_h.to_str().unwrap_or_default().to_string();
    } else {
        return HttpResponse::Ok().body("Signature must be empty");
    }

    let _ = &transaction.set_timestamp();
    let _ = &transaction.calculate_hash();

    let data_transaction = &transaction.build_json_signature();

    let verify_ecdsa = ECDSA::verify(&data_transaction, &transaction.sender, &signature);


    if verify_ecdsa {
        let _ = blockchain_data.queue.send(transaction).unwrap();
        HttpResponse::Ok().body("OK") 
    } else {
        HttpResponse::Ok().body("Signature error.") 
    }
}


// #[actix_web::main]
pub async fn run_server(sender: mpsc::Sender<Transaction>, blockchain_main: Arc<Mutex<Blockchain>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        
            .app_data(web::Data::new(ServerState {
                blockchain: blockchain_main.clone(),
                queue: sender.clone()
            }))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/get").route(web::get().to(get)))
            .service(web::resource("/gen").route(web::get().to(gen_keys)))
            .service(web::resource("/transaction").route(web::post().to(sign_data)))
            .service(web::resource("/get/balance").route(web::get().to(get_balance)))
            .default_service(web::route().guard(guard::Not(guard::Get())).to(HttpResponse::NotFound))
    })
    .bind("127.0.0.1:80")?
    .run()
    .await
}

