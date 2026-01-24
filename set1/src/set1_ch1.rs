use crypto_lib::utils::hex_to_base64;

pub fn hex_to_b64(hex: &str) -> String {
    hex_to_base64(hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_11() {
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(
            hex_to_b64(
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
            ),
            expected
        );
    }
}
