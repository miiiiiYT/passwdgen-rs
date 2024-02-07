# passwdgen-rs

A small CLI tool to generate a password, written in Rust.

## Features

- Generate random passwords based on user preferences.
- Choose from different character types, including lowercase letters, uppercase letters, digits, and special characters.
- Specify the length of the generated passwords.
- Option to generate multiple passwords at once.
- Prefix passwords with their index for easy identification.

## Installation

1. Make sure you have [Rust](https://www.rust-lang.org/) installed on your system.
2. Clone this repository:

   ```bash
   git clone https://github.com/your-username/password-generator-rs.git
   ```

3. Navigate to the project directory:

   ```bash
   cd password-generator-rs
   ```

4. Run the project using `cargo`:

   ```bash
   cargo run --release
   ```

   Alternatively, you can build the project and run the generated executable:

   ```bash
   cargo build --release
   ./target/release/password-generator
   ```

5. Follow the prompts to customize your password generation.

## Usage

1. Run the executable as described in the installation steps.
2. Follow the prompts to customize your password generation:
   - Enter the desired password length.
   - Choose the types of characters to include (lowercase, uppercase, digits, special).
   - Specify the number of passwords to generate.
   - Choose whether to prefix passwords with their index.
3. The generated passwords will be displayed on the console.

## Examples

Generate a password with a length of 12 characters, including lowercase letters and digits:

```bash
./target/release/password-generator
Password length: 12
Please choose the types of characters to use (comma-separated):
Options: lowercase, uppercase, digits, special
Please input your choices: lowercase, digits
Amount of passwords to generate: 1
Should the passwords be prefixed with their index (e.g. 1: passwd)? [Y/n]: y
1: abc123xyz456
```

Feel free to contribute, report issues, or suggest improvements!
