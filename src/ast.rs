use crate::Token;

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
    tokens: Vec<Token>,
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
        match self.peek()? {
            Token::NUMBER(n) => {
                let n = *n;
                self.advance();
                return Some(Expr::Number(n));
            }
            Token::LPAREN => {
                self.advance();
                let exp = self.parse_exp();
                Some(exp)?
            }
            _ => None,
        }
    }

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
}
