use crate::result::*;
use crate::token::*;

pub struct Scanner<'a> {
    line_number: usize,
    chars: std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'a>>>,
}

impl Scanner<'_> {
    pub fn new(source: &str, line_number: usize) -> Scanner {
        Scanner {
            line_number,
            chars: source.chars().enumerate().peekable(),
        }
    }
    pub fn scan_tokens(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.scan_token() {
            tokens.push(token?);
        }
        tokens.push(Token {
            token_type: TokenType::EOF,
            start: 0,
        });
        Ok(tokens)
    }
    fn scan_token(&mut self) -> Option<Result<Token>> {
        let (start, character) = self.chars.next()?;
        let token_type = match character {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '!' => {
                if self.advance_if_matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.advance_if_matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.advance_if_matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.advance_if_matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '/' => {
                if self.advance_if_matches('/') {
                    while self.chars.next().is_some() {}
                    return None;
                } else {
                    TokenType::Slash
                }
            }
            ' ' | '\r' | '\t' | '\n' => {
                return self.scan_token();
            }
            '"' => match self.scan_string() {
                Ok(string) => TokenType::String(string),
                Err(string) => {
                    return Some(Err(Error::UnterminatedString {
                        string,
                        position: start,
                    }));
                }
            },
            '0'..='9' => match self.scan_number(character) {
                Ok((value, length)) => TokenType::Number { value, length },
                Err(string) => {
                    return Some(Err(Error::UnterminatedNumber {
                        string,
                        position: start,
                    }));
                }
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier_or_keyword = self.scan_identifier_or_keyword(character);
                if identifier_or_keyword == "and" {
                    TokenType::And
                } else if identifier_or_keyword == "class" {
                    TokenType::Class
                } else if identifier_or_keyword == "else" {
                    TokenType::Else
                } else if identifier_or_keyword == "false" {
                    TokenType::False
                } else if identifier_or_keyword == "fun" {
                    TokenType::Fun
                } else if identifier_or_keyword == "for" {
                    TokenType::For
                } else if identifier_or_keyword == "if" {
                    TokenType::If
                } else if identifier_or_keyword == "nil" {
                    TokenType::Nil
                } else if identifier_or_keyword == "or" {
                    TokenType::Or
                } else if identifier_or_keyword == "print" {
                    TokenType::Print
                } else if identifier_or_keyword == "return" {
                    TokenType::Return
                } else if identifier_or_keyword == "super" {
                    TokenType::Super
                } else if identifier_or_keyword == "this" {
                    TokenType::This
                } else if identifier_or_keyword == "true" {
                    TokenType::True
                } else if identifier_or_keyword == "var" {
                    TokenType::Var
                } else if identifier_or_keyword == "while" {
                    TokenType::While
                } else {
                    TokenType::Identifier(identifier_or_keyword)
                }
            }
            _ => {
                return Some(Err(Error::UnexpectedCharacter {
                    character,
                    position: start,
                }));
            }
        };
        Some(Ok(Token { token_type, start }))
    }
    fn advance_if_matches(&mut self, expected_next: char) -> bool {
        let Some((_, next)) = self.chars.peek() else {
            return false;
        };
        if next != &expected_next {
            return false;
        }
        self.chars.next();
        true
    }
    fn scan_string(&mut self) -> std::result::Result<String, String> {
        let mut value = String::new();
        while let Some((_, character)) = self.chars.next() {
            if character == '"' {
                return Ok(value);
            }
            value.push(character);
        }
        Err(value)
    }
    fn scan_number(&mut self, first_digit: char) -> std::result::Result<(f64, usize), String> {
        let mut value = String::from(first_digit);

        let integer_part = self.scan_integer()?;
        value += &integer_part;

        if let Some((_, character)) = self.chars.peek() {
            if *character == '.' {
                value.push(*character);
                self.chars.next();
                let fractional_part = self.scan_integer()?;
                if fractional_part.is_empty() {
                    return Err(value);
                }
                value += &fractional_part;
            }
        }
        Ok((value.parse().unwrap(), value.len()))
    }
    fn scan_integer(&mut self) -> std::result::Result<String, String> {
        let mut value = String::new();
        while let Some((_, character)) = self.chars.peek() {
            match character {
                '0'..='9' => {
                    value.push(*character);
                    self.chars.next();
                }
                '.' | ' ' | '\r' | '\t' | '\n' => {
                    break;
                }
                _ => {
                    return Err(value);
                }
            }
        }
        Ok(value)
    }
    fn scan_identifier_or_keyword(&mut self, first_char: char) -> String {
        let mut value = String::from(first_char);
        while let Some((_, character)) = self.chars.peek() {
            match character {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    value.push(*character);
                    self.chars.next();
                }
                _ => {
                    break;
                }
            }
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_empty_line() {
        let tokens = Scanner::new("", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![Token {
                token_type: TokenType::EOF,
                start: 0
            }]
        );
    }

    #[test]
    fn scan_whitespace_line() {
        let tokens = Scanner::new("\t ", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![Token {
                token_type: TokenType::EOF,
                start: 0
            }]
        );
    }

    #[test]
    fn scan_comment_line() {
        let tokens = Scanner::new("\t//comment", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![Token {
                token_type: TokenType::EOF,
                start: 0
            }]
        );
    }

    #[test]
    fn scan_single_character_tokens() {
        let tokens = Scanner::new("(}{,+)", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::LeftParen,
                    start: 0
                },
                Token {
                    token_type: TokenType::RightBrace,
                    start: 1
                },
                Token {
                    token_type: TokenType::LeftBrace,
                    start: 2
                },
                Token {
                    token_type: TokenType::Comma,
                    start: 3
                },
                Token {
                    token_type: TokenType::Plus,
                    start: 4
                },
                Token {
                    token_type: TokenType::RightParen,
                    start: 5
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_single_or_double_character_tokens() {
        let tokens = Scanner::new("!(!= = >=< =", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Bang,
                    start: 0
                },
                Token {
                    token_type: TokenType::LeftParen,
                    start: 1
                },
                Token {
                    token_type: TokenType::BangEqual,
                    start: 2
                },
                Token {
                    token_type: TokenType::Equal,
                    start: 5
                },
                Token {
                    token_type: TokenType::GreaterEqual,
                    start: 7
                },
                Token {
                    token_type: TokenType::Less,
                    start: 9
                },
                Token {
                    token_type: TokenType::Equal,
                    start: 11
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_string() {
        let tokens = Scanner::new("\"my string\"", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::String("my string".to_string()),
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_unterminated_string() {
        let tokens = Scanner::new("\"my unterminated string", 0).scan_tokens();
        assert!(tokens.is_err());
        assert!(matches!(
            tokens.unwrap_err(),
            Error::UnterminatedString { .. }
        ));
    }

    #[test]
    fn scan_integer() {
        let tokens = Scanner::new("123", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Number {
                        value: 123.0,
                        length: 3
                    },
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_decimal() {
        let tokens = Scanner::new("123.0", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Number {
                        value: 123.0,
                        length: 5
                    },
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_unterminated_number() {
        let tokens = Scanner::new("123.", 0).scan_tokens();
        assert!(tokens.is_err());
        assert!(matches!(
            tokens.unwrap_err(),
            Error::UnterminatedNumber { .. }
        ));
    }

    #[test]
    fn scan_keyword_and() {
        let tokens = Scanner::new("and", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::And,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_class() {
        let tokens = Scanner::new("class", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Class,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_else() {
        let tokens = Scanner::new("else", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Else,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_false() {
        let tokens = Scanner::new("false", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::False,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_fun() {
        let tokens = Scanner::new("fun", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Fun,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_for() {
        let tokens = Scanner::new("for", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::For,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_if() {
        let tokens = Scanner::new("if", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::If,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_nil() {
        let tokens = Scanner::new("nil", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Nil,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_or() {
        let tokens = Scanner::new("or", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Or,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_print() {
        let tokens = Scanner::new("print", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Print,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_return() {
        let tokens = Scanner::new("return", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Return,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_super() {
        let tokens = Scanner::new("super", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Super,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_this() {
        let tokens = Scanner::new("this", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::This,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_true() {
        let tokens = Scanner::new("true", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::True,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_var() {
        let tokens = Scanner::new("var", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Var,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_keyword_while() {
        let tokens = Scanner::new("while", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::While,
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_identifier() {
        let tokens = Scanner::new("myvariable", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Identifier("myvariable".to_string()),
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_identifier_with_underscores() {
        let tokens = Scanner::new("__my_var__iable_", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Identifier("__my_var__iable_".to_string()),
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_identifier_with_uppercase_letters() {
        let tokens = Scanner::new("MyVariable", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Identifier("MyVariable".to_string()),
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_identifier_with_number() {
        let tokens = Scanner::new("my1variable", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Identifier("my1variable".to_string()),
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }

    #[test]
    fn scan_identifier_starting_with_keyword() {
        let tokens = Scanner::new("whileforandorvariable", 0).scan_tokens();
        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    token_type: TokenType::Identifier("whileforandorvariable".to_string()),
                    start: 0
                },
                Token {
                    token_type: TokenType::EOF,
                    start: 0
                }
            ]
        );
    }
}
