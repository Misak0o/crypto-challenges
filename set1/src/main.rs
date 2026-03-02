mod set1_ch1;
mod set1_ch2;
mod set1_ch3;
mod set1_ch4;
mod set1_ch5;

fn main() {
    set1_ch1::hex_to_b64(
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
    );

    set1_ch2::xor_fixed(
        "1c0111001f010100061a024b53535009181c",
        "686974207468652062756c6c277320657965",
    );

    set1_ch3::decipher_single_byte_xor(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    );

    set1_ch4::solve();

    set1_ch5::encrypt_repeating_key_xor(
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
        "ICE",
    );
}
