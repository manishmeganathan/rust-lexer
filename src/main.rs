mod lexer;
mod token;
mod utils;

use std::io::{self, Write};


fn main() {
    loop {
        print!("lexer >: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
    }    
}