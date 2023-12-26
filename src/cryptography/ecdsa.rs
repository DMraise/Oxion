use secp256k1::{Secp256k1, SecretKey, PublicKey};
use secp256k1::Message;
use secp256k1::ecdsa::Signature;
use sha2::{Digest, Sha256};
use hex::encode;
use hex::FromHex;
use secp256k1::rand::rngs::OsRng;

pub struct ECDSA;

impl ECDSA {
    pub fn generate_keys() -> (String , String) {
        let secp = Secp256k1::new();

        // let mut rng = rand::thread_rng();
        // let mut array = [0u8; 32];
        // rng.fill_bytes(&mut array);

        // let secret_key = SecretKey::from_slice(&array).expect("32 bytes");

        // let public_key_uncompressed = PublicKey::from_secret_key(&secp, &secret_key);
        // let public_key_hex = encode(&public_key_uncompressed.serialize_uncompressed());

        // let private_key_hex = encode(&secret_key[..]);
        // let end_time = Instant::now();

        // let elapsed_time = end_time.duration_since(start_time);

        // println!("Функция выполнена за {:?}", elapsed_time);
        // (public_key_hex, private_key_hex)
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        
        let public_key = hex::encode(public_key.serialize_uncompressed());
        let secret_key = hex::encode(secret_key.secret_bytes());

        (public_key, secret_key)
    }

    #[allow(dead_code)]
    pub fn sign_message(data: &String, secret_key_hex: &String) -> String {
        let secp = Secp256k1::new();

        let bytes_secret_key = Vec::from_hex(secret_key_hex).expect("Failes to parse hex secret key");
        let secret_key = SecretKey::from_slice(&bytes_secret_key).expect("Invalid secret key format");

        let data_bytes = data.as_bytes();
        let message_hash = Sha256::digest(data_bytes);
        let message = Message::from_digest_slice(&message_hash);

        let signature = secp.sign_ecdsa(&message.unwrap(), &secret_key);

        let signature_hex = encode(&signature.serialize_der()[..]);
        signature_hex
    }

    pub fn verify(data: &String, public_key_hex: &String, signature_hex: &String) -> bool{
        let secp = Secp256k1::new();

        let data_bytes = data.as_bytes();
        let message_hash = Sha256::digest(data_bytes);
        let signature_bytes = hex::decode(signature_hex).expect("Failed to parse hex signature");
        let signature;

        if signature_hex.len() == 128 {
            signature = Signature::from_compact(&signature_bytes).expect("Failed to parse signature bytes");
        }  else {
            signature = Signature::from_der(&signature_bytes).expect("Failed to parse signature bytes");
        }

        let bytes_public_key = hex::decode(public_key_hex).expect("Failes to parse hex public key");
        let public_key = PublicKey::from_slice(&bytes_public_key).expect("Invalid public key format");

        let message = Message::from_digest_slice(&message_hash);
        let is_valid = secp.verify_ecdsa(&message.unwrap(), &signature, &public_key).is_ok();
        is_valid
    }
}

