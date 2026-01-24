use base64::{Engine as _, engine::general_purpose::STANDARD};
use hex;

pub fn hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    STANDARD.encode(&bytes)
}

pub fn fixed_xor(hex1: &str, hex2: &str) -> String {
    let decoded1 = hex::decode(hex1).unwrap();
    let decoded2 = hex::decode(hex2).unwrap();

    let mut result: Vec<u8> = vec![];

    for i in 0..decoded1.len() {
        result.push(decoded1[i] ^ decoded2[i]);
    }

    let result = hex::encode(result);
    result
}

pub fn score_text(text: &str) -> f64 {
    let frequencies = "etaoin shrdlu";
    let mut score = 0.0;

    for c in text.to_lowercase().chars() {
        if frequencies.contains(c) {
            score += 1.0;
        } else if c.is_ascii_alphabetic() {
            score += 0.1;
        } else if c.is_ascii_whitespace() {
            score += 0.5;
        } else if c.is_ascii_control() {
            score -= 10.0;
        }
    }
    score
}
