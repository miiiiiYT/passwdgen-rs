use std::{io::{self, BufRead},collections::HashSet};
use rand::prelude::*;
use rand_chacha::rand_core::CryptoRngCore;

macro_rules! user_error {
    ($message:expr) => {
        eprintln!("Error: {}", $message);
        std::process::exit(1);
    };
}

struct Charset {
    lowercase_alphabet: Vec<char>,
    uppercase_alphabet: Vec<char>,
    digits: Vec<u8>
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum CharTypes {
    LowercaseLetters,
    UppercaseLetters,
    Digits
}

struct PasswordOptions {
    char_types: HashSet<CharTypes>,
    length: u32
}

fn main() {
    let mut rng: rand_chacha::ChaCha20Rng = rand_chacha::ChaCha20Rng::from_entropy();
    let charset: Charset = generate_charset();

    let length: u32 = match convert_to_number(get_input("Password length: ")) {
        Some(l) => l,
        None => {user_error!("Invalid length given.");},
    };

    println!("Please choose the types of characters to use. Add all of the numbers together you want to use.");
    println!("Lowercase letters = 1");
    println!("Uppercase letters = 2");
    println!("Digits = 4");
    let char_types: HashSet<CharTypes> = match convert_to_number(get_input("Please input a number: ")) {
        Some(1) => HashSet::from([CharTypes::LowercaseLetters]),
        Some(2) => HashSet::from([CharTypes::UppercaseLetters]),
        Some(3) => HashSet::from([CharTypes::LowercaseLetters, CharTypes::UppercaseLetters]),
        Some(4) => HashSet::from([CharTypes::Digits]),
        Some(5) => HashSet::from([CharTypes::LowercaseLetters, CharTypes::Digits]),
        Some(6) => HashSet::from([CharTypes::UppercaseLetters, CharTypes::Digits]),
        Some(7) => HashSet::from([CharTypes::LowercaseLetters, CharTypes::UppercaseLetters, CharTypes::Digits]),
        Some(_) => {user_error!("Invalid charater type given.");},
        None => {user_error!("Invalid charater type given.");},
    };

    let amount: u32 = match convert_to_number(get_input("Amount of passwords to generate: ")) {
        Some(a) => a,
        None => {user_error!("Invalid amount given.");},
    };

    let has_prefix: bool = match get_input("Should the passwords be prefixed with their index (e.g. 1: passwd)? [Y/n]: ").as_str() {
        "y" => true,
        "n" => false,
        "" => true,
        _ => {user_error!("invalid input");},
    };

    let options: PasswordOptions = PasswordOptions { char_types: char_types, length: length };

    for i in 1..=amount {
        let password = create_password(&options, &charset, &mut rng);
        if has_prefix {
            println!("{}", i.to_string() + ": " + &password);
        } else {
            println!("{}", password);
        }
    }
    

    /* leaving this in because i enjoy the 
    message of the unreachable! and have yet
    to find a good place to use it somewhere else 
    so ig its living in the comments
    
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
    } */

    
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

fn create_password<R: CryptoRngCore>(options: &PasswordOptions, charset: &Charset, rng: &mut R ) -> String {
    let mut password: String = String::new();
    for _ in 0..options.length {
        let char_type = options.char_types.iter().cloned().collect::<Vec<_>>();
        match char_type.choose(rng) {
            Some(CharTypes::LowercaseLetters) => password.push(*charset.lowercase_alphabet.choose(rng).unwrap()),
            Some(CharTypes::UppercaseLetters) => password.push(*charset.uppercase_alphabet.choose(rng).unwrap()),
            Some(CharTypes::Digits) => {
                let digit = *charset.digits.choose(rng).unwrap();
                password.push(char::from_u32(digit.into()).unwrap())
            }
            None => (),
        }
    }

    return password
}