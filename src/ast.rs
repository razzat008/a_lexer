
use crate::{Token};

pub enum Expr {
    Number(i32),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
pub enum UnaryOp {
    Neg,
}

// parser struct
// #[derive(nex)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn init(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    // check current token
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    // consume current token and move forward
    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn parse_exp(&mut self) -> Option<Expr> {
        let mut lterm = self.parse_term()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::PLUS => {
                    self.advance();
                    let rterm = self.parse_term()?;
                    lterm = Expr::Binary {
                        op: BinaryOp::Add,
                        left: Box::new(lterm),
                        right: Box::new(rterm),
                    }
                }
                Token::MINUS => {
                    self.advance();
                    let rterm = self.parse_term()?;
                    lterm = Expr::Binary {
                        op: BinaryOp::Sub,
                        left: Box::new(lterm),
                        right: Box::new(rterm),
                    }
                }
                _ => break,
            }
        }
        Some(lterm)
    }

    // Factor -> Number | LPAREN Expr RPAREN
    fn parse_factor(&mut self) -> Option<Expr> {
        let peeked = match self.peek() {
            Some(token) => token,
            None => return None,
        };
        match peeked.clone() {
            Token::NUMBER(n) => {
                self.advance();
                Some(Expr::Number(n))
            }
            Token::LPAREN => {
                self.advance();
                let expr = self.parse_exp()?;
                match self.peek()? {
                    Token::RPAREN => {
                        self.advance();
                        return Some(expr);
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    // Unary -> MINUS Unary | Factor
    fn parse_unary(&mut self) -> Option<Expr> {
        let peeked = self.peek()?;
        match peeked {
            Token::MINUS => {
                self.advance();
                let exp = self.parse_unary()?;
                return Some(Expr::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(exp),
                });
            }
            _ => self.parse_factor(),
        }
    }

    // Term = Unary ((MUL | DIV) Unary )*
    fn parse_term(&mut self) -> Option<Expr> {
        let mut left = self.parse_unary()?;
        while let Some(token) = self.peek() {
            match token {
                Token::MUL => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Binary {
                        op: BinaryOp::Mul,
                        left: Box::new(left),
                        right: Box::new(right),
                    }
                }
                Token::DIV => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Binary {
                        op: BinaryOp::Div,
                        left: Box::new(left),
                        right: Box::new(right),
                    }
                }
                _ => break,
            }
        }

        Some(left)
    }
}
