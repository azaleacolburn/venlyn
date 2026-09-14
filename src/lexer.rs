use crate::token::Token;
use regex::Regex;
use strum::IntoEnumIterator;

enum TokenizeResult {
    Match { matched: Token, len: usize },
    NoMatch,
}

fn tokenize(string: &str, idx: usize) -> TokenizeResult {
    for token in Token::iter() {
        let regex: Regex = token.clone().into();

        if let Some(needle) = regex.find(&string[idx..]) {
            return TokenizeResult::Match {
                matched: token,
                len: needle.len(),
            };
        }
    }

    TokenizeResult::NoMatch
}

/*
* Parses as ASCII string into a series of tokens
*/
pub fn lex(string: &str) -> Vec<Token> {
    let mut idx = 0;
    let indexable = string.as_bytes();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(c) = indexable.get(idx)
        && c.is_ascii_whitespace()
    {
        idx += 1;
    }

    if indexable.get(idx).is_none() {
        return tokens;
    }

    while let TokenizeResult::Match { matched, len } = tokenize(string, idx) {
        tokens.push(matched);
        idx += len;

        while let Some(c) = indexable.get(idx)
            && c.is_ascii_whitespace()
        {
            idx += 1;
        }

        if indexable.get(idx).is_none() {
            break;
        }
    }

    return tokens;
}
