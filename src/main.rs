use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short)]
    tree: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum Commands {}
fn main() {
    let args = Args::parse();
    match args.tree {
        true => todo!(),
        false => todo!(),
    }
}
