# rust-lexer
### A simple tokenizer/lexer, written in Rust

### Usage
Run the binary with cargo, to start the lexer's input loop
```shell
cargo run

Welcome to the lexer. Input any series of characters. 
lexer >:
```

Input any series of character and literals and the lexer will tokenize the character and return a set of Tokens for the input.

### Examples
```shell
lexer >: name = "Reuben"
(IDENT, name)   
(ASSIGN, =)     
(STRING, Reuben)
(EOF, )
```

```shell
lexer >: num = 1224;
(IDENT, num)
(ASSIGN, =) 
(INT, 1224) 
(EOF, ) 
```

```shell
lexer >: 234234 == 564545
(INT, 234234)
(EQ, ==)     
(INT, 564545)
(EOF, )  
```

### Note
This program does not perform any logical computation on the input. It simply tokenizes it and returns it backs a set of Tokens, so logical expressions will not yield evaluated values as a response.