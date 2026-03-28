use clap::Parser;

/// Simple Cat Implementation
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File name
    #[arg(short, long)]
    pub name: Vec<String>,
}
