use crypto_lib::utils::score_text;

pub fn decipher_single_byte_xor(hex: &str) -> (f64, String) {
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
    (best_score, best_candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_13() {
        let expected = "Cooking MC's like a pound of bacon";
        assert_eq!(
            decipher_single_byte_xor(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            )
            .1,
            expected
        );
    }
}
