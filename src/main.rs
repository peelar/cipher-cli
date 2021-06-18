mod caesar;

use std::io;

enum Cipher {
    _Caesar,
}

impl Cipher {
    fn caesar(input: String, shift: u32) -> String {
        caesar::cipher(input, shift)
    }
}

fn main() {
    println!("Welcome to Caesar Cipher tool. The action of a Caesar cipher is to replace each plaintext letter with a different one a fixed number of places down the alphabet.");
    println!("Please provide the word you want to cipher:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Please provide shift:");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(s) => {
            let caeser_ciphered = Cipher::caesar(input, s);
            println!("The ciphered word is: {}", caeser_ciphered)
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}
