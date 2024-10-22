use crate::tokenizer::{Function, Token};
use std::cmp::PartialEq;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Add,
    Del(DelTarget),
    Len,
    Slice(Option<usize>, Option<usize>),
    Iterate,
    Identity,
    Object(String),
    Index(usize),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DelTarget {
    ArrayIndexes(Vec<usize>),
    Object(String),
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Unexpected token: {:?}", .0)]
    UnexpectedToken(Token),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
}

#[derive(Debug, PartialEq)]
enum ParseState {
    Operation,
    Pipe,
}

pub fn parse_filter(tokens: Vec<Token>) -> Result<Vec<Operation>, ParseError> {
    let mut operations = Vec::new();
    let mut iter = tokens.into_iter().peekable();
    let mut state = ParseState::Operation;

    while let Some(token) = iter.next() {
        match token {
            Token::Dot if state == ParseState::Operation => {
                if let Some(first_filter_token) = iter.next() {
                    match first_filter_token {
                        Token::Pipe => {
                            operations.push(Operation::Identity);
                        }
                        Token::Literal(identifier) => {
                            // Object identifier syntax
                            operations.push(Operation::Object(identifier));
                            state = ParseState::Pipe;
                        }
                        Token::LeftBracket => {
                            // Could be a slice or an index
                            operations.push(parse_bracket_operation(&mut iter)?);
                            state = ParseState::Pipe;
                        }
                        _ => return Err(ParseError::UnexpectedToken(first_filter_token)),
                    }
                } else {
                    operations.push(Operation::Identity);
                }
            }
            Token::Function(f) if state == ParseState::Operation => match f {
                Function::Add => operations.push(Operation::Add),
                Function::Length => operations.push(Operation::Len),
                Function::Del => {
                    operations.push(parse_del_operation(&mut iter)?);
                    state = ParseState::Pipe;
                }
            },
            Token::Pipe if state == ParseState::Pipe => state = ParseState::Operation,
            _ => return Err(ParseError::UnexpectedToken(token)),
        }
    }
    Ok(operations)
}

fn parse_del_operation<I>(iter: &mut I) -> Result<Operation, ParseError>
where
    I: Iterator<Item = Token>,
{
    match iter.next() {
        Some(Token::LeftParenthesis) => {
            match iter.next() {
                Some(Token::Dot) => {
                    match iter.next() {
                        Some(Token::LeftBracket) => {
                            // Array deletion: del(.[<indexes>])
                            let indexes = parse_array_indexes(iter)?;
                            match iter.next() {
                                Some(Token::RightParenthesis) => {
                                    Ok(Operation::Del(DelTarget::ArrayIndexes(indexes)))
                                }
                                _ => Err(ParseError::UnexpectedEndOfInput),
                            }
                        }
                        Some(Token::Literal(key)) => {
                            // Object deletion: del(.<key_name>)
                            match iter.next() {
                                Some(Token::RightParenthesis) => {
                                    Ok(Operation::Del(DelTarget::Object(key)))
                                }
                                _ => Err(ParseError::UnexpectedEndOfInput),
                            }
                        }
                        _ => Err(ParseError::UnexpectedToken(Token::Dot)),
                    }
                }
                _ => Err(ParseError::UnexpectedToken(Token::LeftParenthesis)),
            }
        }
        _ => Err(ParseError::UnexpectedEndOfInput),
    }
}

fn parse_array_indexes<I>(iter: &mut I) -> Result<Vec<usize>, ParseError>
where
    I: Iterator<Item = Token>,
{
    let mut indexes = Vec::new();
    loop {
        match iter.next() {
            Some(Token::Number(n)) => indexes.push(n),
            Some(Token::RightBracket) => break,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEndOfInput),
        }
    }
    Ok(indexes)
}

fn parse_bracket_operation<I>(iter: &mut I) -> Result<Operation, ParseError>
where
    I: Iterator<Item = Token>,
{
    match iter.next() {
        Some(Token::Colon) => {
            // Case: .[:<NUMBER>]
            let end = parse_number(iter)?;
            match iter.next() {
                Some(Token::RightBracket) => Ok(Operation::Slice(None, Some(end))),
                _ => Err(ParseError::UnexpectedEndOfInput),
            }
        }
        Some(Token::Number(start)) => {
            match iter.next() {
                Some(Token::RightBracket) => Ok(Operation::Index(start)),
                Some(Token::Colon) => {
                    match iter.next() {
                        // Case: .[<NUMBER>:]
                        Some(Token::RightBracket) => Ok(Operation::Slice(Some(start), None)),
                        // Case: .[<NUMBER>:<NUMBER>]
                        Some(Token::Number(end)) => match iter.next() {
                            Some(Token::RightBracket) => {
                                Ok(Operation::Slice(Some(start), Some(end)))
                            }
                            _ => Err(ParseError::UnexpectedEndOfInput),
                        },
                        _ => Err(ParseError::UnexpectedEndOfInput),
                    }
                }
                _ => Err(ParseError::UnexpectedEndOfInput),
            }
        }
        Some(Token::RightBracket) => Ok(Operation::Iterate),
        Some(token) => Err(ParseError::UnexpectedToken(token)),
        None => Err(ParseError::UnexpectedEndOfInput),
    }
}

