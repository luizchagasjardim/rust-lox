use crate::result::*;
use crate::token::*;

pub struct Scanner<'a> {
    line_number: usize,
    chars: std::iter::Peekable::<std::iter::Enumerate<std::str::Chars<'a>>>,
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
        let (position, character) = self.chars.next()?;
        let start = position;
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
                todo!();
            }
            _ => {
                return Some(Err(Error::UnexpectedCharacter {
                    character,
                    position: start,
                }));
            }
        };
        Some(Ok(Token {
            token_type,
            start,
        }))
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
}
