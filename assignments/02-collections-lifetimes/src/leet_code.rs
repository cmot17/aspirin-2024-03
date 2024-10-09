fn longest_equal_sequence_prescriptive<T: std::cmp::PartialEq>(sequence: &[T]) -> i32 {
    let mut max_length = 0;
    let mut current_length = 1;
    if sequence.is_empty() {
        return 0;
    }
    for i in 1..sequence.len() {
        if sequence[i] == sequence[i - 1] {
            current_length += 1;
            if current_length > max_length {
                max_length = current_length;
            }
        } else {
            current_length = 1;
        }
    }
    if current_length > max_length {
        max_length = current_length;
    }
    max_length
}

fn longest_equal_sequence_functional<T: std::cmp::PartialEq>(sequence: &[T]) -> i32 {
    sequence
        .iter()
        .fold(
            (0, 1, None),
            |(max_length, current_length, previous_element): (i32, i32, Option<&T>), element| {
                match previous_element {
                    Some(prev) if prev == element => {
                        let new_length = current_length + 1;
                        (max_length.max(new_length), new_length, Some(element))
                    }
                    _ => (max_length.max(current_length), 1, Some(element)),
                }
            },
        )
        .0
}

fn is_valid_paranthesis(paranthesis: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in paranthesis.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if stack.pop().unwrap_or_default()
                    != match c {
                        ')' => '(',
                        ']' => '[',
                        '}' => '{',
                        _ => return false,
                    }
                {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

fn longest_common_substring(first_str: &str, second_str: &str) -> String {
    let first_chars: Vec<char> = first_str.chars().collect();
    let second_chars: Vec<char> = second_str.chars().collect();
    let mut max_length = 0;
    let mut end_index = 0;

    for i in 0..first_chars.len() {
        for j in 0..second_chars.len() {
            let mut k = 0;
            while i + k < first_chars.len()
                && j + k < second_chars.len()
                && first_chars[i + k] == second_chars[j + k]
            {
                k += 1;
            }
            if k > max_length {
                max_length = k;
                end_index = i + k;
            }
        }
    }

    first_str[end_index - max_length..end_index].to_string()
}

fn longest_common_substring_multiple(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut result = strings[0].to_string();

    for string in strings.iter().skip(1) {
        result = longest_common_substring(&result, string);
        if result.is_empty() {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_equal_sequence_prescriptive() {
        assert_eq!(longest_equal_sequence_prescriptive(&[1, 1, 1, 1, 1]), 5);
        assert_eq!(
            longest_equal_sequence_prescriptive(&[1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0]),
            3
        );
        assert_eq!(longest_equal_sequence_prescriptive(&[-100]), 1);
        let empty_vec: Vec<char> = Vec::new();
        assert_eq!(longest_equal_sequence_prescriptive(&empty_vec), 0);
        assert_eq!(
            longest_equal_sequence_prescriptive(&[
                1000, 1000, 2000, 2000, 2000, 3000, 3000, 3000, 3000
            ]),
            4
        );
        assert_eq!(
            longest_equal_sequence_prescriptive(&['a', 'b', 'a', 'b', 'a', 'b']),
            1
        );
        let vec: Vec<u8> = vec![5, 5, 5, 1, 2, 3];
        assert_eq!(longest_equal_sequence_prescriptive(&vec), 3);
        assert_eq!(longest_equal_sequence_prescriptive(&[1, 2, 3, 4, 4, 4]), 3);
        assert_eq!(longest_equal_sequence_prescriptive(&[1, 2, 3, 4, 5]), 1);
        assert_eq!(
            longest_equal_sequence_prescriptive(&[1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1]),
            5
        );
    }
    #[test]
    fn test_longest_equal_sequence_functional() {
        assert_eq!(longest_equal_sequence_functional(&[1, 1, 1, 1, 1]), 5);
        assert_eq!(
            longest_equal_sequence_functional(&[1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0]),
            3
        );
        assert_eq!(longest_equal_sequence_functional(&[-100]), 1);
        let empty_vec: Vec<char> = Vec::new();
        assert_eq!(longest_equal_sequence_functional(&empty_vec), 0);
        assert_eq!(
            longest_equal_sequence_functional(&[
                1000, 1000, 2000, 2000, 2000, 3000, 3000, 3000, 3000
            ]),
            4
        );
        assert_eq!(
            longest_equal_sequence_functional(&['a', 'b', 'a', 'b', 'a', 'b']),
            1
        );
        let vec: Vec<u8> = vec![5, 5, 5, 1, 2, 3];
        assert_eq!(longest_equal_sequence_functional(&vec), 3);
        assert_eq!(longest_equal_sequence_functional(&[1, 2, 3, 4, 4, 4]), 3);
        assert_eq!(longest_equal_sequence_functional(&[1, 2, 3, 4, 5]), 1);
        assert_eq!(
            longest_equal_sequence_functional(&[1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1]),
            5
        );
    }

    #[test]
    fn test_is_valid_paranthesis() {
        assert!(is_valid_paranthesis(&String::from("{}")));
        assert!(is_valid_paranthesis(&String::from("()")));
        assert!(is_valid_paranthesis(&String::from("()[]{}")));
        assert!(is_valid_paranthesis(&String::from("({[]})")));
        assert!(is_valid_paranthesis(&String::from("([]){}{}([]){}")));
        assert!(!is_valid_paranthesis(&String::from("()(")));
        assert!(!is_valid_paranthesis(&String::from("(()")));
        assert!(!is_valid_paranthesis(&String::from("([)]{[})")));
        assert!(!is_valid_paranthesis(&String::from("({[()]}){[([)]}")));
        assert!(!is_valid_paranthesis(&String::from("()[]{}(([])){[()]}(")));
    }

    #[test]
    fn test_common_substring() {
        assert_eq!(longest_common_substring("abcdefg", "bcdef"), "bcdef");
        assert_eq!(longest_common_substring("apple", "pineapple"), "apple");
        assert_eq!(longest_common_substring("dog", "cat"), "");
        assert_eq!(longest_common_substring("racecar", "racecar"), "racecar");
        assert_eq!(longest_common_substring("ababc", "babca"), "babc");
        assert_eq!(longest_common_substring("xyzabcxyz", "abc"), "abc");
        assert_eq!(longest_common_substring("", "abc"), "");
        assert_eq!(longest_common_substring("abcdefgh", "defghijk"), "defgh");
        assert_eq!(longest_common_substring("xyabcz", "abcxy"), "abc");
        assert_eq!(longest_common_substring("ABCDEFG", "abcdefg"), "");
        assert_eq!(
            longest_common_substring(
                "thisisaverylongstringwithacommonsubstring",
                "anotherlongstringwithacommonsubstring"
            ),
            "longstringwithacommonsubstring"
        );
        assert_eq!(longest_common_substring("a", "a"), "a");
    }

    #[test]
    fn test_common_substring_multiple() {
        assert_eq!(
            longest_common_substring_multiple(&["abcdefg", "cdef"]),
            "cdef"
        );
        assert_eq!(
            longest_common_substring_multiple(&["apple", "pineapple", "maple", "snapple"]),
            "ple"
        );
        assert_eq!(
            longest_common_substring_multiple(&["dog", "cat", "fish"]),
            ""
        );
        assert_eq!(
            longest_common_substring_multiple(&["racecar", "car", "scar"]),
            "car"
        );
        assert_eq!(
            longest_common_substring_multiple(&["ababc", "babca", "abcab"]),
            "abc"
        );
        assert_eq!(
            longest_common_substring_multiple(&["xyzabcxyz", "abc", "zabcy", "abc"]),
            "abc"
        );
        assert_eq!(longest_common_substring_multiple(&["", "abc", "def"]), "");
        assert_eq!(
            longest_common_substring_multiple(&["abcdefgh", "bcd", "bcdtravels", "abcs", "webcam"]),
            "bc"
        );
        assert_eq!(
            longest_common_substring_multiple(&["identical", "identical", "identical"]),
            "identical"
        );
        assert_eq!(
            longest_common_substring_multiple(&["xyabcz", "abcxy", "zabc"]),
            "abc"
        );
        assert_eq!(longest_common_substring_multiple(&["a", "a", "a"]), "a");
        assert_eq!(
            longest_common_substring_multiple(&[
                "thisisaverylongstringwiththecommonsubstring",
                "anotherlongstringwithacommonsubstring",
                "yetanotherstringthatcontainsacommonsubstring"
            ]),
            "commonsubstring",
        );
    }
}
