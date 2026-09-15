use std::str::FromStr;

use regex::Regex;
use strum_macros::{EnumIter, EnumString};

use crate::ast::Type;

#[derive(Debug, Clone, PartialEq, EnumIter, EnumString)]
pub enum Token {
    Let,
    Fn,
    Proc,

    Plus,
    Minus,
    Div,
    Star,
    BitXor,
    BitOr,
    Amperstand,

    PlusEq,
    MinusEq,
    DivEq,
    MulEq,
    BitXorEq,
    BitOrEq,
    BitAndEq,

    Eq,
    CmpEq,
    Semi,

    Colon,
    Arrow,

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftSquare,
    RightSquare,

    // Must come after all other tokens they could match instead of
    Id(String),
    NumericalLiteral(i32),
    Type(Type),
}

impl Into<Regex> for Token {
    fn into(self) -> Regex {
        let str = match self {
            Token::Let => "^let",
            Token::Fn => "^fn",
            Token::Proc => "^proc",

            Token::Plus => "^\\+",
            Token::Minus => "^-",
            Token::Div => "^/",
            Token::Star => "^\\+",
            Token::BitXor => "^\\^",
            Token::BitOr => "^\\|",
            Token::Amperstand => "^&",

            Token::PlusEq => "^+=",
            Token::MinusEq => "^-=",
            Token::DivEq => "^/=",
            Token::MulEq => "^*=",
            Token::BitXorEq => "^\\^=",
            Token::BitOrEq => "^\\|=",
            Token::BitAndEq => "^&=",

            Token::Eq => "^=",
            Token::CmpEq => "^==",
            Token::Semi => "^;",

            Token::Colon => "^:",
            Token::Arrow => "^->",

            Token::LeftParen => "^\\(",
            Token::RightParen => "^\\)",
            Token::LeftBracket => "^\\{",
            Token::RightBracket => "^\\}",
            Token::LeftSquare => "^\\[",
            Token::RightSquare => "^\\]",

            Token::Id(_) => "^[a-zA-Z_]+",
            Token::NumericalLiteral(_) => "^[0-9]+",
            Token::Type(_) => "^(u8|u16|u32|u64|i8|i16|i32|i64|usize)",
        };

        Regex::from_str(str).expect("Invalid RegEx: Bug in Venlyn")
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Let => "let",
            Token::Id(id) => id,
            Token::NumericalLiteral(n) => return n.to_string(),

            Token::Fn => "fn",
            Token::Proc => "proc",

            Token::Plus => "+",
            Token::Minus => "-",
            Token::Div => "/",
            Token::Star => "+",
            Token::BitXor => "^",
            Token::BitOr => "|",
            Token::Amperstand => "&",

            Token::PlusEq => "+=",
            Token::MinusEq => "-=",
            Token::DivEq => "/=",
            Token::MulEq => "*=",
            Token::BitXorEq => "^=",
            Token::BitOrEq => "|=",
            Token::BitAndEq => "&=",

            Token::Eq => "=",
            Token::CmpEq => "==",
            Token::Semi => ";",

            Token::Colon => ":",
            Token::Arrow => "->",

            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LeftBracket => "{",
            Token::RightBracket => "}",
            Token::LeftSquare => "[",
            Token::RightSquare => "]",

            Token::Type(t) => return t.to_string(),
        }
        .to_string()
    }
}
