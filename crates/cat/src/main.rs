mod cli;
mod reader;
mod run;

use std::process;

use clap::Parser;
fn main() {
    let args = cli::Args::parse();
    // TODO: implement multiple file reader
    if let Err(e) = run::print_file(&args.name[0]) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
