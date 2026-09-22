use clap::{Parser, ValueEnum};
use colored_text::Colorize;
use humansize::{BINARY, format_size};
use std::{fs, path::PathBuf};

/// Simple program to get the size of a file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to print in tree format or not
    #[arg(short, default_value_t = false)]
    tree: bool,

    file: Option<PathBuf>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Commands {}
fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    match args.tree {
        true => Ok(()),
        false => {
            let file = match args.file {
                Some(ref f) => f,
                None => &".".into(),
            };
            let metadata = match fs::metadata(&file) {
                Ok(m) => m,
                Err(e) => return Err(e),
            };
            let len = format_size(metadata.len(), BINARY);
            let file_colored = match metadata.is_dir() {
                true => file.display().blue(),
                false => file.display().white(),
            };

            println!("{} | {}", file_colored, len);
            Ok(())
        }
    }
}
