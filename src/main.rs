use std::{io::{self, BufRead},collections::HashSet};
use rand::prelude::*;
use rand_chacha::rand_core::CryptoRngCore;

/// Print an error message to stderr and exit with a non-zero code.
///
/// # Arguments
///
/// * `$message` - The error message to display.
///
/// # Example
///
/// ```
/// user_error!("Invalid input provided.");
/// ```
macro_rules! user_error {
    ($message:expr) => {
        eprintln!("Error: {}", $message);
        std::process::exit(1);
    };
}

/// Represents the character set used in password generation.
///
/// The character set includes:
/// - Lowercase alphabet
/// - Uppercase alphabet
/// - Digits (0-9)
/// - Special characters (non-alphanumeric and non-control ASCII characters)
struct Charset {
    lowercase_alphabet: Vec<char>,
    uppercase_alphabet: Vec<char>,
    digits: Vec<u8>,
    special: Vec<char>,
}

/// Represents the character types that can be used in password generation.
///
/// The available character types are:
/// - Lowercase letters
/// - Uppercase letters
/// - Digits
/// - Special characters
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum CharTypes {
    LowercaseLetters,
    UppercaseLetters,
    Digits,
    Special
}

/// Represents the options for password generation.
///
/// Users can specify the character types and length of the generated password.
struct PasswordOptions {
    char_types: HashSet<CharTypes>,
    length: u32
}

