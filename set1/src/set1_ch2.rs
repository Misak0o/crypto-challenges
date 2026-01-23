pub fn fixed_xor(hex1: &str, hex2: &str) -> String {
    let decoded1 = hex::decode(hex1).unwrap();
    let decoded2 = hex::decode(hex2).unwrap();

    let mut result: Vec<u8> = vec![];

    for i in 0..decoded1.len() {
        result.push(decoded1[i] ^ decoded2[i]);
    }

    let result = hex::encode(result);
    println!("Xored result : {}", result);
    result
}
