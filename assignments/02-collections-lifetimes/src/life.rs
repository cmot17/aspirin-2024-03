fn split_string<'a>(string: &'a str, delimeter: &str) -> Vec<&'a str> {
    let mut split: Vec<&str> = Vec::new();
    let mut last_pos = 0;

    if string.is_empty() {
        return vec![];
    }

    for (start_pos, _) in string.match_indices(delimeter) {
        split.push(&string[last_pos..start_pos]);
        last_pos = start_pos + delimeter.len();
    }

    if last_pos < string.len() {
        split.push(&string[last_pos..]);
    }
    split
}

#[derive(PartialEq, Debug)]
struct Differences<'a> {
    only_in_first: Vec<&'a str>,
    only_in_second: Vec<&'a str>,
}

// Function to find differences between two strings
fn find_differences<'a>(first_string: &'a str, second_string: &'a str) -> Differences<'a> {
    // Split the first string into words
    let first_words = split_string(first_string, " ");
    // Split the second string into words
    let second_words = split_string(second_string, " ");

    // Create and return a Differences struct
    Differences {
        // Find words that are only in the first string
        only_in_first: first_words
            .iter()
            .filter(|&word| !second_string.contains(word))
            .cloned()
            .collect(),
        // Find words that are only in the second string
        only_in_second: second_words
            .iter()
            .filter(|&word| !first_string.contains(word))
            .cloned()
            .collect(),
    }
}
fn merge_names(first_name: &str, second_name: &str) -> String {
    let mut merged_name = String::new();
    let mut current_name = 0;
    let mut first_char = true;
    let mut first_name_chars = first_name.chars().peekable();
    let mut second_name_chars = second_name.chars().peekable();

    println!("Merging names: '{}' and '{}'", first_name, second_name);

    loop {
        if current_name == 0 {
            println!("Processing first name");
            match first_name_chars.peek() {
                Some(&c) if first_char || !"AEIOUaeiou".contains(c) => {
                    println!("Adding character '{}' from first name", c);
                    merged_name.push(c);
                    first_name_chars.next(); // Advance iterator only when character is used
                    first_char = false;
                }
                Some(&c) => {
                    println!("Skipping vowel '{}' from first name", c);
                    current_name = 1; // Switch to second name if vowel encountered
                    first_char = true;
                }
                None => {
                    println!("Finished processing first name");
                    current_name = 1;
                    first_char = true;
                }
            }
        } else {
            println!("Processing second name");
            match second_name_chars.peek() {
                Some(&c) if first_char || !"AEIOUaeiou".contains(c) => {
                    println!("Adding character '{}' from second name", c);
                    merged_name.push(c);
                    second_name_chars.next(); // Advance iterator only when character is used
                    first_char = false;
                }
                Some(&c) => {
                    println!("Skipping vowel '{}' from second name", c);
                    current_name = 0; // Switch to first name if vowel encountered
                    first_char = true;
                }
                None => {
                    println!("Finished processing second name");
                    current_name = 0;
                    first_char = true;
                }
            }
        }
        if first_name_chars.peek().is_none() && second_name_chars.peek().is_none() {
            println!("Both names fully processed");
            break;
        }
    }
    println!("Final merged name: '{}'", merged_name);
    merged_name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        // First, make sure the lifetimes were correctly marked
        let matches;
        let string_to_split = String::from("Hello, World!");

        {
            let delimeter = String::from(", ");
            matches = split_string(&string_to_split, &delimeter);
        }
        println!("Matches can be printed! See: {:?}", matches);

        // Now check the split logic
        assert_eq!(split_string("", ""), Vec::<&str>::new());
        assert_eq!(split_string("Hello, World!", ", "), vec!["Hello", "World!"]);
        assert_eq!(
            split_string(
                "I this think this that this sentence this is this very this confusing this ",
                " this "
            ),
            vec!["I", "think", "that", "sentence", "is", "very", "confusing"]
        );
        assert_eq!(
            split_string("apple🍎banana🍎orange", "🍎"),
            vec!["apple", "banana", "orange"]
        );
        assert_eq!(
            split_string("Ayush;put|a,lot~of`random;delimeters|in|this,sentence", ";"),
            vec![
                "Ayush",
                "put|a,lot~of`random",
                "delimeters|in|this,sentence"
            ]
        );
    }

    #[test]
    fn test_find_differences() {
        assert_eq!(
            find_differences("", ""),
            Differences {
                only_in_first: Vec::new(),
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences("pineapple pen", "apple"),
            Differences {
                only_in_first: vec!["pineapple", "pen"],
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(
                "Sally sold seashells at the seashore",
                "Seashells seashells at the seashore"
            ),
            Differences {
                only_in_first: vec!["Sally", "sold"],
                only_in_second: vec!["Seashells"]
            }
        );
        assert_eq!(
            find_differences(
                "How much wood could a wood chuck chuck",
                "If a wood chuck could chuck wood"
            ),
            Differences {
                only_in_first: vec!["How", "much"],
                only_in_second: vec!["If"]
            }
        );
        assert_eq!(
            find_differences(
                "How much ground would a groundhog hog",
                "If a groundhog could hog ground"
            ),
            Differences {
                only_in_first: vec!["How", "much", "would"],
                only_in_second: vec!["If", "could"]
            }
        );
    }

    #[test]
    fn test_merge_names() {
        assert_eq!(merge_names("alex", "jake"), "aljexake");
        assert_eq!(merge_names("steven", "stephen"), "ststevephenen");
        assert_eq!(merge_names("gym", "rhythm"), "gymrhythm");
        assert_eq!(merge_names("walter", "gibraltor"), "wgaltibreraltor");
        assert_eq!(merge_names("baker", "quaker"), "bqakueraker");
        assert_eq!(merge_names("", ""), "");
        assert_eq!(merge_names("samesies", "samesies"), "ssamamesesiieses");
        assert_eq!(merge_names("heather", "meagan"), "hmeeathageran");
        assert_eq!(merge_names("panda", "turtle"), "ptandurtlae");
        assert_eq!(merge_names("hot", "sauce"), "hsotauce");
        assert_eq!(merge_names("", "second"), "second");
        assert_eq!(merge_names("first", ""), "first");
    }
}
