use regex::Regex;

pub trait Search {
    fn matches(&self, text: &str) -> Vec<Match>;
}

pub struct LiteralSearch {
    search_term: String,
}

impl LiteralSearch {
    pub fn new(search_term: &str) -> Self {
        Self {
            search_term: search_term.to_string(),
        }
    }
}

impl Search for LiteralSearch {
    fn matches(&self, text: &str) -> Vec<Match> {
        let mut matches = Vec::new();
        for line in text.lines() {
            let mut start_idx = line.find(&self.search_term);
            while let Some(start) = start_idx {
                let end = start + self.search_term.len();
                matches.push(Match {
                    line: line.to_string(),
                    start,
                    end,
                });
                start_idx = line[start + 1..]
                    .find(&self.search_term)
                    .map(|idx| start + 1 + idx);
            }
        }
        matches
    }
}

pub struct RegexSearch {
    regex: Regex,
}

impl RegexSearch {
    pub fn new(pattern: &str) -> Self {
        Self {
            regex: Regex::new(pattern).expect("Invalid regex pattern"),
        }
    }
}

impl Search for RegexSearch {
    fn matches(&self, text: &str) -> Vec<Match> {
        let mut matches = Vec::new();
        for line in text.lines() {
            for m in self.regex.find_iter(line) {
                matches.push(Match {
                    line: line.to_string(),
                    start: m.start(),
                    end: m.end(),
                });
            }
        }
        matches
    }
}

#[allow(dead_code)]
pub struct Match {
    pub line: String,
    pub start: usize,
    pub end: usize,
}

pub fn perform_search<S: Search>(searcher: S, lines: Vec<String>) -> Vec<Vec<Match>> {
    lines
        .iter()
        .map(|line| searcher.matches(line).into_iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_search_single_match() {
        let searcher = LiteralSearch::new("hello");
        let text = "hello world";
        let matches = searcher.matches(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line, "hello world");
        assert_eq!(matches[0].start, 0);
        assert_eq!(matches[0].end, 5);
    }

    #[test]
    fn test_literal_search_multiple_matches() {
        let searcher = LiteralSearch::new("a");
        let text = "a banana";
        let matches = searcher.matches(text);
        assert_eq!(matches.len(), 4);
        assert_eq!(matches[0].start, 0);
        assert_eq!(matches[1].start, 3);
        assert_eq!(matches[2].start, 5);
        assert_eq!(matches[3].start, 7);
    }

    #[test]
    fn test_literal_search_no_match() {
        let searcher = LiteralSearch::new("apple");
        let text = "banana";
        let matches = searcher.matches(text);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_literal_search_case_sensitive() {
        let searcher = LiteralSearch::new("Hello");
        let text = "hello world";
        let matches = searcher.matches(text);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_regex_search_single_match() {
        let searcher = RegexSearch::new(r"\d+");
        let text = "abc123def";
        let matches = searcher.matches(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line, "abc123def");
        assert_eq!(matches[0].start, 3);
        assert_eq!(matches[0].end, 6);
    }

    #[test]
    fn test_regex_search_multiple_matches() {
        let searcher = RegexSearch::new(r"\b\w{3}\b");
        let text = "The cat and dog ran";
        let matches = searcher.matches(text);
        assert_eq!(matches.len(), 5);
        assert_eq!(matches[0].line, "The cat and dog ran");
        assert_eq!(matches[0].start, 0);
        assert_eq!(matches[0].end, 3);
        assert_eq!(matches[1].start, 4);
        assert_eq!(matches[1].end, 7);
        assert_eq!(matches[2].start, 8);
        assert_eq!(matches[2].end, 11);
        assert_eq!(matches[3].start, 12);
        assert_eq!(matches[3].end, 15);
        assert_eq!(matches[4].start, 16);
        assert_eq!(matches[4].end, 19);
    }

    #[test]
    fn test_regex_search_no_match() {
        let searcher = RegexSearch::new(r"\d+");
        let text = "abc";
        let matches = searcher.matches(text);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_regex_search_case_insensitive() {
        let searcher = RegexSearch::new(r"(?i)hello");
        let text = "Hello World";
        let matches = searcher.matches(text);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].start, 0);
        assert_eq!(matches[0].end, 5);
    }

    #[test]
    fn test_perform_search_literal() {
        let searcher = LiteralSearch::new("test");
        let lines = vec![
            "This is a test".to_string(),
            "Another line".to_string(),
            "More test cases".to_string(),
        ];
        let results = perform_search(searcher, lines);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].len(), 1);
        assert_eq!(results[1].len(), 0);
        assert_eq!(results[2].len(), 1);
    }

    #[test]
    fn test_perform_search_regex() {
        let searcher = RegexSearch::new(r"\b\d{2}\b");
        let lines = vec![
            "Year 20".to_string(),
            "Number 42 is the answer".to_string(),
            "No match here".to_string(),
        ];
        let results = perform_search(searcher, lines);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].len(), 1);
        assert_eq!(results[1].len(), 1);
        assert_eq!(results[2].len(), 0);
    }
}
