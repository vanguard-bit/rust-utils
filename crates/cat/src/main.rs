mod cli;
mod reader;
mod run;

use std::process;

use clap::Parser;
fn main() {
    let args = cli::Args::parse();
    if let Err(_) = run::print_files(&(args.files)) {
        process::exit(1);
    }
}
