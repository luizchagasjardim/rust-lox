use crate::result::*;
use crate::token::*;

pub struct Scanner {
    source: String,
    line_number: usize,
}

impl Scanner {
    pub fn new(source: String, line_number: usize) -> Scanner {
        Scanner {
            source,
            line_number,
        }
    }
    pub fn scan_tokens(&self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars = self.source.chars().enumerate().peekable();
        while let Some(token) = self.scan_token(&mut chars) {
            tokens.push(token?);
        }
        tokens.push(Token {
            token_type: TokenType::EOF,
            location: Location { start: 0, end: 0 },
        });
        Ok(tokens)
    }
    fn scan_token(
        &self,
        chars: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'_>>>,
    ) -> Option<Result<Token>> {
        let (position, character) = chars.next()?;
        let start = position;
        let mut end = position + 1;
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
                if self.advance_if_matches('=', chars) {
                    end += 1;
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.advance_if_matches('=', chars) {
                    end += 1;
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.advance_if_matches('=', chars) {
                    end += 1;
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.advance_if_matches('=', chars) {
                    end += 1;
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '/' => {
                if self.advance_if_matches('/', chars) {
                    while chars.next().is_some() {}
                    return None;
                } else {
                    TokenType::Slash
                }
            }
            _ => {
                return Some(Err(Error::UnexpectedCharacter {
                    character,
                    location: Location { start, end },
                }));
            }
        };
        let location = Location { start, end };
        Some(Ok(Token {
            token_type,
            location,
        }))
    }
    fn advance_if_matches(
        &self,
        expected_next: char,
        chars: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Chars<'_>>>,
    ) -> bool {
        let Some((_, next)) = chars.peek() else {
            return false;
        };
        if next != &expected_next {
            return false;
        }
        chars.next();
        true
    }
}
