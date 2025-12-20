#![allow(dead_code)]
mod ast;
mod tokens;

use std::{iter::Peekable, str::Chars};

use crate::tokens::Token;

pub(crate) struct Lexer<'a> {
    pos: usize,
    chars: Peekable<Chars<'a>>,
    eof_returned: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            pos: 0,
            eof_returned: false,
            chars: input.chars().peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn advance(&mut self) -> Option<char> {
        self.chars.next().inspect(|_c| {
            self.pos += 1;
        })
    }

    fn skip_whitespaces(&mut self) {
        while let Some(char) = self.peek() {
            if char.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        if self.peek().is_none() {
            if self.eof_returned {
                return None;
            } else {
                self.eof_returned = true;
                return Some(Token::EOF);
            }
        }

        let next_char = self.peek().unwrap();

        match next_char {
            '0'..'9' => {
                let mut number = 0;
                while let Some(c) = self.peek() {
                    if let Some(digit) = c.to_digit(10) {
                        number = number * 10 + digit as i32;
                        self.advance();
                    } else {
                        break;
                    }
                }
                Some(Token::NUMBER(number))
            }
            '+' => {
                self.advance();
                Some(Token::PLUS)
            }
            '-' => {
                self.advance();
                Some(Token::MINUS)
            }
            '*' => {
                self.advance();
                Some(Token::MUL)
            }
            '/' => {
                self.advance();
                Some(Token::DIV)
            }
            '^' => {
                self.advance();
                Some(Token::POW)
            }
            '(' => {
                self.advance();
                Some(Token::LPAREN)
            }
            ')' => {
                self.advance();
                Some(Token::RPAREN)
            }
            '{' => {
                self.advance();
                Some(Token::LCURLY)
            }
            '}' => {
                self.advance();
                Some(Token::RCURLY)
            }
            _ => {
                return None;
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_tokenization() {
    // use crate::ast::Parser;
    use crate::tokens::Token;

    let input = "12 + 24  - (3 * 4) / 2"; // ^ 5 { }";
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();

    let expected_tokens = vec![
        Token::NUMBER(12),
        Token::PLUS,
        Token::NUMBER(24),
        Token::MINUS,
        Token::LPAREN,
        Token::NUMBER(3),
        Token::MUL,
        Token::NUMBER(4),
        Token::RPAREN,
        Token::DIV,
        Token::NUMBER(2),
        // Token::POW,
        // Token::NUMBER(5),
        // Token::LCURLY,
        // Token::RCURLY,
        Token::EOF,
    ];

    loop {
        let token = lexer
            .next_token()
            .expect("Lexer returned None unexpectedly");
        println!("{}", token);
        tokens.push(token.clone());
        if token == Token::EOF {
            break;
        }
    }
    assert_eq!(tokens, expected_tokens);

    // for expected in expected_tokens {
    //     assert_eq!(token, expected);
    // }
}

