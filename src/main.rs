use std::io;

fn generate_alphabet() -> Vec<char> {
    let alphabet = ('a'..='z') // Start as u8
        .map(|c| c as char) // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect::<Vec<_>>(); // Collect as Vec<char>

    return alphabet;
}

fn find_letter_index_in_alphabet(letter: char, alphabet: Vec<char>) -> Option<usize> {
    alphabet.iter().position(|&x| x == letter)
}

fn find_shifted_letter(letter: char, shift: u32, alphabet: Vec<char>) -> char {
    let index = find_letter_index_in_alphabet(letter, alphabet.clone());
    match index {
        Some(i) => {
            let shifted_index = i + shift as usize;
            let safe_index = if shifted_index > alphabet.len() {
                alphabet.len() - i
            } else {
                shifted_index
            };
            return alphabet[safe_index];
        }
        None => panic!("No index!"),
    }
}

enum Cipher {
    _Caesar,
}

impl Cipher {
    fn caesar(input: String, shift: u32) -> String {
        let mut letters: Vec<char> = input.chars().collect();
        let alphabet = generate_alphabet();

        letters.truncate(letters.len() - 1);

        let shifted_letters: Vec<char> = letters
            .iter()
            .map(|&letter| find_shifted_letter(letter, shift, alphabet.clone()))
            .collect();

        let shifted_word: String = shifted_letters.into_iter().collect();

        return shifted_word;
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
