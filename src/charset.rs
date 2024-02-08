/// Represents the character set used in password generation.
///
/// The character set includes:
/// - Lowercase alphabet
/// - Uppercase alphabet
/// - Digits (0-9)
/// - Special characters (non-alphanumeric and non-control ASCII characters)
pub struct Charset {
	pub lowercase_alphabet: Vec<char>,
	pub uppercase_alphabet: Vec<char>,
	pub digits: Vec<u8>,
	pub special: Vec<char>,
}

impl Charset {
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
    pub fn new() -> Self {
		let uppercase_alphabet: Vec<char> = (b'A'..=b'Z') // get all ascii byte numbers from A to Z
			.map(|c| c as char) // map these numbers into chars
			.filter(|c| c.is_alphabetic()) // filter out all chars, that arent alphabet, this actually chooses the corresponding chars
			.collect::<Vec<_>>(); // collect the chars into a vec
			// all other vecs are generated the same way
			// i deliberately decided against .filter_map, as this way is easier to understand
			// the ascii byte numbers correspond to https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)

		let lowercase_alphabet: Vec<char> = (b'a'..=b'z')
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

		Self { lowercase_alphabet, uppercase_alphabet, digits, special } 
	}
}

/// Represents the character types that can be used in password generation.
///
/// The available character types are:
/// - Lowercase letters
/// - Uppercase letters
/// - Digits
/// - Special characters
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum CharTypes {
	LowercaseLetters,
	UppercaseLetters,
	Digits,
	Special
}