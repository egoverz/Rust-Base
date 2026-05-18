use std::io::{self, Read};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    
    let string_count: u32 = input.iter().filter(|byte| byte == &&10).count() as u32;
    let byte_count = input.len();

    let mut word_count: u32 = 0;
    let mut in_word = false;

    for byte in input {
        if byte.is_ascii_whitespace() {
            in_word = false;
        } else if !in_word {
            word_count += 1;
            in_word = true;
        }
    }

    println!("{} {} {}", string_count, word_count, byte_count);


}
