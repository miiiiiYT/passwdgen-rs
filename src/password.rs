use std::collections::HashSet;
use rand::prelude::*;
use rand_chacha::rand_core::CryptoRngCore;

use crate::charset::{CharTypes, Charset};

/// Represents the options for password generation.
///
/// Users can specify the character types and length of the generated password.
pub struct PasswordOptions {
    pub char_types: HashSet<CharTypes>,
    pub length: u32,
    pub amount: u32,
    pub has_prefix: bool,
}

impl PasswordOptions {
    pub fn new() -> Self { Self { char_types: HashSet::<CharTypes>::new(), length: 0, amount: 0, has_prefix: false }}
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
pub fn create_password<R: CryptoRngCore>(options: &PasswordOptions, charset: &Charset, rng: &mut R ) -> String {
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