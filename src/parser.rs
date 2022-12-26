use crate::expression::*;
use crate::token::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();
        while self.match_token(TokenType::BangEqual) || self.match_token(TokenType::EqualEqual) {
            let operator_token_type = self.previous();
            let operator = match operator_token_type {
                TokenType::BangEqual => BinaryOperator::Different,
                TokenType::EqualEqual => BinaryOperator::Equality,
                _ => unimplemented!(),
            };
            let right = self.comparison();

            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();
        while self.match_token(TokenType::Greater)
            || self.match_token(TokenType::GreaterEqual)
            || self.match_token(TokenType::Less)
            || self.match_token(TokenType::LessEqual)
        {
            let operator_token_type = self.previous();
            let operator = match operator_token_type {
                TokenType::Greater => BinaryOperator::Different,
                TokenType::GreaterEqual => BinaryOperator::Equality,
                TokenType::Less => BinaryOperator::Equality,
                TokenType::LessEqual => BinaryOperator::EqualOrLess,
                _ => unimplemented!(),
            };
            let right = self.comparison();

            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();
        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let operator = match self.previous() {
                TokenType::Minus => BinaryOperator::Subtraction,
                TokenType::Plus => BinaryOperator::Addition,
                _ => unimplemented!(),
            };
            let right = self.factor();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();
        while self.match_token(TokenType::Slash) || self.match_token(TokenType::Star) {
            let operator = match self.previous() {
                TokenType::Slash => BinaryOperator::Division,
                TokenType::Star => BinaryOperator::Multiplication,
                _ => unreachable!(),
            };
            let right = self.unary();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn unary(&mut self) -> Expression {
        if self.match_token(TokenType::Bang) || self.match_token(TokenType::Minus) {
            let operator = match self.previous() {
                TokenType::Bang => UnaryOperator::Negation,
                TokenType::Minus => UnaryOperator::Minus,
                _ => unreachable!(),
            };
            let expression = self.unary();
            Expression::Unary {
                operator,
                expression: Box::new(expression),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expression {
        todo!()
    }

    fn match_token(&mut self, token: TokenType) -> bool {
        if self.check(token) {
            self.advance();
            return true;
        }
        return false;
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek() == token
    }

    fn advance(&mut self) -> TokenType {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek() == TokenType::EOF
    }

    fn peek(&self) -> TokenType {
        self.tokens[self.current].token_type.clone()
    }
    fn previous(&self) -> TokenType {
        self.tokens[self.current - 1].token_type.clone()
    }
}
