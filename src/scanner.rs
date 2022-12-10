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
}
