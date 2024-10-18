use clap::Parser;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    // filter_string: String,

    path: PathBuf,
    

    #[arg(short = 'C', long, default_value_t = true)]
    color_output: bool,

    #[arg(short = 'M', long, default_value_t = false)]
    monochrome_output: bool,

    #[arg(short = 'S', long, default_value_t = false)]
    sort_keys: bool,

    #[arg(long, default_value_t = 2, value_parser = clap::value_parser!(u8).range(0..=7))]
    indent: u8,

    #[arg(short, long, default_value_t = false)]
    compact_output: bool,
}

fn main() {
    let args = Args::parse();

    if args.color_output && args.monochrome_output {
        eprintln!("Error: Both color-output and monochrome-output cannot be specified at the same time.");
        std::process::exit(1);
    }

    if args.compact_output && args.indent != 2 {
        eprintln!("Error: Both compact-output and indent cannot be specified at the same time.");
        std::process::exit(1);
    }

    let filter_string: std::io::Result<Vec<String>> = &std::io::stdin().lines().collect()

}
