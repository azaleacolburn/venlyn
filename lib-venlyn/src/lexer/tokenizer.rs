use crate::lexer::{
    char_iter::CharIter,
    lex_tree::{IncompleteCharList, LexNode},
    token::Token,
};
use either::Either;

pub fn str_to_token_or_charlist(s: impl ToString) -> Either<Token, IncompleteCharList> {
    let token_res = s.to_string().try_into();
    match token_res {
        Ok(token) => Either::Left(token),
        Err(s) => Either::Right(s),
    }
}

pub fn token_or_charlist_to_str(token: &Either<Token, IncompleteCharList>) -> String {
    token.clone().right_or_else(|token| token.into())
}

pub fn tokenize(code: &str, base_tree: &LexNode) -> Vec<Token> {
    let mut tokens = Vec::with_capacity(code.len() / 3);
    let mut curr_code = CharIter::new(code.chars());

    while let Some(token) = lex_token(&mut curr_code, base_tree) {
        tokens.push(token);
    }

    tokens
}

fn lex_token<I: Iterator<Item = char>>(
    curr_code: &mut CharIter<I>,
    base_tree: &LexNode,
) -> Option<Token> {
    // Special cases for number and identifier parsing
    // TODO Find way to integrate this into the main LexTree system
    while let Some(c) = curr_code.peek()
        && c.is_whitespace()
    {
        curr_code.next();
    }
    // println!("first in new token {:?}", curr_code.peek());
    if let Some(token) = try_parse_number(curr_code) {
        return Some(token);
    }

    let mut current_tree: &LexNode = base_tree;
    // NOTE
    // Holds the valid nodes we've traversed to but could be a later one
    // For example:
    // in the case of parsing "+=", we first put `Token::Plus`, then `Token::PlusEq`, then break
    // and pop the latter when we find the "="
    let mut stack: Vec<Token> = Vec::with_capacity(3);
    // NOTE
    // This should loop once for each node on the path to the current token being lexed
    // So for most tokens, this should only run once
    'token_loop: loop {
        // println!("\n");
        if let Either::Left(token) = &current_tree.token {
            // println!("on stack");
            stack.push(token.clone());
        }
        // println!("parse_stack: {:?}", stack);

        if current_tree.children.is_none() {
            // println!("no children");
            break;
        }

        let children = current_tree.children.as_ref().unwrap();
        // println!("children: {:?}", children);
        let max_suffix_length = children
            .iter()
            .map(|child| child.0.len())
            .max()
            .unwrap_or_default();
        // println!("max_suffix_length {:?}", max_suffix_length);
        let mut current_suffix = String::with_capacity(3);

        // NOTE
        // This should loop once for each character in the node being lexed
        //
        // TODO
        // This system is kind of stupid because if you have a character that isn't the prefix for
        // any of the suffixes, you must check characters until `max_suffix_length` instead of
        // immediantly breaking
        loop {
            let c = match curr_code.next() {
                // This assumes that no token passes the whitespace boundary
                Some(c) if !c.is_whitespace() => c,
                _ => {
                    curr_code.push_str_front(&current_suffix);
                    break 'token_loop;
                }
            };

            current_suffix.push(c);
            // println!("current_suffix {:?}", current_suffix);
            if let Some(new_tree) = children.get(&current_suffix) {
                // println!("match {:?}", current_suffix);
                current_tree = new_tree;
                break;
            }

            if current_suffix.len() >= max_suffix_length {
                // println!("too big {:?}", current_suffix);
                curr_code.push_str_front(&current_suffix);
                break 'token_loop;
            }
        }
    }

    stack.pop().or_else(|| try_parse_identifier(curr_code))
}

fn try_parse_identifier<I: Iterator<Item = char>>(curr_code: &mut CharIter<I>) -> Option<Token> {
    let mut id = String::with_capacity(4);
    if let Some(c) = curr_code.next()
        && is_valid_id_start(c)
    {
        id.push(c);
    } else {
        // NOTE
        // At this point we've failed to parse all other tokens, so if it's not an id, we don't
        // care about maintaining the character iterator
        return None;
    }

    while let Some(c) = curr_code.next() {
        if !is_valid_id_body(c) {
            if !c.is_whitespace() {
                curr_code.push_front(c);
            }
            break;
        }

        id.push(c);
    }

    Some(Token::Id(id))
}

fn is_valid_id_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_valid_id_body(c: char) -> bool {
    is_valid_id_start(c) || c.is_numeric()
}

fn try_parse_number<I: Iterator<Item = char>>(curr_code: &mut CharIter<I>) -> Option<Token> {
    if let Some(c) = curr_code.peek() {
        if c.is_numeric() {
            return parse_num(curr_code, false);
        }

        if c == '-' {
            let _ = curr_code.next().unwrap();
            // TODO Look ahead without this hack
            let v = curr_code.collect::<Vec<char>>();
            match v.iter().peekable().peek() {
                Some(next) if next.is_numeric() => parse_num(curr_code, true),
                Some(next) => {
                    if !next.is_whitespace() {
                        curr_code.push_front(**next);
                    }
                    return None;
                }
                None => return None,
            };
        }
    }

    None
}

fn parse_num<I: Iterator<Item = char>>(curr_code: &mut CharIter<I>, neg: bool) -> Option<Token> {
    let mut num: i32 = 0;
    while let Some(c) = curr_code.next() {
        let digit: i32 = match c.to_digit(10) {
            Some(n) => n.try_into().unwrap(),
            None => {
                curr_code.push_front(c);
                break;
            }
        };
        num *= 10;
        num += digit
    }

    if !neg {
        return Some(Token::NumericalLiteral(num));
    } else {
        return Some(Token::NumericalLiteral(-num));
    }
}
