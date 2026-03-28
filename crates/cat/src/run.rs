use crate::reader::get_file;
use std::{
    error::Error,
    io::{self, BufRead},
};
fn process_reader<R: BufRead>(reader: R) -> io::Result<()> {
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

pub fn print_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let reader = get_file(file_path)?;
    process_reader(reader)?;
    Ok(())
}
