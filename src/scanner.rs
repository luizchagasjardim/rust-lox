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
        let mut chars = self.source.chars().enumerate();
        while let Some(token) = self.scan_token(&mut chars) {
            //TODO: error handling
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
        chars: &mut std::iter::Enumerate<std::str::Chars<'_>>,
    ) -> Option<Result<Token>> {
        let (position, character) = chars.next()?;
        // let Some((position, character)) = chars.next() else {
        // let location = Location {
        //     start: 0,
        //     end: 0,
        // };
        // return Ok(Token {
        //     token_type: TokenType::EOF,
        //     location,
        // });
        // };
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
            _ => {
                return Some(Err(Error::UnexpectedCharacter {
                    character,
                    location: Location {
                        start: position,
                        end: position + 1,
                    },
                }));
            }
        };
        let location = Location {
            start: position,
            end: position + 1,
        };
        Some(Ok(Token {
            token_type,
            location,
        }))
    }
}
