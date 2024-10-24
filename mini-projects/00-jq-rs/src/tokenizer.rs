use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TokenizerError {
    #[error("unknown character encountered: {0}")]
    UnknownCharacter(char),

    #[error("failed to parse number from string: {0}")]
    NumberParseError(#[from] std::num::ParseIntError),
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Dot,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
    Pipe,
    Function(Function),
    Colon,
    Literal(String),
    Number(usize),
}

#[derive(PartialEq, Debug)]
pub enum Function {
    Add,
    Length,
    Del,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '.' => tokens.push(Token::Dot),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            '(' => tokens.push(Token::LeftParenthesis),
            ')' => tokens.push(Token::RightParenthesis),
            '|' => tokens.push(Token::Pipe),
            ':' => tokens.push(Token::Colon),
            c if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                s.push(c);
                while let Some(c) = chars.next_if(|c| c.is_alphanumeric() || *c == '_') {
                    //this is not really correct - real JSON key names can be arbitrary Unicode strings, but Ayush told me this is fine
                    s.push(c);
                }
                match s.as_str() {
                    "add" => tokens.push(Token::Function(Function::Add)),
                    "length" | "len" => tokens.push(Token::Function(Function::Length)),
                    "del" => tokens.push(Token::Function(Function::Del)),
                    _ => tokens.push(Token::Literal(s.clone())),
                }
            }
            c if c.is_numeric() => {
                let mut s = String::new();

                s.push(c);
                while let Some(c) = chars.next_if(|c| c.is_numeric()) {
                    s.push(c)
                }

                let n = s.parse::<usize>()?;

                tokens.push(Token::Number(n));
            }
            c if c.is_whitespace() => {
                //ignore whitespace
                continue;
            }
            _ => {
                // Handle other characters or errors here
                return Err(TokenizerError::UnknownCharacter(c));
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_string() {
        let input = ". | del(.fizzes)";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token::Dot,
                Token::Pipe,
                Token::Function(Function::Del),
                Token::LeftParenthesis,
                Token::Dot,
                Token::Literal("fizzes".to_string()),
                Token::RightParenthesis,
            ])
        );
    }

    #[test]
    fn test_basic_tokens() {
        let input = ". [ ] ( ) | :";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token::Dot,
                Token::LeftBracket,
                Token::RightBracket,
                Token::LeftParenthesis,
                Token::RightParenthesis,
                Token::Pipe,
                Token::Colon,
            ])
        );
    }

    #[test]
    fn test_functions_and_literals() {
        let input = "add len del custom_function";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token::Function(Function::Add),
                Token::Function(Function::Length),
                Token::Function(Function::Del),
                Token::Literal("custom_function".to_string()),
            ])
        );
    }

    #[test]
    fn test_numbers_and_whitespace() {
        let input = "42 123   456";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token::Number(42),
                Token::Number(123),
                Token::Number(456),
            ])
        );
    }

    #[test]
    fn test_mixed_tokens() {
        let input = ".items[0] | add(5) : len";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            Ok(vec![
                Token::Dot,
                Token::Literal("items".to_string()),
                Token::LeftBracket,
                Token::Number(0),
                Token::RightBracket,
                Token::Pipe,
                Token::Function(Function::Add),
                Token::LeftParenthesis,
                Token::Number(5),
                Token::RightParenthesis,
                Token::Colon,
                Token::Function(Function::Length),
            ])
        );
    }

    #[test]
    fn test_unknown_character() {
        let input = "valid @invalid";
        let tokens = tokenize(input);
        assert_eq!(tokens, Err(TokenizerError::UnknownCharacter('@')));
    }
}
