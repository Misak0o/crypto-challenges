use crypto_lib::utils::fixed_xor;

pub fn xor_fixed(hex1: &str, hex2: &str) -> String {
    fixed_xor(hex1, hex2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_12() {
        let expected = "746865206b696420646f6e277420706c6179";
        assert_eq!(
            xor_fixed(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            expected
        );
    }
}