fn main() {
    // initializing crypto-safe rng
    let mut rng: rand_chacha::ChaCha20Rng = rand_chacha::ChaCha20Rng::from_entropy();
    let charset: Charset = generate_charset();

    let length: u32 = match convert_to_number(get_input("Password length: ")) {
        Some(len) => len,
        None => {user_error!("Invalid length given.");},
    };

    let char_types: HashSet<CharTypes> = match get_char_types() {
        Some(ct) => ct,
        None => {user_error!("Invalid character type(s) specified");},
    };

    let amount: u32 = match convert_to_number(get_input("Amount of passwords to generate: ")) {
        Some(amt) => amt,
        None => {user_error!("Invalid amount given.");},
    };

    let has_prefix: bool = match get_input("Should the passwords be prefixed with their index (e.g. 1: passwd)? [Y/n]: ").as_str() {
        "y" => true,
        "n" => false,
        "" => true, // just pressing enter will choose yes
        _ => {user_error!("invalid input");},
    };

    let options: PasswordOptions = PasswordOptions { char_types: char_types, length: length };

    // generating passwords
    for i in 1..=amount {
        let password = create_password(&options, &charset, &mut rng);
        // depending on has_prefix, print the prefix or drop it
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

// create a charset to use in password generation from scratch
// since just putting the chars into a vec is ugly, this is skillful
//
/// Generates a character set to use in password generation.
///
/// The character set includes the lowercase and uppercase alphabets,
/// digits, and special characters.
///
/// # Returns
///
/// A `Charset` struct representing the generated character set.
fn generate_charset() -> Charset{
    let upper_alphabet: Vec<char> = (b'A'..=b'Z') // get all ascii byte numbers from A to Z
        .map(|c| c as char) // map these numbers into chars
        .filter(|c| c.is_alphabetic()) // filter out all chars, that arent alphabet, this actually chooses the corresponding chars
        .collect::<Vec<_>>(); // collect the chars into a vec
    // all other vecs are generated the same way
    // i deliberately decided against .filter_map, as this way is easier to understand
    // the ascii byte numbers correspond to https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)

    let lower_alphabet: Vec<char> = (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    let digits: Vec<u8> = (b'0'..=b'9')
        .map(|c| c as u8)
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<_>>();

    let special: Vec<char> = (32..=126 as u8) // 32 (0x20) is SP (space), the start of ascii symbols, 126 (0x7e) is ~, the end of symbols. however, inbetween are letters and digits
        .map(|c| c as char)
        // since we dont want the alphanumerics nor the controls (even tho they arent included anyways), we filter them out (using an '!' to invert)
        .filter(|c| !c.is_ascii_alphanumeric())
        .filter(|c| !c.is_ascii_control())
        .collect::<Vec<_>>();

    Charset{lowercase_alphabet: lower_alphabet, uppercase_alphabet: upper_alphabet, digits: digits, special: special}   
}

/// Reads a line of input from the user.
///
/// # Returns
///
/// The user's input as a `String`.
fn read_input() -> String {
    // stolen from https://github.com/miiiiiyt/calc.rs
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap(); // TODO: implement panic safe
    return line
}

/// Prompts the user for input and returns the input as a `String`.
///
/// # Arguments
///
/// * `prompt` - The prompt to display to the user.
///
/// # Returns
///
/// The user's input as a `String`.
fn get_input(prompt: &'static str) -> String {
    print!("{}", prompt); // the flush here is needed, in order to print the prompt 
    io::Write::flush(&mut io::stdout()).expect("flush failed!"); // TODO: implement panic safe flush
    let input = read_input();
    return input
}

/// Converts a string to a `u32`.
///
/// # Arguments
///
/// * `string` - The string to convert.
///
/// # Returns
///
/// An `Option<u32>` representing the parsed value. `None` if parsing fails.
fn convert_to_number(string: String) -> Option<u32> {
    string.parse().ok()
}

// create a password
//                 VVVVVV defining R to be used for the rng argument, since we cant use the trait directly
//
/// Generates a password based on the specified options, character set, and RNG.
///
/// # Arguments
///
/// * `options` - The password generation options.
/// * `charset` - The character set to use in password generation.
/// * `rng` - The random number generator.
///
/// # Returns
///
/// The generated password as a `String`.
fn create_password<R: CryptoRngCore>(options: &PasswordOptions, charset: &Charset, rng: &mut R ) -> String {
    let mut password: String = String::new();
    // generating a character for each index
    for _ in 0..options.length {
        let char_type = options.char_types.iter().cloned().collect::<Vec<_>>(); // turn the char_types from options into a usable vec
        match char_type.choose(rng) { // choose a random character type
            Some(CharTypes::LowercaseLetters) => password.push(*charset.lowercase_alphabet.choose(rng).unwrap()), // choosing a random char from the corresponding charset, and derefing it
            Some(CharTypes::UppercaseLetters) => password.push(*charset.uppercase_alphabet.choose(rng).unwrap()),
            Some(CharTypes::Digits) => {
                let digit = *charset.digits.choose(rng).unwrap(); // extra step needed to convert the u8 into u32
                password.push(char::from_u32(digit.into()).unwrap())
            }
            Some(CharTypes::Special) => password.push(*charset.special.choose(rng).unwrap()),
            None => (),
        }
    }

    return password
}

/// Prompts the user to choose character types for password generation.
///
/// # Returns
///
/// An `Option<HashSet<CharTypes>>` representing the selected character types.
/// `None` if no valid character types are selected.
fn get_char_types() -> Option<HashSet<CharTypes>> {
    println!("Please choose the types of characters to use (comma-separated):");
    println!("Options: lowercase, uppercase, digits, special");

    let input_string = get_input("Please input your choices: ");
    let choices: HashSet<&str> = input_string.split(',').map(|c| c.trim()).collect(); // splitting the input at commas

    let char_types: HashSet<CharTypes> = choices
        .into_iter()
        .flat_map(|choice| match choice { // inserting chartypes into the hashset 
            "lowercase" => Some(CharTypes::LowercaseLetters),
            "uppercase" => Some(CharTypes::UppercaseLetters),
            "digits" => Some(CharTypes::Digits),
            "special" => Some(CharTypes::Special),
            _ => None,
        })
        .collect();
    
    if char_types.is_empty() {
        // if this is not here, the password will the "", so literally nothing. unwanted behavior
        return None
    }

    Some(char_types)
}