use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn get_file(file_path: &str) -> io::Result<Box<dyn BufRead>> {
    let file = File::open(file_path)?;
    Ok(Box::new(BufReader::new(file)))
}
