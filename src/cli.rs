use std::collections::HashSet;

use clap::Parser;

use crate::{charset::CharTypes, get_char_types, password::PasswordOptions, user_error};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the password(s) to generate
    #[arg(short, long)]
    length: u32,

    /// Number of times to greet
    #[arg(short, long, default_value = "all")]
    types: String,

    /// Amount of passwords to generate
    #[arg(short, long, default_value_t = 1)]
    amount: u32,

    /// Whether or not to prepend an index to the password
    #[arg(short, long, default_value_t = false)]
    prefix: bool,
}

pub fn cli_run() -> PasswordOptions {
    let args = Args::parse();

    let char_types: HashSet<CharTypes> = match get_char_types(args.types) {
        Some(ct) => ct,
        None => {user_error!("Invalid character type(s) specified");},
    };


    PasswordOptions { char_types: char_types, length: args.length, amount: args.amount, has_prefix: args.prefix }
}