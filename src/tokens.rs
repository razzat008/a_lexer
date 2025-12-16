#![allow(unused_variables, dead_code)]

use std::fmt::{Display, Write};

#[derive(Debug, PartialEq, Clone)]
pub(super) enum Token {
    NUMBER(i32),
    PLUS,
    MINUS,
    DIV,
    MUL,
    POW,
    LPAREN,
    RPAREN,
    RCURLY,
    LCURLY,
    EOF,
    WhiteSpace(WhiteSpace),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NUMBER(n) => write!(f, "{}", n),
            Self::PLUS => write!(f, "+"),
            Self::MINUS => write!(f, "-"),
            Self::DIV => write!(f, "/"),
            Self::MUL => write!(f, "*"),
            Self::POW => write!(f, "^"),
            Self::LPAREN => write!(f, "("),
            Self::RPAREN => write!(f, ")"),
            Self::RCURLY => write!(f, "{{"),
            Self::LCURLY => write!(f, "}}"),
            Self::EOF => write!(f, "EOF"),
            Self::WhiteSpace(whitespace) => write!(f, "{whitespace}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum WhiteSpace {
    SPACE,
    TAB,
    NEWLINE,
}

impl Display for WhiteSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::TAB => '\t',
            Self::SPACE => ' ',
            Self::NEWLINE => '\n',
        })
    }
}
