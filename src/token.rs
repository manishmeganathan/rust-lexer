// This module contains the token definitions for the lexer

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,
    STRING,

    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    TILDE,
    PIPE,
    ASSIGN,

    LT,
    GT,
    EQ, 
    NEQ,

    COMMA,
    COLON,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    TRUE,
    FALSE,
    ALL
}

// A structure that represents a Token
pub struct Token {
    pub tokentype: TokenType,
    literal: String,
}

// Constructor implementation for Token
impl Token {
    pub fn new(tokentype: TokenType, literal: String) -> Self {
        Self {tokentype, literal}
    }
}

// Display Trait implementation for Token
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {})", self.tokentype, self.literal)
    }
}