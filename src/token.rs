#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier(String),
    String(String),
    Number { value: f64, length: usize },
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

impl TokenType {
    pub fn length(&self) -> usize {
        match self {
            TokenType::LeftParen => 1,
            TokenType::RightParen => 1,
            TokenType::LeftBrace => 1,
            TokenType::RightBrace => 1,
            TokenType::Comma => 1,
            TokenType::Dot => 1,
            TokenType::Minus => 1,
            TokenType::Plus => 1,
            TokenType::Semicolon => 1,
            TokenType::Slash => 1,
            TokenType::Star => 1,
            TokenType::Bang => 1,
            TokenType::BangEqual => 2,
            TokenType::Equal => 1,
            TokenType::EqualEqual => 2,
            TokenType::Greater => 1,
            TokenType::GreaterEqual => 2,
            TokenType::Less => 1,
            TokenType::LessEqual => 2,
            TokenType::Identifier(string) => string.len(),
            TokenType::String(string) => string.len(),
            TokenType::Number { value: _, length } => *length,
            TokenType::And => todo!(),
            TokenType::Class => todo!(),
            TokenType::Else => todo!(),
            TokenType::False => todo!(),
            TokenType::Fun => todo!(),
            TokenType::For => todo!(),
            TokenType::If => todo!(),
            TokenType::Nil => todo!(),
            TokenType::Or => todo!(),
            TokenType::Print => todo!(),
            TokenType::Return => todo!(),
            TokenType::Super => todo!(),
            TokenType::This => todo!(),
            TokenType::True => todo!(),
            TokenType::Var => todo!(),
            TokenType::While => todo!(),
            TokenType::EOF => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_paren_length() {
        assert_eq!(TokenType::LeftParen.length(), 1,);
    }
    #[test]
    fn right_paren_length() {
        assert_eq!(TokenType::RightParen.length(), 1,);
    }
    #[test]
    fn left_brace_length() {
        assert_eq!(TokenType::LeftBrace.length(), 1,);
    }
    #[test]
    fn right_brace_length() {
        assert_eq!(TokenType::RightBrace.length(), 1,);
    }
    #[test]
    fn comma_length() {
        assert_eq!(TokenType::Comma.length(), 1,);
    }
    #[test]
    fn dot_length() {
        assert_eq!(TokenType::Dot.length(), 1,);
    }
    #[test]
    fn minus_length() {
        assert_eq!(TokenType::Minus.length(), 1,);
    }

    #[test]
    fn plus_length() {
        assert_eq!(TokenType::Plus.length(), 1,);
    }

    #[test]
    fn semicolon_length() {
        assert_eq!(TokenType::Semicolon.length(), 1,);
    }

    #[test]
    fn slash_length() {
        assert_eq!(TokenType::Slash.length(), 1,);
    }
    #[test]
    fn star_length() {
        assert_eq!(TokenType::Star.length(), 1,);
    }
    #[test]
    fn bang_length() {
        assert_eq!(TokenType::Bang.length(), 1,);
    }

    #[test]
    fn bang_equal_length() {
        assert_eq!(TokenType::BangEqual.length(), 2,);
    }
    #[test]
    fn equal_length() {
        assert_eq!(TokenType::Equal.length(), 1,);
    }
    #[test]
    fn equal_equal_length() {
        assert_eq!(TokenType::EqualEqual.length(), 2,);
    }

    #[test]
    fn greater_length() {
        assert_eq!(TokenType::Greater.length(), 1,);
    }
    #[test]
    fn greater_equal_length() {
        assert_eq!(TokenType::GreaterEqual.length(), 2,);
    }
    #[test]
    fn less_length() {
        assert_eq!(TokenType::Less.length(), 1,);
    }
    #[test]
    fn less_equal_length() {
        assert_eq!(TokenType::LessEqual.length(), 2,);
    }

    #[test]
    fn identifier_length() {
        assert_eq!(
            TokenType::Identifier("baseado".parse().unwrap()).length(),
            7,
        );
    }

    #[test]
    fn eof_length() {
        assert_eq!(TokenType::EOF.length(), 0,);
    }
}
