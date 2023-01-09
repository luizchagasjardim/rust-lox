use crate::expression::*;
use crate::result::Error;
use crate::statement::Statement;
use crate::token::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Statement, Error> {
        let result = if self.match_token(TokenType::Var) {
            self.variable_declaration()
        } else {
            self.statement()
        };
        if result.is_err() {
            self.synchronize();
        }
        result
    }

    fn variable_declaration(&mut self) -> Result<Statement, Error> {
        if !self.match_identifier() {
            return Err(Error::ExpectedEndOfExpression);
        }
        let TokenType::Identifier(identifier) = self.previous() else { unreachable!() };
        let identifier = identifier.clone();
        // this is equivalent to being null
        let initializer = if self.match_token(TokenType::Equal) {
            Some(self.expression()?)
        } else {
            None
        };

        if !self.match_token(TokenType::Semicolon) {
            return Err(Error::ExpectedEndOfExpression);
        }
        Ok(Statement::VariableDeclaration {
            identifier: identifier.clone(),
            expression: initializer,
        })
    }

    fn statement(&mut self) -> Result<Statement, Error> {
        if self.match_token(TokenType::Print) {
            self.print_statement()
        } else if self.match_token(TokenType::LeftBrace) {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Result<Statement, Error> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        if self.match_token(TokenType::RightBrace) {
            self.advance();
            Ok(Statement::Block(statements))
        } else {
            Err(Error::ExpectedEndOfBlock)
        }
    }
    fn print_statement(&mut self) -> Result<Statement, Error> {
        let value = self.expression();
        if !self.match_token(TokenType::Semicolon) {
            Err(Error::ExpectedEndOfExpression)
        } else {
            Ok(Statement::Print(value?))
        }
    }

    fn expression_statement(&mut self) -> Result<Statement, Error> {
        let value = self.expression();
        if self.match_token(TokenType::Semicolon) {
            Err(Error::ExpectedEndOfExpression)
        } else {
            Ok(Statement::Expression(value?))
        }
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, Error> {
        let expr = self.equality()?;

        if self.match_token(TokenType::Equal) {
            let equals = self.previous();
            let value = self.assignment()?;
            return if let Expression::Variable(identifier) = expr {
                Ok(Expression::Assignment {
                    identifier,
                    value: Box::new(value),
                })
            } else {
                Err(Error::InvalidAssignmentTarget)
            };
        }
        Ok(expr)
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
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::GreaterEqual => BinaryOperator::EqualOrGreater,
                TokenType::Less => BinaryOperator::Less,
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
        } else if self.match_identifier() {
            let TokenType::Identifier(string) = self.previous() else { unreachable!() };
            Ok(Expression::Variable(string.clone()))
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

    fn match_identifier(&mut self) -> bool {
        if self.is_at_end() {
            return false;
        }
        let TokenType::Identifier(_) = self.peek().token_type else {
            return false;
        };
        self.advance();
        true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(matches!(
            result,
            Ok(Expression::Literal(Literal::Number(123.0)))
        ));
    }

    #[test]
    fn add_numbers() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::Plus, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: BinaryOperator::Addition,
            right: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }

    #[test]
    fn subtract_numbers() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::Minus, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: BinaryOperator::Subtraction,
            right: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }

    #[test]
    fn multiply_numbers() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::Star, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: BinaryOperator::Multiplication,
            right: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }

    #[test]
    fn divide_numbers() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::Slash, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: BinaryOperator::Division,
            right: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }
    #[test]
    fn equality() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EqualEqual, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: BinaryOperator::Equality,
            right: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }
    #[test]
    fn inequality() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::BangEqual, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: BinaryOperator::Different,
            right: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }
    #[test]
    fn less_or_equal_than() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::LessEqual, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: BinaryOperator::EqualOrLess,
            right: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }
    #[test]
    fn less_than() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::Less, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: BinaryOperator::Less,
            right: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }
    #[test]
    fn greater_than() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::Greater, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: BinaryOperator::Greater,
            right: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }
    #[test]
    fn greater_or_equal_than() {
        let tokens = vec![
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::GreaterEqual, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal(Literal::Number(123.0))),
            operator: BinaryOperator::EqualOrGreater,
            right: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }
    #[test]
    fn negation() {
        let tokens = vec![
            Token::new(TokenType::Bang, 0),
            Token::new(TokenType::True, 0),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Unary {
            operator: UnaryOperator::Negation,
            expression: Box::new(Expression::Literal(Literal::True)),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }
    #[test]
    fn minus() {
        let tokens = vec![
            Token::new(TokenType::Minus, 0),
            Token::new(
                TokenType::Number {
                    value: 123 as f64,
                    length: 5,
                },
                0,
            ),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Unary {
            operator: UnaryOperator::Minus,
            expression: Box::new(Expression::Literal(Literal::Number(123.0))),
        };
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }

    #[test]
    fn match_false() {
        let tokens = vec![
            Token::new(TokenType::False, 0),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Literal(Literal::False);
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }

    #[test]
    fn match_nil() {
        let tokens = vec![Token::new(TokenType::Nil, 0), Token::new(TokenType::EOF, 0)];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Literal(Literal::Nil);
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }

    #[test]
    fn match_string() {
        let tokens = vec![
            Token::new(TokenType::String("baseado".to_string()), 0),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Literal(Literal::String("baseado".to_string()));
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }

    #[test]
    fn match_paren() {
        let tokens = vec![
            Token::new(TokenType::LeftParen, 0),
            Token::new(TokenType::String("baseado".to_string()), 0),
            Token::new(TokenType::RightParen, 0),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Literal(Literal::String("baseado".to_string()));
        assert!(result.is_ok());
        assert_eq!(expr, result.unwrap());
    }

    #[test]
    fn unmatch_paren() {
        let tokens = vec![
            Token::new(TokenType::LeftParen, 0),
            Token::new(TokenType::String("baseado".to_string()), 0),
            Token::new(TokenType::EOF, 0),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        let expr = Expression::Literal(Literal::String("baseado".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn expected_expression() {
        let tokens = vec![Token::new(TokenType::EOF, 0)];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
    }
}
