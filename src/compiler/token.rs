use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Let,
    Id(String),
    NumericalLiteral(i32),

    Plus,
    Minus,
    ForwardSlash,
    Star,
    BitXor,
    BitOr,
    BitAnd,

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
}

impl From<Token> for Regex {
    fn from(value: Token) -> Self {
        let pattern = match value {
            Token::Let => "let",
            Token::Id(_) => "[a-z_][a-z0-9_]+",
            Token::NumericalLiteral(_) => "-?[0-9]*",
            Token::CmpEq => r"==",
            Token::Eq => r"=",
            Token::Semi => ";",

            Token::Plus => "+",
            Token::Minus => "-",
            Token::ForwardSlash => "/",
            Token::Star => r"\*",

            Token::BitXor => "^",
            Token::BitOr => r"\|",
            Token::BitAnd => r"&",

            Token::PlusEq => r"\+=",
            Token::MinusEq => r"\-=",
            Token::DivEq => r"/=",
            Token::MulEq => r"\*=",
            Token::BitXorEq => r"^=",
            Token::BitOrEq => r"\|=",
            Token::BitAndEq => r"&=",
        };

        Regex::new(pattern).unwrap()
    }
}

impl From<Token> for &str {
    fn from(value: Token) -> Self {
        match value {
            Token::Let => "let",
            Token::CmpEq => r"==",
            Token::Eq => r"=",
            Token::Semi => ";",

            Token::Plus => "+",
            Token::Minus => "-",
            Token::ForwardSlash => "/",
            Token::Star => "*",

            Token::BitXor => "^",
            Token::BitOr => "|",
            Token::BitAnd => "&",

            Token::PlusEq => "+=",
            Token::MinusEq => "-=",
            Token::DivEq => "=",
            Token::MulEq => "*=",
            Token::BitXorEq => "^=",
            Token::BitOrEq => "|=",
            Token::BitAndEq => "&=",

            Token::Id(_id) => "",
            Token::NumericalLiteral(_num) => "",
        }
    }
}

impl From<Token> for String {
    fn from(value: Token) -> Self {
        match value {
            Token::Let => "let",
            Token::CmpEq => r"==",
            Token::Eq => r"=",
            Token::Semi => ";",

            Token::Plus => "+",
            Token::Minus => "-",
            Token::ForwardSlash => "/",
            Token::Star => "*",

            Token::BitXor => "^",
            Token::BitOr => "|",
            Token::BitAnd => "&",

            Token::PlusEq => "+=",
            Token::MinusEq => "-=",
            Token::DivEq => "=",
            Token::MulEq => "*=",
            Token::BitXorEq => "^=",
            Token::BitOrEq => "|=",
            Token::BitAndEq => "&=",

            Token::Id(_c) => "",
            Token::NumericalLiteral(_c) => "",
        }
        .to_string()
    }
}

impl TryFrom<String> for Token {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let token = match value.as_str() {
            "let" => Token::Let,
            "==" => Token::CmpEq,
            "=" => Token::Eq,
            ";" => Token::Semi,

            "+" => Token::Plus,
            "-" => Token::Minus,
            "/" => Token::ForwardSlash,
            "*" => Token::Star,

            "^" => Token::BitXor,
            "|" => Token::BitOr,
            "&" => Token::BitAnd,

            "+=" => Token::PlusEq,
            "-=" => Token::MinusEq,
            "/=" => Token::DivEq,
            "*=" => Token::MulEq,
            "^=" => Token::BitXorEq,
            "|=" => Token::BitOrEq,
            "&=" => Token::BitAndEq,

            num if value.chars().all(|c| c.is_numeric() || c == '-') => {
                // TODO Don't unwrap
                Token::NumericalLiteral(num.parse().unwrap())
            }
            id if is_valid_identifier(id) => Token::Id(id.to_string()),
            unknown => return Err(unknown.to_string()),
        };

        Ok(token)
    }
}

pub fn is_valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();

    match chars.nth(0) {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        Some(_) => return false,
        None => return false,
    }
    chars.all(|c| c.is_alphanumeric() || c == '_')
}
