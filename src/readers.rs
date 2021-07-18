// This module contains implementations of Lexer 
// for reading different types of character sets.

use crate::lexer::Lexer;
use crate::utils::{is_digit, is_alpha};

impl Lexer {
    // A method of Lexer that reads the next character
    // into the lexer and advances the lex cursor
    pub fn read_char(&mut self) {
        // Check if the cursor has crossed the eof
        if self.cursor >= self.source.len() {
            // Set the current character to an EOF
            self.ch = '0';
        } else {
            // Set the current character after reading it
            self.ch = self.source[self.cursor];
        }

        // Move the lex position to lex cursor position
        self.position = self.cursor;
        // Move the lex cursor to the next position
        self.cursor = self.cursor + 1;
    }  

    // A method of Lexer that reads a full identifier
    pub fn read_identifier(&mut self) -> Vec<char> {
        // Collect the start position of the identifier
        let start = self.position;

        // Iterate until the char is not a letter
        loop {
            if is_alpha(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }

        // Create a slice of the source that contains the identifier
        let ident = &self.source[start..self.position];
        // Convert the char slice into vector and return it
        return ident.to_vec();
    }

    pub fn read_number(&mut self) -> Vec<char> {
        // Collect the start position of the identifier
        let start = self.position;

        // Iterate until the char is not a letter
        loop {
            if is_digit(self.ch) {
                self.read_char();
            } else {
                break;
            }
        }

        // Create a slice of the source that contains the identifier
        let number = &self.source[start..self.position];
        // Convert the char slice into vector and return it
        return number.to_vec();
    }
}