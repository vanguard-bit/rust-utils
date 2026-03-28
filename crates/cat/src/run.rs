use std::{
    fs::File,
    io::{self, Read, copy},
};

fn process_reader<R: Read>(reader: R) -> io::Result<()> {
    let mut writer = io::stdout().lock();
    let mut reader = reader;
    copy(&mut reader, &mut writer)?;
    Ok(())
}

pub fn print_files(file_paths: &[String]) -> io::Result<()> {
    let mut had_error = false;
    let mut writer = io::stdout().lock();
    if file_paths.is_empty() {
        if let Err(e) = process_reader(io::stdin().lock()) {
            eprintln!("{}", e);
            had_error = true;
        }
    } else {
        for file_path in file_paths {
            match File::open(file_path) {
                Err(e) => {
                    eprintln!("{}", e);
                    had_error = true;
                }
                Ok(reader) => {
                    let mut reader = reader;
                    copy(&mut reader, &mut writer)?;
                }
            }
        }
    }
    if had_error {
        Err(io::Error::new(io::ErrorKind::Other, "Error(s) occurred"))
    } else {
        Ok(())
    }
}
