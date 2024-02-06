use std::io::{self, BufRead};
use rand::prelude::*;

struct Charset {
    lowercase_alphabet: Vec<char>,
    uppercase_alphabet: Vec<char>,
    digits: Vec<u8>
}

fn main() {
    let mut rng: rand_chacha::ChaCha20Rng = rand_chacha::ChaCha20Rng::from_entropy();
    let charset: Charset = generate_charset();

    let length: u32 = match convert_to_number(get_input("Password length: ")) {
        Some(l) => l,
        None => panic!("Invalid length given.")
    };

    let mut password: String = String::new();

    for _ in 0..length {
        match rng.gen_range(0..=2) {
            0 => password.push(*charset.lowercase_alphabet.choose(&mut rng).unwrap()),
            1 => password.push(*charset.uppercase_alphabet.choose(&mut rng).unwrap()),
            2 => {
                let digit = *charset.digits.choose(&mut rng).unwrap();
                password.push(char::from_u32(digit.into()).unwrap())
            }
            _ => unreachable!("The range of 0..=2 will contain 0,1, or 2. If this executes, check the current state of mathematics and consider all that you've known to be invalid"),
        }
    }

    println!("{}", password);
}

fn generate_charset() -> Charset{
    let upper_alphabet: Vec<char> = (b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    let lower_alphabet: Vec<char> = (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    let digits: Vec<u8> = (b'0'..=b'9')
        .map(|c| c as u8)
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<_>>();

    Charset{lowercase_alphabet: lower_alphabet, uppercase_alphabet: upper_alphabet, digits: digits}   
}

fn read_input() -> String {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap(); // TODO: implement panic safe
    return line
}

fn get_input(prompt: &'static str) -> String {
    print!("{}", prompt); // the flush here is needed, in order to print the prompt 
    io::Write::flush(&mut io::stdout()).expect("flush failed!"); // TODO: implement panic safe flush
    let input = read_input();
    return input
}

fn convert_to_number(string: String) -> Option<u32> {
    string.parse().ok()
}