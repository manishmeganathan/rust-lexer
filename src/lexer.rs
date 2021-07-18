// This module contains the Lexer struct and its implementations

use crate::token::{Token, TokenType};
use crate::utils;

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

    // A method of Lexer that reads the next character
    // and returns it without changing the lexer state.
    fn peek_char(&self) -> char {
        // Check if the cursor has crossed the eof
        if self.cursor >= self.source.len() {
            // Return an EOF character
            return '0';
        } else {
            // Return the next character
            return self.source[self.cursor];
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
            // PLUS character
            '+' => {
                // Generate a PLUS Token
                token = Token::new(TokenType::PLUS, "+".to_string());
            },
            // MINUS character
            '-' => {
                // Generate a MINUS Token
                token = Token::new(TokenType::MINUS, "-".to_string());
            },
            // ASTERISK character
            '*' => {
                // Generate an ASTERISK Token
                token = Token::new(TokenType::ASTERISK, "*".to_string());
            },
            // SLASH character
            '/' => {
                // Generate a SLASH Token
                token = Token::new(TokenType::SLASH, "/".to_string());
            },
            // RPAREN character
            ')' => {
                // Generate a RPAREN Token
                token = Token::new(TokenType::RPAREN, ")".to_string());
            },
            // LPAREN character
            '(' => {
                // Generate an LPAREN Token
                token = Token::new(TokenType::LPAREN, "(".to_string());
            },
            // RBRACE character
            '}' => {
                // Generate a RBRACE Token
                token = Token::new(TokenType::RBRACE, "}".to_string());
            },
            // LBRACE character
            '{' => {
                // Generate an LBRACE Token
                token = Token::new(TokenType::LBRACE, "{".to_string());
            },
            // COMMA character
            ',' => {
                // Generate a COMMA Token
                token = Token::new(TokenType::COMMA, ",".to_string());
            },
            // COLON character
            ':' => {
                // Generate a COLON Token
                token = Token::new(TokenType::COLON, ":".to_string());
            },
            // SEMICOLON character
            ';' => {
                // Generate a SEMICOLON Token
                token = Token::new(TokenType::SEMICOLON, ";".to_string());
            },
            // LT character
            '<' => {
                // Generate a LT Token
                token = Token::new(TokenType::LT, "<".to_string());
            },
            // GT character
            '>' => {
                // Generate a GT Token
                token = Token::new(TokenType::GT, ">".to_string());
            },
            // TILDE character
            '~' => {
                // Generate a TILDE Token
                token = Token::new(TokenType::TILDE, "~".to_string());
            },
            // PIPE character
            '|' => {
                // Generate a PIPE Token
                token = Token::new(TokenType::PIPE, "|".to_string());
            },

            // = character
            '=' => {
                // Check if the next character is also an '='
                if self.peek_char() == '=' {
                    // Advance the cursor
                    self.read_char();
                    // Generate an EQ Token
                    token = Token::new(TokenType::EQ, "==".to_string());
                
                // ASSIGN character
                } else {
                    // Generate an ASSIGN Token
                    token = Token::new(TokenType::ASSIGN, "=".to_string());
                }
            },
            // ! character
            '!' => {
                // Check if the next character is an '='
                if self.peek_char() == '=' {
                    // Advance the cursor
                    self.read_char();
                    // Generate a NEQ Token
                    token = Token::new(TokenType::NEQ, "!=".to_string());
                
                // ASSIGN character
                } else {
                    // Generate a BANG Token
                    token = Token::new(TokenType::BANG, "!".to_string());
                }
            },

            // String literals
            '"' => {
                // Read the string literal fully
                let value = self.read_string();
                // Generate a STRING token
                token = Token::new(TokenType::STRING, value.iter().collect());
            }

            // Not a single character 
            _ => {
                // Check if char is a letter
                if utils::is_alpha(self.ch) {
                    // Read full identifier
                    let ident = self.read_identifier();
                    // Convert vector char into a string
                    let identstr = ident.clone().into_iter().collect();

                    // Check if the identifer is a keyword
                    match utils::lookup_keyword(&ident) {
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
                } else if utils::is_digit(self.ch) {
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