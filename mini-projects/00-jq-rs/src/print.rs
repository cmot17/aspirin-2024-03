use serde_json::Value;
use std::env;
use std::io::{self, Write};

const RESET: &str = "\x1b[0m";

struct Colors {
    null: String,
    false_: String,
    true_: String,
    number: String,
    string: String,
    array: String,
    object: String,
    object_key: String,
}

impl Colors {
    fn monochrome() -> Self {
        Colors {
            null: "".to_string(),
            false_: "".to_string(),
            true_: "".to_string(),
            number: "".to_string(),
            string: "".to_string(),
            array: "".to_string(),
            object: "".to_string(),
            object_key: "".to_string(),
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            null: "\x1b[0;90m".to_string(),
            false_: "\x1b[0;37m".to_string(),
            true_: "\x1b[0;37m".to_string(),
            number: "\x1b[0;37m".to_string(),
            string: "\x1b[0;32m".to_string(),
            array: "\x1b[1;37m".to_string(),
            object: "\x1b[1;37m".to_string(),
            object_key: "\x1b[1;34m".to_string(),
        }
    }
}

fn get_jq_colors() -> String {
    env::var("JQ_COLORS").unwrap_or_else(|_| "0;90:0;37:0;37:0;37:0;32:1;37:1;37:1;34".to_string())
}

fn parse_color_entry(entry: &str) -> String {
    let parts: Vec<&str> = entry.split(';').collect();
    match parts.len() {
        2 => format!("\x1b[{};{}m", parts[0], parts[1]),
        1 => format!("\x1b[{}m", parts[0]),
        _ => "\x1b[0m".to_string(), // Default to reset if invalid
    }
}

fn parse_jq_colors(jq_colors: &str) -> Colors {
    let entries: Vec<&str> = jq_colors.split(':').collect();
    let mut colors = Colors::default();

    if entries.len() == 8 {
        colors.null = parse_color_entry(entries[0]);
        colors.false_ = parse_color_entry(entries[1]);
        colors.true_ = parse_color_entry(entries[2]);
        colors.number = parse_color_entry(entries[3]);
        colors.string = parse_color_entry(entries[4]);
        colors.array = parse_color_entry(entries[5]);
        colors.object = parse_color_entry(entries[6]);
        colors.object_key = parse_color_entry(entries[7]);
    }
    colors
}

pub fn pretty_print_json(value: &Value, step: usize, sort_keys: bool, color: bool, compact: bool) {
    let mut stdout = io::stdout();
    pretty_print_json_to_writer(value, step, sort_keys, color, compact, &mut stdout)
        .expect("Failed to write to stdout");
}

pub fn pretty_print_json_to_writer(
    value: &Value,
    step: usize,
    sort_keys: bool,
    color: bool,
    compact: bool,
    writer: &mut dyn Write,
) -> io::Result<()> {
    let jq_colors = get_jq_colors();
    let colors = if color {
        Some(parse_jq_colors(&jq_colors))
    } else {
        None
    };

    pretty_print_json_inner(value, 0, step, colors.as_ref(), sort_keys, compact, writer)?;
    writeln!(writer)
}

