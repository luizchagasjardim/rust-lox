use crate::expression::*;
use crate::result::Error;
use crate::token::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(mut self) -> Result<Expression, Error> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, Error> {
        let mut expr = self.comparison()?;
        while self.match_token(TokenType::BangEqual) || self.match_token(TokenType::EqualEqual) {
            let operator_token_type = self.previous();
            let operator = match operator_token_type {
                TokenType::BangEqual => BinaryOperator::Different,
                TokenType::EqualEqual => BinaryOperator::Equality,
                _ => unreachable!(),
            };
            let right = self.comparison()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, Error> {
        let mut expr = self.term()?;
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
                _ => unreachable!(),
            };
            let right = self.comparison()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, Error> {
        let mut expr = self.factor()?;
        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let operator = match self.previous() {
                TokenType::Minus => BinaryOperator::Subtraction,
                TokenType::Plus => BinaryOperator::Addition,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, Error> {
        let mut expr = self.unary()?;
        while self.match_token(TokenType::Slash) || self.match_token(TokenType::Star) {
            let operator = match self.previous() {
                TokenType::Slash => BinaryOperator::Division,
                TokenType::Star => BinaryOperator::Multiplication,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, Error> {
        if self.match_token(TokenType::Bang) || self.match_token(TokenType::Minus) {
            let operator = match self.previous() {
                TokenType::Bang => UnaryOperator::Negation,
                TokenType::Minus => UnaryOperator::Minus,
                _ => unreachable!(),
            };
            let expression = self.unary()?;
            Ok(Expression::Unary {
                operator,
                expression: Box::new(expression),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, Error> {
        if self.match_token(TokenType::False) {
            Ok(Expression::Literal(Literal::False))
        } else if self.match_token(TokenType::True) {
            Ok(Expression::Literal(Literal::True))
        } else if self.match_token(TokenType::Nil) {
            Ok(Expression::Literal(Literal::Nil))
        } else if self.match_number() {
            let TokenType::Number { value, .. } = self.previous() else { unreachable!() };
            Ok(Expression::Literal(Literal::Number(*value)))
        } else if self.match_string() {
            let TokenType::String(string) = self.previous() else { unreachable!() };
            Ok(Expression::Literal(Literal::String(string.clone())))
        } else if self.match_token(TokenType::LeftParen) {
            let expression = self.expression();
            if self.match_token(TokenType::RightParen) {
                expression
            } else {
                Err(Error::UnmatchedParenthesis {
                    position: self.peek().start,
                })
            }
        } else {
            Err(Error::ExpectedExpression {
                position: self.peek().start,
            })
        }
    }

    fn match_token(&mut self, token: TokenType) -> bool {
        if self.check(token) {
            self.advance();
            return true;
        }
        false
    }

    fn match_number(&mut self) -> bool {
        if self.is_at_end() {
            return false;
        }
        let TokenType::Number{..} = self.peek().token_type else {
            return false;
        };
        self.advance();
        true
    }

    fn match_string(&mut self) -> bool {
        if self.is_at_end() {
            return false;
        }
        let TokenType::String(_) = self.peek().token_type else {
            return false;
        };
        self.advance();
        true
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token
    }

    fn advance(&mut self) -> &TokenType {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &TokenType {
        &self.tokens[self.current - 1].token_type
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous() == &TokenType::Semicolon {
                return;
            }
            if matches!(
                self.peek().token_type,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }
        }
        self.advance();
    }
}
