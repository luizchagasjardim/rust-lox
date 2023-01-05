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

pub const AND_KEYWORD: &'static str = "and";
pub const CLASS_KEYWORD: &'static str = "class";
pub const ELSE_KEYWORD: &'static str = "else";
pub const FALSE_KEYWORD: &'static str = "false";
pub const FUN_KEYWORD: &'static str = "fun";
pub const FOR_KEYWORD: &'static str = "for";
pub const IF_KEYWORD: &'static str = "if";
pub const NIL_KEYWORD: &'static str = "nil";
pub const OR_KEYWORD: &'static str = "or";
pub const PRINT_KEYWORD: &'static str = "print";
pub const RETURN_KEYWORD: &'static str = "return";
pub const SUPER_KEYWORD: &'static str = "super";
pub const THIS_KEYWORD: &'static str = "this";
pub const TRUE_KEYWORD: &'static str = "true";
pub const VAR_KEYWORD: &'static str = "var";
pub const WHILE_KEYWORD: &'static str = "while";

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
            TokenType::And => AND_KEYWORD.len(),
            TokenType::Class => CLASS_KEYWORD.len(),
            TokenType::Else => ELSE_KEYWORD.len(),
            TokenType::False => FALSE_KEYWORD.len(),
            TokenType::Fun => FUN_KEYWORD.len(),
            TokenType::For => FOR_KEYWORD.len(),
            TokenType::If => IF_KEYWORD.len(),
            TokenType::Nil => NIL_KEYWORD.len(),
            TokenType::Or => OR_KEYWORD.len(),
            TokenType::Print => PRINT_KEYWORD.len(),
            TokenType::Return => RETURN_KEYWORD.len(),
            TokenType::Super => SUPER_KEYWORD.len(),
            TokenType::This => THIS_KEYWORD.len(),
            TokenType::True => TRUE_KEYWORD.len(),
            TokenType::Var => VAR_KEYWORD.len(),
            TokenType::While => WHILE_KEYWORD.len(),
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

    #[test]
    fn and_length() {
        assert_eq!(TokenType::And.length(), 3);
    }

    #[test]
    fn class_length() {
        assert_eq!(TokenType::Class.length(), 5);
    }

    #[test]
    fn else_length() {
        assert_eq!(TokenType::Else.length(), 4);
    }

    #[test]
    fn false_length() {
        assert_eq!(TokenType::False.length(), 5);
    }

    #[test]
    fn fun_length() {
        assert_eq!(TokenType::Fun.length(), 3);
    }

    #[test]
    fn for_length() {
        assert_eq!(TokenType::For.length(), 3);
    }

    #[test]
    fn if_length() {
        assert_eq!(TokenType::If.length(), 2);
    }

    #[test]
    fn nil_length() {
        assert_eq!(TokenType::Nil.length(), 3);
    }

    #[test]
    fn or_length() {
        assert_eq!(TokenType::Or.length(), 2);
    }

    #[test]
    fn print_length() {
        assert_eq!(TokenType::Print.length(), 5);
    }

    #[test]
    fn return_length() {
        assert_eq!(TokenType::Return.length(), 6);
    }

    #[test]
    fn super_length() {
        assert_eq!(TokenType::Super.length(), 5);
    }

    #[test]
    fn this_length() {
        assert_eq!(TokenType::This.length(), 4);
    }

    #[test]
    fn true_length() {
        assert_eq!(TokenType::True.length(), 4);
    }

    #[test]
    fn var_length() {
        assert_eq!(TokenType::Var.length(), 3);
    }

    #[test]
    fn while_length() {
        assert_eq!(TokenType::While.length(), 5);
    }
    #[test]
    fn string_length() {
        assert_eq!(TokenType::String("ola".to_string()).length(), 3);
    }
    #[test]
    fn number_length() {
        assert_eq!(
            TokenType::Number {
                value: 2.0,
                length: 1
            }
            .length(),
            1
        );
    }
}
