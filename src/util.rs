use std::io::{self, BufRead};

pub enum Output {
	Stdout,
	Commandline,
	File,
}

/// Reads a line of input from the user.
///
/// # Returns
///
/// The user's input as a `String`.
pub fn read_input() -> String {
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
pub fn get_input(prompt: &'static str) -> String {
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
pub fn convert_to_number(string: String) -> Option<u32> {
    string.parse().ok()
}