mod cli;
mod io;
mod search;

use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    println!("{:?}", args);
    let lines = if let Some(file) = args.file {
        io::get_lines(&std::fs::File::open(file).context("Failed to open file")?)?
    } else {
        io::get_lines(&std::io::stdin())?
    };

    let results = if args.regex {
        search::perform_search(search::RegexSearch::new(&args.needle), lines)
    } else {
        search::perform_search(search::LiteralSearch::new(&args.needle), lines)
    };

    let mut printed_lines = std::collections::HashSet::new();
    for matches in results {
        for m in matches {
            if printed_lines.insert(m.line.clone()) {
                println!("{}", m.line);
            }
        }
    }

    Ok(())
}
