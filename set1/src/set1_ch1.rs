use base64::{engine::general_purpose::STANDARD, Engine as _};
use hex;

pub fn hex_to_base64(hex: &str) -> String {
    println!("Hex format : {}", hex);
    let bytes = hex::decode(hex).unwrap();
    let b64 = STANDARD.encode(&bytes);
    println!("Base64 encoded : {}", b64);
    b64
}
