use std::fs::File;
use std::io::{self, prelude::*, Error};
use std::path::{Display, Path};

pub fn init_path(path: &str) -> io::Result<(File, Display)> {
	let path_ = Path::new(path);
	let display = path_.display();

	// Open a file in write-only mode, returns `io::Result<File>`
    let file = match File::create(&path) {
        Err(why) => {
			return Err(why)
		},
        Ok(file) => file,
    };

	Ok((file, display))
}

pub fn write(text: &str, file: &mut File) -> (bool, Option<Error>) {
    // Write the `text` string to `file`, returns `io::Result<()>`
    match file.write_all(text.as_bytes()) {
        Err(why) => (false, Some(why)),
        Ok(_) => (true, None),
    }
}