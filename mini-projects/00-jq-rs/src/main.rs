#![allow(dead_code)]

mod filter;
mod parse;
mod print;
mod tokenizer;

use clap::Parser;
use std::io::BufReader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    filter_string: String,

    #[arg(
        short = 'C',
        long,
        default_value_t = true,
        conflicts_with = "monochrome_output"
    )]
    color_output: bool,

    #[arg(
        short = 'M',
        long,
        default_value_t = false,
        conflicts_with = "color_output"
    )]
    monochrome_output: bool,

    #[arg(short = 'S', long, default_value_t = false)]
    sort_keys: bool,

    #[arg(long, default_value_t = 2, value_parser = clap::value_parser!(u8).range(0..=7), conflicts_with = "compact_output")]
    indent: u8,

    #[arg(short, long, default_value_t = false, conflicts_with = "indent")]
    compact_output: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Determine if color should be used
    let use_color = if args.monochrome_output {
        false
    } else {
        args.color_output
    };

    let input = BufReader::new(std::io::stdin());

    //let lines = input.lines().collect::<Result<Vec<String>, _>>()?;
    //println!("{:?}", lines);
    let json_obj: serde_json::Value = serde_json::from_reader(input).unwrap();

    let tokens = tokenizer::tokenize(&args.filter_string).expect("TODO: panic message");

    let operations = parse::parse_filter(tokens)?;

    let filtered_json = filter::filter(json_obj, operations)?;

    for value in filtered_json {
        print::pretty_print_json(
            &value,
            args.indent.into(),
            args.sort_keys,
            use_color,
            args.compact_output,
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    // Separate module for filter tests
    use crate::filter;
    use crate::parse;
    use crate::tokenizer;

    #[test]
    fn test_identity_filter() {
        // Prepare the sample data
        let sample_data = json!({
            "fizz": "buzz",
            "baz": null,
            "fuzz": true,
            "bizz": 22.0,
            "biz": 42,
            "fizzes": [
                "buzz",
                null,
                true,
                22.0,
                42.0
            ]
        });

        // Prepare the filter string
        let filter_string = ".";

        // Tokenize the filter
        let tokens = tokenizer::tokenize(filter_string).expect("Tokenization failed");

        // Parse the filter
        let operations = parse::parse_filter(tokens).expect("Parsing failed");

        // Apply the filter
        let filtered_json =
            filter::filter(sample_data.clone(), operations).expect("Filtering failed");

        // Verify the output
        assert_eq!(filtered_json.len(), 1);
        assert_eq!(&filtered_json[0], &sample_data);
    }

    #[test]
    fn test_object_identifier_index() {
        let sample_data = json!({
            "fizz": "buzz",
            "baz": null,
            "fuzz": true,
            "bizz": 22.0,
            "biz": 42,
            "fizzes": [
                "buzz",
                null,
                true,
                22.0,
                42.0
            ]
        });

        let filter_string = ".fizz";
        let tokens = tokenizer::tokenize(filter_string).expect("Tokenization failed");
        let operations = parse::parse_filter(tokens).expect("Parsing failed");
        let filtered_json = filter::filter(sample_data, operations).expect("Filtering failed");

        assert_eq!(filtered_json.len(), 1);
        assert_eq!(filtered_json[0], json!("buzz"));
    }

    #[test]
    fn test_array_index() {
        let sample_data = json!(["one", "two", "three"]);
        let filter_string = ".[0]";
        let tokens = tokenizer::tokenize(filter_string).expect("Tokenization failed");
        let operations = parse::parse_filter(tokens).expect("Parsing failed");
        let filtered_json = filter::filter(sample_data, operations).expect("Filtering failed");

        assert_eq!(filtered_json.len(), 1);
        assert_eq!(filtered_json[0], json!("one"));
    }

    #[test]
    fn test_array_slice() {
        let sample_data = json!(["one", "two", "three"]);
        let filter_string = ".[0:2]";
        let tokens = tokenizer::tokenize(filter_string).expect("Tokenization failed");
        let operations = parse::parse_filter(tokens).expect("Parsing failed");
        let filtered_json = filter::filter(sample_data, operations).expect("Filtering failed");

        assert_eq!(filtered_json.len(), 1);
        assert_eq!(filtered_json[0], json!(["one", "two"]));
    }

    #[test]
    fn test_pipe_operator() {
        let sample_data = json!({
            "fizz": "buzz",
            "baz": null,
            "fuzz": true,
            "bizz": 22.0,
            "biz": 42,
            "fizzes": [
                "buzz",
                null,
                true,
                22.0,
                42.0
            ]
        });

        let filter_string = ".fizzes | .[1]";
        let tokens = tokenizer::tokenize(filter_string).expect("Tokenization failed");
        let operations = parse::parse_filter(tokens).expect("Parsing failed");
        let filtered_json = filter::filter(sample_data, operations).expect("Filtering failed");

        assert_eq!(filtered_json.len(), 1);
        assert_eq!(filtered_json[0], json!(null));
    }

    #[test]
    fn test_array_iterator() {
        let sample_data = json!([
            {"name": "Leo Lightning"},
            {"name": "Maximus Defender"},
            {"name": "Sophie Swift"}
        ]);

        let filter_string = ".[] | .name";
        let tokens = tokenizer::tokenize(filter_string).expect("Tokenization failed");
        let operations = parse::parse_filter(tokens).expect("Parsing failed");
        let filtered_json = filter::filter(sample_data, operations).expect("Filtering failed");

        let expected = vec![
            json!("Leo Lightning"),
            json!("Maximus Defender"),
            json!("Sophie Swift"),
        ];

        assert_eq!(filtered_json, expected);
    }

    #[test]
    fn test_add_function() {
        let sample_data = json!(["one", "two", "three"]);
        let filter_string = ". | add";
        let tokens = tokenizer::tokenize(filter_string).expect("Tokenization failed");
        let operations = parse::parse_filter(tokens).expect("Parsing failed");
        let filtered_json = filter::filter(sample_data, operations).expect("Filtering failed");

        assert_eq!(filtered_json.len(), 1);
        assert_eq!(filtered_json[0], json!("onetwothree"));
    }

    #[test]
    fn test_length_function() {
        let sample_data = json!(["one", "two", "three"]);
        let filter_string = ". | length";
        let tokens = tokenizer::tokenize(filter_string).expect("Tokenization failed");
        let operations = parse::parse_filter(tokens).expect("Parsing failed");
        let filtered_json = filter::filter(sample_data, operations).expect("Filtering failed");

        assert_eq!(filtered_json.len(), 1);
        assert_eq!(filtered_json[0], json!(3));
    }

    #[test]
    fn test_del_function() {
        let sample_data = json!({
            "fizz": "buzz",
            "baz": null,
            "fuzz": true,
            "bizz": 22.0,
            "biz": 42,
            "fizzes": [
                "buzz",
                null,
                true,
                22.0,
                42.0
            ]
        });

        let filter_string = ". | del(.fizzes)";
        let tokens = tokenizer::tokenize(filter_string).expect("Tokenization failed");
        let operations = parse::parse_filter(tokens).expect("Parsing failed");
        let filtered_json = filter::filter(sample_data, operations).expect("Filtering failed");

        let expected_output = json!({
            "fizz": "buzz",
            "baz": null,
            "fuzz": true,
            "bizz": 22.0,
            "biz": 42
        });

        assert_eq!(filtered_json.len(), 1);
        assert_eq!(filtered_json[0], expected_output);
    }
}
