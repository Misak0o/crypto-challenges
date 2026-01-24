fn score_text(text: &str) -> f64 {
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

pub fn decipher_single_byte_xor(hex: &str) -> String {
    let decoded = hex::decode(hex).unwrap();

    let mut best_score = f64::MIN;
    let mut best_candidate = String::new();

    for key in 0u8..=255 {
        let mut tmp: Vec<u8> = Vec::new();
        for i in 0..decoded.len() {
            tmp.push(decoded[i] ^ key);
        }
        let candidate = String::from_utf8_lossy(&tmp).to_string();

        if candidate.len() > 1 {
            let score = score_text(&candidate);
            if score > best_score {
                best_score = score;
                best_candidate = candidate;
            }
        }
    }
    println!("Deciphered text : {}", best_candidate);
    best_candidate
}
