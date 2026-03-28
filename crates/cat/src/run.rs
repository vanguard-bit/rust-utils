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

pub fn print_files(file_paths: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut had_error = false;
    if file_paths.is_empty() {
        if let Err(e) = process_reader(io::stdin().lock()) {
            eprintln!("{}", e);
            had_error = true;
        }
    } else {
        for file_path in file_paths.iter() {
            if let Err(e) = print_file(&file_path) {
                eprintln!("{}", e);
                had_error = true;
            }
        }
    }
    if had_error {
        Err("Error(s) occured.".into())
    } else {
        Ok(())
    }
}
