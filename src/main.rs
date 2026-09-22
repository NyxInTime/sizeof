use clap::{Parser, ValueEnum};
use colored_text::Colorize;
use dir_walker::Walker;
use humansize::{BINARY, DECIMAL, WINDOWS, format_size};
use std::{
    collections::HashSet,
    fs::{self, Metadata},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

/// Simple program to get the size of a file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /*
    /// Whether to print in tree format or not
    #[arg(short, default_value_t = false)]
    tree: bool,
    */
    /// Number formats:
    #[arg(short, value_enum, default_value_t = Formats::BINARY)]
    format: Formats,

    /// Whether to count hardlinks multiple times
    #[arg(short, default_value_t = false)]
    links: bool,

    file: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
#[value(rename_all = "lowercase")]
enum Formats {
    BINARY,
    DECIMAL,
    WINDOWS,
}

#[derive(ValueEnum, Clone, Debug)]
enum Commands {}
fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    //match args.tree {
    match false {
        true => Ok(()),
        false => {
            //TODO Make my command return basically the same as du -sh .
            let mut seen_inodes = HashSet::new();
            let file = match args.file {
                Some(ref f) => f,
                None => &".".into(),
            };
            let metadata = match fs::metadata(&file) {
                Ok(m) => m,
                Err(e) => return Err(e),
            };

            let len: (u64, u64) = match metadata.is_dir() {
                true => {
                    let walker = Walker::new(file).walk_dir().unwrap();
                    let x = walker.into_iter().fold((0, 0), |(a, b), x| {
                        match fs::metadata(x.dirent.path()) {
                            Ok(m) => match args.links {
                                true => sizes(a, b, m),
                                false => {
                                    if seen_inodes.insert(m.ino()) {
                                        sizes(a, b, m)
                                    } else {
                                        (a, b)
                                    }
                                }
                            },
                            Err(_) => (a, b),
                        }
                    });
                    //x.to_string()
                    x
                }
                false => (metadata.blocks() * 512, metadata.len()),
            };

            let file_colored = match metadata.is_dir() {
                true => file.display().blue(),
                false => file.display().white(),
            };

            let human_len = match args.format {
                Formats::BINARY => (format_size(len.0, BINARY), format_size(len.1, BINARY)),
                Formats::DECIMAL => (format_size(len.0, DECIMAL), format_size(len.1, DECIMAL)),
                Formats::WINDOWS => (format_size(len.0, WINDOWS), format_size(len.1, WINDOWS)),
            };

            println!("{} | {} | {}", file_colored, human_len.0, human_len.1);
            Ok(())
        }
    }
}

fn sizes(a: u64, b: u64, m: Metadata) -> (u64, u64) {
    (a + m.blocks() * 512, b + m.len())
}
