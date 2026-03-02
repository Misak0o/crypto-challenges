pub fn encrypt_repeating_key_xor(plain_text: &str, key: &str) -> String {
    let p_bytes = plain_text.as_bytes();
    let k_bytes = key.as_bytes();

    (0..plain_text.len())
        .map(|i| format!("{:02x}", p_bytes[i] ^ k_bytes[i % key.len()]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_15() {
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
                        a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        assert_eq!(
            encrypt_repeating_key_xor(
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
                "ICE"
            ),
            expected
        );
    }
}
