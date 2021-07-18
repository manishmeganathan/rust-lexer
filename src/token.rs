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

