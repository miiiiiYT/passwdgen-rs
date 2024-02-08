pub mod password;
pub mod charset;
pub mod util;
pub mod file_export;

use std::{collections::HashSet, vec};
use rand::prelude::*;

use password::{PasswordOptions, create_password};
use charset::{Charset, CharTypes};
use util::{convert_to_number, get_input, Output};
use file_export::{write, init_path};

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


fn main() {
    // initializing crypto-safe rng
    let mut rng: rand_chacha::ChaCha20Rng = rand_chacha::ChaCha20Rng::from_entropy();
    let charset: Charset = Charset::new();

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
    let mut passwords: Vec<String> = Vec::new();
    for i in 1..=amount {
        let password = create_password(&options, &charset, &mut rng);
        // depending on has_prefix, print the prefix or drop it
        if has_prefix {
            passwords.push(i.to_string() + ": " + &password);
        } else {
            passwords.push(password);
        }
    }

    let output: Output = match get_output_type(false) {
        Some(o) => o,
        None => {user_error!("You did not provide a valid output type");},
    };

    match output {
        Output::Stdout => {
            for password in passwords {
                println!("{}", password);
            }
        },
        Output::File => {
            println!("Writing to file.");
            let input = &get_input("Please supply a file name: ");
            let mut file = match init_path(input.as_str()) {
                Ok(p) => {
                    println!("Writing to {}", p.1);
                    p
                },
                Err(e) => {user_error!(format!("An error occured opening the file: {}", e));}
            };
            let result = write(&passwords.join("\n"), &mut file.0);
            if result.0 {
                println!("Wrote to {}", file.1);
            } else {
                eprintln!("Writing to file failed: {}", result.1.unwrap())
            }
        },
        Output::Commandline => {
            unimplemented!("used from commandline");
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

/// Prompts the user to choose character types for password generation.
///
/// # Returns
///
/// An `Option<HashSet<CharTypes>>` representing the selected character types.
/// `None` if no valid character types are selected.
fn get_char_types() -> Option<HashSet<CharTypes>> {
	println!("Please choose the types of characters to use (comma-separated):");
	println!("Options: lowercase, uppercase, digits, special, all");

	let input_string = get_input("Please input your choice(s): ");
	let choices: HashSet<&str> = input_string.split(',').map(|c| c.trim()).collect(); // splitting the input at commas

	let char_types: HashSet<CharTypes> = choices
		.into_iter()
		.flat_map(|choice| match choice { // inserting chartypes into the hashset 
			"lowercase" => vec![CharTypes::LowercaseLetters],
			"uppercase" => vec![CharTypes::UppercaseLetters],
			"digits" => vec![CharTypes::Digits],
			"special" => vec![CharTypes::Special],
            "all" => vec![CharTypes::LowercaseLetters, CharTypes::UppercaseLetters, CharTypes::Digits, CharTypes::Special],
			_ => vec![],
		})
		.collect();

    //"all" => Some(CharTypes::LowercaseLetters, CharTypes::UppercaseLetters, CharTypes::Digits, CharTypes::Special),
	
	if char_types.is_empty() {
		// if this is not here, the password will the "", so literally nothing. unwanted behavior
		return None
	}

	Some(char_types)
}

fn get_output_type(cli: bool) -> Option<Output> {
    if cli {
        return Some(Output::Commandline)
    }
    println!("How do you want the passwords to be outputted?");
    println!("Options: stdout, file");

    match get_input("Please input one option (standard: stdout): ").as_str() {
        "stdout" => Some(Output::Stdout),
        "file" => Some(Output::File),
        "" => Some(Output::Stdout),
        _ => None
    }
}