fn pretty_print_json_inner(
    value: &Value,
    level: usize,
    step: usize,
    colors: Option<&Colors>,
    sort_keys: bool,
    compact: bool,
    writer: &mut dyn Write,
) -> io::Result<()> {
    match value {
        Value::Object(map) => {
            if compact {
                if let Some(c) = colors {
                    write!(writer, "{}{{{}", c.object, RESET)?;
                } else {
                    write!(writer, "{{")?;
                }
            } else if let Some(c) = colors {
                writeln!(writer, "{}{{{}", c.object, RESET)?;
            } else {
                writeln!(writer, "{{")?;
            }

            let keys: Vec<&String> = if sort_keys {
                let mut ks: Vec<&String> = map.keys().collect();
                ks.sort();
                ks
            } else {
                map.keys().collect()
            };

            let last_index = keys.len().saturating_sub(1);
            for (index, key) in keys.iter().enumerate() {
                let is_last_entry = index == last_index;
                if !compact {
                    if let Some(c) = colors {
                        write!(
                            writer,
                            "{:width$}{}\"{}\"{}: ",
                            "",
                            c.object_key,
                            key,
                            RESET,
                            width = (level + 1) * step
                        )?;
                    } else {
                        write!(
                            writer,
                            "{:width$}\"{}\": ",
                            "",
                            key,
                            width = (level + 1) * step
                        )?;
                    }
                } else if let Some(c) = colors {
                    write!(writer, "{}\"{}\"{}:", c.object_key, key, RESET)?;
                } else {
                    write!(writer, "\"{}\":", key)?;
                }
                pretty_print_json_inner(
                    &map[*key],
                    level + 1,
                    step,
                    colors,
                    sort_keys,
                    compact,
                    writer,
                )?;
                if !is_last_entry {
                    write!(writer, ",")?;
                    if !compact {
                        writeln!(writer)?;
                    }
                }
            }
            if !compact {
                if let Some(c) = colors {
                    write!(
                        writer,
                        "\n{:width$}{}}}{}",
                        "",
                        c.object,
                        RESET,
                        width = level * step
                    )?;
                } else {
                    write!(writer, "\n{:width$}}}", "", width = level * step)?;
                }
            } else if let Some(c) = colors {
                write!(writer, "{}}}{}", c.object, RESET)?;
            } else {
                write!(writer, "}}")?;
            }
        }
        Value::Array(arr) => {
            if compact {
                if let Some(c) = colors {
                    write!(writer, "{}[{}", c.array, RESET)?;
                } else {
                    write!(writer, "[")?;
                }
            } else if let Some(c) = colors {
                writeln!(writer, "{}[{}", c.array, RESET)?;
            } else {
                writeln!(writer, "[")?;
            }
            let last_index = arr.len().saturating_sub(1);
            for (index, val) in arr.iter().enumerate() {
                let is_last_entry = index == last_index;
                if !compact {
                    write!(writer, "{:width$}", "", width = (level + 1) * step)?;
                }
                pretty_print_json_inner(val, level + 1, step, colors, sort_keys, compact, writer)?;
                if !is_last_entry {
                    write!(writer, ",")?;
                    if !compact {
                        writeln!(writer)?;
                    }
                }
            }
            if !compact {
                if let Some(c) = colors {
                    write!(
                        writer,
                        "\n{:width$}{}]{}",
                        "",
                        c.array,
                        RESET,
                        width = level * step
                    )?;
                } else {
                    write!(writer, "\n{:width$}]", "", width = level * step)?;
                }
            } else if let Some(c) = colors {
                write!(writer, "{}]{}", c.array, RESET)?;
            } else {
                write!(writer, "]")?;
            }
        }
        Value::String(s) => {
            if let Some(c) = colors {
                write!(writer, "{}\"{}\"{}", c.string, s, RESET)?;
            } else {
                write!(writer, "\"{}\"", s)?;
            }
        }
        Value::Number(num) => {
            if let Some(c) = colors {
                write!(writer, "{}{}{}", c.number, num, RESET)?;
            } else {
                write!(writer, "{}", num)?;
            }
        }
        Value::Bool(b) => {
            let color = if *b {
                colors.map(|c| &c.true_)
            } else {
                colors.map(|c| &c.false_)
            };
            if let Some(c) = color {
                write!(writer, "{}{}{}", c, b, RESET)?;
            } else {
                write!(writer, "{}", b)?;
            }
        }
        Value::Null => {
            if let Some(c) = colors {
                write!(writer, "{}null{}", c.null, RESET)?;
            } else {
                write!(writer, "null")?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_colored_printing() {
        let sample_data = json!(null);
        let indent = 2;
        let sort_keys = false;
        let use_color = true;
        let compact_output = false;

        // Capture the output
        let mut output = Vec::new();
        pretty_print_json_to_writer(
            &sample_data,
            indent,
            sort_keys,
            use_color,
            compact_output,
            &mut output,
        )
        .expect("Failed to write to writer");

        let output_str = String::from_utf8(output).expect("Invalid UTF-8 output");
        // Based on Colors::default(), null should be gray: \x1b[0;90mnull\x1b[0m
        assert!(output_str.contains("\x1b[0;90mnull\x1b[0m"));
    }

    #[test]
    fn test_monochrome_printing() {
        let sample_data = json!(null);
        let indent = 2;
        let sort_keys = false;
        let use_color = false;
        let compact_output = false;

        // Capture the output
        let mut output = Vec::new();
        pretty_print_json_to_writer(
            &sample_data,
            indent,
            sort_keys,
            use_color,
            compact_output,
            &mut output,
        )
        .expect("Failed to write to writer");

        let output_str = String::from_utf8(output).expect("Invalid UTF-8 output");
        assert_eq!(output_str.trim(), "null");
    }

    #[test]
    fn test_sorted_keys() {
        let sample_data = json!({
            "fuzz": true,
            "baz": null,
            "biz": 42,
            "bizz": 22.0,
            "fizz": "buzz",
            "fizzes": ["buzz", null, true, 22.0, 42.0]
        });

        let indent = 2;
        let sort_keys = true;
        let use_color = false;
        let compact_output = false;

        // Capture the output
        let mut output = Vec::new();
        pretty_print_json_to_writer(
            &sample_data,
            indent,
            sort_keys,
            use_color,
            compact_output,
            &mut output,
        )
        .expect("Failed to write to writer");

        let output_str = String::from_utf8(output).expect("Invalid UTF-8 output");
        let expected_output = r#"{
  "baz": null,
  "biz": 42,
  "bizz": 22.0,
  "fizz": "buzz",
  "fizzes": [
    "buzz",
    null,
    true,
    22.0,
    42.0
  ],
  "fuzz": true
}"#;
        assert_eq!(output_str.trim(), expected_output);
    }

    #[test]
    fn test_indent_option() {
        let sample_data = json!({
            "baz": null,
            "biz": 42,
            "bizz": 22.0,
            "fizz": "buzz",
            "fizzes": ["buzz", null, true, 22.0, 42.0],
            "fuzz": true
        });

        let indent = 7;
        let sort_keys = false;
        let use_color = false;
        let compact_output = false;

        // Capture the output
        let mut output = Vec::new();
        pretty_print_json_to_writer(
            &sample_data,
            indent,
            sort_keys,
            use_color,
            compact_output,
            &mut output,
        )
        .expect("Failed to write to writer");

        let output_str = String::from_utf8(output).expect("Invalid UTF-8 output");
        let expected_output = r#"{
       "baz": null,
       "biz": 42,
       "bizz": 22.0,
       "fizz": "buzz",
       "fizzes": [
              "buzz",
              null,
              true,
              22.0,
              42.0
       ],
       "fuzz": true
}"#;
        assert_eq!(output_str.trim(), expected_output);
    }

    #[test]
    fn test_compact_output() {
        let sample_data = json!({
            "fizz": "buzz",
            "baz": null,
            "fuzz": true,
            "bizz": 22.0,
            "biz": 42,
            "fizzes": ["buzz", null, true, 22.0, 42.0]
        });

        let indent = 0; // Ignored when compact_output is true
        let sort_keys = false;
        let use_color = false;
        let compact_output = true;

        // Capture the output
        let mut output = Vec::new();
        pretty_print_json_to_writer(
            &sample_data,
            indent,
            sort_keys,
            use_color,
            compact_output,
            &mut output,
        )
        .expect("Failed to write to writer");

        let output_str = String::from_utf8(output).expect("Invalid UTF-8 output");
        let expected_output = r#"{"fizz":"buzz","baz":null,"fuzz":true,"bizz":22.0,"biz":42,"fizzes":["buzz",null,true,22.0,42.0]}"#;
        assert_eq!(output_str.trim(), expected_output);
    }

    #[test]
    fn test_pretty_print_unsorted() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 30,
            "isDeveloper": true,
            "languages": ["Rust", "JavaScript", "Python"],
            "address": {
                "city": "Somewhere",
                "zip": "12345"
            }
        }
        "#;

        let json_value: Value = serde_json::from_str(data).expect("Failed to parse JSON");

        // Capture the output
        let mut output = Vec::new();
        pretty_print_json_to_writer(
            &json_value,
            4,
            false,
            false, // monochrome
            false, // not compact
            &mut output,
        )
        .expect("Failed to write to writer");

        let output_str = String::from_utf8(output).expect("Invalid UTF-8 output");
        let expected_output = r#"{
    "name": "John Doe",
    "age": 30,
    "isDeveloper": true,
    "languages": [
        "Rust",
        "JavaScript",
        "Python"
    ],
    "address": {
        "city": "Somewhere",
        "zip": "12345"
    }
}"#;
        assert_eq!(output_str.trim(), expected_output);
    }

    #[test]
    fn test_pretty_print_sorted() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 30,
            "isDeveloper": true,
            "languages": ["Rust", "JavaScript", "Python"],
            "address": {
                "city": "Somewhere",
                "zip": "12345"
            }
        }
        "#;

        let json_value: Value = serde_json::from_str(data).expect("Failed to parse JSON");

        // Capture the output
        let mut output = Vec::new();
        pretty_print_json_to_writer(
            &json_value,
            4,
            true,  // sort_keys
            false, // monochrome
            false, // not compact
            &mut output,
        )
        .expect("Failed to write to writer");

        let output_str = String::from_utf8(output).expect("Invalid UTF-8 output");
        let expected_output = r#"{
    "address": {
        "city": "Somewhere",
        "zip": "12345"
    },
    "age": 30,
    "isDeveloper": true,
    "languages": [
        "Rust",
        "JavaScript",
        "Python"
    ],
    "name": "John Doe"
}"#;
        assert_eq!(output_str.trim(), expected_output);
    }

    #[test]
    fn test_pretty_print_with_nulls() {
        let data = r#"
        {
            "name": null,
            "age": 30,
            "isDeveloper": null,
            "languages": ["Rust", null, "Python"]
        }
        "#;

        let json_value: Value = serde_json::from_str(data).expect("Failed to parse JSON");

        // Capture the pretty output
        let mut pretty_output = Vec::new();
        pretty_print_json_to_writer(
            &json_value,
            4,
            false,
            true,  // colored
            false, // not compact
            &mut pretty_output,
        )
        .expect("Failed to write to writer");

        let pretty_str = String::from_utf8(pretty_output).expect("Invalid UTF-8 output");
        // Check for colored nulls (\x1b[0;90mnull\x1b[0m)
        assert!(pretty_str.contains("\x1b[0;90mnull\x1b[0m"));

        // Capture the compact output
        let mut compact_output = Vec::new();
        pretty_print_json_to_writer(
            &json_value,
            4,
            false,
            false, // monochrome
            true,  // compact
            &mut compact_output,
        )
        .expect("Failed to write to writer");

        let compact_str = String::from_utf8(compact_output).expect("Invalid UTF-8 output");
        let expected_compact =
            r#"{"name":null,"age":30,"isDeveloper":null,"languages":["Rust",null,"Python"]}"#;
        assert_eq!(compact_str.trim(), expected_compact);
    }

    #[test]
    fn test_pretty_print_with_arrays() {
        let data = r#"
        {
            "languages": ["Rust", "JavaScript", "Python", "Go"],
            "projects": ["Open Source", "Private Projects"]
        }
        "#;

        let json_value: Value = serde_json::from_str(data).expect("Failed to parse JSON");

        // Capture the pretty output
        let mut pretty_output = Vec::new();
        pretty_print_json_to_writer(
            &json_value,
            4,
            false,
            false, // monochrome
            false, // not compact
            &mut pretty_output,
        )
        .expect("Failed to write to writer");

        let pretty_str = String::from_utf8(pretty_output).expect("Invalid UTF-8 output");
        let expected_pretty = r#"{
    "languages": [
        "Rust",
        "JavaScript",
        "Python",
        "Go"
    ],
    "projects": [
        "Open Source",
        "Private Projects"
    ]
}"#;
        assert_eq!(pretty_str.trim(), expected_pretty);

        // Capture the compact output
        let mut compact_output = Vec::new();
        pretty_print_json_to_writer(
            &json_value,
            4,
            false,
            false, // monochrome
            true,  // compact
            &mut compact_output,
        )
        .expect("Failed to write to writer");

        let compact_str = String::from_utf8(compact_output).expect("Invalid UTF-8 output");
        let expected_compact = r#"{"languages":["Rust","JavaScript","Python","Go"],"projects":["Open Source","Private Projects"]}"#;
        assert_eq!(compact_str.trim(), expected_compact);
    }

    #[test]
    fn test_pretty_print_with_nested_objects() {
        let data = r#"
        {
            "name": "Jane Doe",
            "age": 25,
            "isDeveloper": true,
            "languages": ["Rust", "Python"],
            "address": {
                "city": "Anywhere",
                "state": "CA",
                "coordinates": {
                    "latitude": 37.7749,
                    "longitude": -122.4194
                }
            }
        }
        "#;

        let json_value: Value = serde_json::from_str(data).expect("Failed to parse JSON");

        // Capture the pretty output
        let mut pretty_output = Vec::new();
        pretty_print_json_to_writer(
            &json_value,
            4,
            false,
            false, // monochrome
            false, // not compact
            &mut pretty_output,
        )
        .expect("Failed to write to writer");

        let pretty_str = String::from_utf8(pretty_output).expect("Invalid UTF-8 output");
        let expected_pretty = r#"{
    "name": "Jane Doe",
    "age": 25,
    "isDeveloper": true,
    "languages": [
        "Rust",
        "Python"
    ],
    "address": {
        "city": "Anywhere",
        "state": "CA",
        "coordinates": {
            "latitude": 37.7749,
            "longitude": -122.4194
        }
    }
}"#;
        assert_eq!(pretty_str.trim(), expected_pretty);

        // Capture the compact output
        let mut compact_output = Vec::new();
        pretty_print_json_to_writer(
            &json_value,
            4,
            false,
            false, // monochrome
            true,  // compact
            &mut compact_output,
        )
        .expect("Failed to write to writer");

        let compact_str = String::from_utf8(compact_output).expect("Invalid UTF-8 output");
        let expected_compact = r#"{"name":"Jane Doe","age":25,"isDeveloper":true,"languages":["Rust","Python"],"address":{"city":"Anywhere","state":"CA","coordinates":{"latitude":37.7749,"longitude":-122.4194}}}"#;
        assert_eq!(compact_str.trim(), expected_compact);
    }

    #[test]
    fn test_pretty_print_with_boolean_values() {
        let data = r#"
        {
            "isActive": true,
            "isAdmin": false,
            "featuresEnabled": [true, false, true]
        }
        "#;

        let json_value: Value = serde_json::from_str(data).expect("Failed to parse JSON");

        // Capture the pretty output with colors
        let mut pretty_output = Vec::new();
        pretty_print_json_to_writer(
            &json_value,
            4,
            false,
            true,  // colored
            false, // not compact
            &mut pretty_output,
        )
        .expect("Failed to write to writer");

        let pretty_str = String::from_utf8(pretty_output).expect("Invalid UTF-8 output");
        // Based on Colors::default(), true is white: \x1b[0;37mtrue\x1b[0m
        // and false is white: \x1b[0;37mfalse\x1b[0m
        assert!(pretty_str.contains("\x1b[0;37mtrue\x1b[0m"));
        assert!(pretty_str.contains("\x1b[0;37mfalse\x1b[0m"));

        // Capture the compact output without colors
        let mut compact_output = Vec::new();
        pretty_print_json_to_writer(
            &json_value,
            4,
            false,
            false, // monochrome
            true,  // compact
            &mut compact_output,
        )
        .expect("Failed to write to writer");

        let compact_str = String::from_utf8(compact_output).expect("Invalid UTF-8 output");
        let expected_compact =
            r#"{"isActive":true,"isAdmin":false,"featuresEnabled":[true,false,true]}"#;
        assert_eq!(compact_str.trim(), expected_compact);
    }
}
