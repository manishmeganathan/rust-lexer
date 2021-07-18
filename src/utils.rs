// This module contains utility functions for the lexer

use crate::token::TokenType;

// A function that checks if a character is a letter.
pub fn is_alpha(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

// A function that checks if a character is a digit.
pub fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

// A function that returns a Token for a keyword identifier
pub fn lookup_keyword(data: &Vec<char>) -> Result<TokenType, String> {
    let content: String = data.into_iter().collect();

    match &content[..] {
        "TRUE" => Ok(TokenType::TRUE),
        "FALSE" => Ok(TokenType::FALSE),
        "ALL" => Ok(TokenType::ALL),
        _ => Err(String::from("Not a keyword"))
    }
}