fn parse_number<I>(iter: &mut I) -> Result<usize, ParseError>
where
    I: Iterator<Item = Token>,
{
    match iter.next() {
        Some(Token::Number(n)) => Ok(n),
        Some(token) => Err(ParseError::UnexpectedToken(token)),
        None => Err(ParseError::UnexpectedEndOfInput),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::Operation::Identity;

    #[test]
    fn test_parse_filter_object_access() {
        let tokens = vec![Token::Dot, Token::Literal("name".to_string())];
        assert_eq!(
            parse_filter(tokens),
            Ok(vec![Operation::Object("name".to_string())])
        );
    }

    #[test]
    fn test_parse_filter_index_operation() {
        let tokens = vec![
            Token::Dot,
            Token::LeftBracket,
            Token::Number(2),
            Token::RightBracket,
        ];
        assert_eq!(parse_filter(tokens), Ok(vec![Operation::Index(2)]));
    }

    #[test]
    fn test_parse_filter_slice_operation() {
        let tokens = vec![
            Token::Dot,
            Token::LeftBracket,
            Token::Number(1),
            Token::Colon,
            Token::Number(3),
            Token::RightBracket,
        ];
        assert_eq!(
            parse_filter(tokens),
            Ok(vec![Operation::Slice(Some(1), Some(3))])
        );
    }

    #[test]
    fn test_parse_filter_open_ended_slice() {
        let tokens = vec![
            Token::Dot,
            Token::LeftBracket,
            Token::Number(2),
            Token::Colon,
            Token::RightBracket,
        ];
        assert_eq!(
            parse_filter(tokens),
            Ok(vec![Operation::Slice(Some(2), None)])
        );
    }

    #[test]
    fn test_parse_pipe() {
        let tokens = vec![Token::Dot, Token::Pipe, Token::Dot];
        assert_eq!(parse_filter(tokens), Ok(vec![Identity, Identity]));
    }
    #[test]
    fn test_parse_filter_combination_of_operations() {
        let tokens = vec![
            Token::Dot,
            Token::Literal("users".to_string()),
            Token::Pipe,
            Token::Dot,
            Token::LeftBracket,
            Token::Number(0),
            Token::RightBracket,
            Token::Pipe,
            Token::Dot,
            Token::Literal("name".to_string()),
        ];
        assert_eq!(
            parse_filter(tokens),
            Ok(vec![
                Operation::Object("users".to_string()),
                Operation::Index(0),
                Operation::Object("name".to_string()),
            ])
        );
    }

    #[test]
    fn test_delete_function() {
        let tokens = vec![
            Token::Dot,
            Token::Pipe,
            Token::Function(Function::Del),
            Token::LeftParenthesis,
            Token::Dot,
            Token::Literal("fizzes".to_string()),
            Token::RightParenthesis,
        ];
        assert_eq!(
            parse_filter(tokens),
            Ok(vec![
                Operation::Identity,
                Operation::Del(DelTarget::Object("fizzes".to_string()))
            ])
        );
    }

    #[test]
    fn test_parse_del_array() {
        let tokens = vec![
            Token::Function(Function::Del),
            Token::LeftParenthesis,
            Token::Dot,
            Token::LeftBracket,
            Token::Number(0),
            Token::RightBracket,
            Token::RightParenthesis,
        ];
        assert_eq!(
            parse_filter(tokens),
            Ok(vec![Operation::Del(DelTarget::ArrayIndexes(vec![0]))])
        );
    }

    #[test]
    fn test_parse_del_object() {
        let tokens = vec![
            Token::Function(Function::Del),
            Token::LeftParenthesis,
            Token::Dot,
            Token::Literal("key".to_string()),
            Token::RightParenthesis,
        ];
        assert_eq!(
            parse_filter(tokens),
            Ok(vec![Operation::Del(DelTarget::Object("key".to_string()))])
        );
    }

    #[test]
    fn test_parse_filter_unexpected_end_of_input() {
        let tokens = vec![Token::Dot, Token::LeftBracket];
        assert!(matches!(
            parse_filter(tokens),
            Err(ParseError::UnexpectedEndOfInput)
        ));
    }

    #[test]
    fn test_parse_filter_unexpected_token() {
        let tokens = vec![Token::Dot, Token::LeftBracket, Token::Dot];
        assert!(matches!(
            parse_filter(tokens),
            Err(ParseError::UnexpectedToken(Token::Dot))
        ));
    }
}
