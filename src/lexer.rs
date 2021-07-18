// This module contains the Lexer struct and its implementations

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

