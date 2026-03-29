use std::{
    fs::File,
    io::{self, Read, copy},
};

fn process_reader<R: Read, W: io::Write>(reader: R, writer: &mut W) -> io::Result<()> {
    let mut reader = reader;
    copy(&mut reader, writer)?;
    Ok(())
}

fn print_file<W: io::Write>(file_path: &str, writer: &mut W) -> io::Result<()> {
    let reader = File::open(file_path)?;
    process_reader(reader, writer)?;
    Ok(())
}
pub fn print_files(file_paths: &[String]) -> io::Result<()> {
    let mut had_error = false;
    let mut writer = io::stdout().lock();
    if file_paths.is_empty() {
        if let Err(e) = process_reader(io::stdin().lock(), &mut writer) {
            eprintln!("{}", e);
            had_error = true;
        }
    } else {
        for file_path in file_paths {
            if let Err(e) = print_file(file_path, &mut writer) {
                eprintln!("cat: {}: {}", file_path, e);
                had_error = true;
            }
        }
    }
    if had_error {
        return Err(io::ErrorKind::Other.into());
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::run::process_reader;

    #[test]
    fn test_process_reader() {
        let reader = b"hello\nrust cat\n";
        let mut writer = Vec::new();
        let _ = process_reader(reader.as_slice(), &mut writer);
        assert_eq!(writer, b"hello\nrust cat\n");
    }
}
