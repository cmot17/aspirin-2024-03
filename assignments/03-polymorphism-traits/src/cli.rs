use clap::Parser;
use colored::Color;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub ignore_case: bool,

    #[clap(short = 'v', long)]
    pub invert_match: bool,

    #[clap(short, long)]
    pub regex: bool,

    #[clap(short, long)]
    pub color: Option<Color>,

    pub needle: String,

    pub file: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parsing_default_values() {
        let args = Args::parse_from(&["program", "needle"]);
        assert_eq!(args.ignore_case, false);
        assert_eq!(args.invert_match, false);
        assert_eq!(args.regex, false);
        assert_eq!(args.color, None);
        assert_eq!(args.needle, "needle");
        assert_eq!(args.file, None);
    }

    #[test]
    fn test_cli_parsing_all_options() {
        let args = Args::parse_from(&[
            "program",
            "--ignore-case",
            "-v",
            "--regex",
            "--color",
            "red",
            "pattern",
            "file.txt",
        ]);
        assert_eq!(args.ignore_case, true);
        assert_eq!(args.invert_match, true);
        assert_eq!(args.regex, true);
        assert_eq!(args.color, Some(Color::Red));
        assert_eq!(args.needle, "pattern");
        assert_eq!(args.file, Some(PathBuf::from("file.txt")));
    }
}
