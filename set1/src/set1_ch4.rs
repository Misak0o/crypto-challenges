use crate::set1_ch3::decipher_single_byte_xor;

pub fn solve() -> (f64, String) {
    let input = include_str!("../ressources/4.txt");
    let mut best_score = f64::MIN;
    let mut best_candidate = String::new();
    for line in input.lines() {
        let (score, candidate) = decipher_single_byte_xor(&line);
        if score > best_score {
            best_candidate = candidate;
            best_score = score;
        }
    }
    (best_score, best_candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_14() {
        let expected = "Now that the party is jumping\n";
        assert_eq!(solve().1, expected);
    }
}
