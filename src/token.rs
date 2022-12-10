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
    Identifier,
    String,
    Number,
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
            TokenType::Identifier => todo!(),
            TokenType::String => todo!(),
            TokenType::Number => todo!(),
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