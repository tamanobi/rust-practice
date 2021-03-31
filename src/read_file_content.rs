use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

pub fn perform(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(error) => return Err(error),
    };

    Ok(string)
}
