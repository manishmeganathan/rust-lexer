// This module contains the Lexer struct and its implementations

use crate::token::{Token, TokenType};
use crate::utils::{is_alpha, is_digit, lookup_keyword};

// A structure that represents a Lexer
pub struct Lexer {
    pub source: Vec<char>,
    pub position: usize,
    pub cursor: usize,
    pub ch: char
}

// Constructor Implementation of the Lexer
impl Lexer {
    // A constructor function for the Lexer struct
    pub fn new(input: String) -> Self {
        // Prepare the input (convert to upper vector chars)
        let source = input.chars().collect();
        // Create a lexer object
        let mut l = Self {
            source,
            position: 0,
            cursor: 0,
            ch: '0'
        };

        // Initialize the lexer by reading the first character
        // This will set the cursor, position and ch fields of the lexer
        l.read_char();

        // Return the created lexer
        return l;
    } 
}

impl Lexer {
    // A method of Lexer that lexes the source input
    pub fn lex(&mut self) -> Vec<Token> {
        // Declare a vector of Tokens
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        // Declare a accumulation Token variable
        let mut newtoken: Token;

        loop {
            // Lex the current token
            newtoken = self.next_token();

            // Check if the token is an EOF token
            if newtoken.tokentype == TokenType::EOF {
                // Push the token into the vector
                tokens.push(newtoken);
                // Break the loop
                break;
            }

            // Push the token into the vector
            tokens.push(newtoken);
        }

        // Return the vector of Tokens
        return tokens;
    }
}

// Navigation methods of the Lexer
impl Lexer {
    // A method of Lexer that reads and consumes all white 
    // spaces until the next non whitespace character
    fn eat_whitespaces(&mut self) {
        // Iterate until character is a non white space character
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            // Read the character and consume it
            self.read_char();
        }
    }

    // A method of Lexer that lexes the current char 
    // into a Token and advances the lex cursor
    fn next_token(&mut self) -> Token {
        // Declare a Token
        let token: Token;
        // Consume any white space chars
        self.eat_whitespaces();

        // Check the value of the current char
        match self.ch {
            // EOF character
            '0' => {
                // Generate an EOF Token
                token = Token::new(TokenType::EOF, "".to_string());
            },

            // Not a single character 
            _ => {
                // Check if char is a letter
                if is_alpha(self.ch) {
                    // Read full identifier
                    let ident = self.read_identifier();
                    // Convert vector char into a string
                    let identstr = ident.clone().into_iter().collect();

                    // Check if the identifer is a keyword
                    match lookup_keyword(&ident) {
                        // Keyword
                        Ok(keyword_token) => {
                            // Generate the appropriate Keyword Token
                            token = Token::new(keyword_token, identstr);
                        },

                        // Identifier 
                        Err(_err) => {
                            // Generate an Identifier Token
                            token = Token::new(TokenType::IDENT, identstr);
                        }
                    }

                // Check if char is a digit
                } else if is_digit(self.ch) {
                    // Read full number
                    let number = self.read_number();
                    // Convert vector char into a string
                    let numberstr = number.clone().into_iter().collect();

                    // Generate an INT Token
                    token = Token::new(TokenType::INT, numberstr);
                
                // Illegal char
                } else {
                    // Generate an Illegal Token
                    token = Token::new(TokenType::ILLEGAL, self.ch.to_string());
                }
            }
        }

        // Advance the lex parser
        self.read_char();
        // Return the parsed token
        return token;
    }
}