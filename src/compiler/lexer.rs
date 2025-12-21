use crate::compiler::token::Token;
use either::Either;
use regex::Regex;
use std::collections::HashMap;

macro_rules! unwrap_or_break {
    ($s:expr) => {
        match $s {
            Some(c) => c,
            None => break None,
        }
    };
}

// The idea is that the IncompleteCharList would always fail to be converted to a Token
type IncompleteCharList = String;

#[derive(Debug, Clone)]
pub struct LexNode {
    token: Either<Token, IncompleteCharList>,
    children: Option<HashMap<String, LexNode>>,
}

impl LexNode {
    pub fn root() -> Self {
        LexNode {
            token: Either::Right("".to_string()),
            children: None,
        }
    }

    pub fn trunk(token_str: impl ToString) -> Self {
        let token = Either::Left(
            token_str
                .to_string()
                .try_into()
                .expect(format!("Invalid base character: {:?}", token_str.to_string()).as_str()),
        );

        LexNode {
            token,
            children: None,
        }
    }

    pub fn with_children_leaf(mut self, children_suffixes: &[impl ToString + Clone]) -> Self {
        let prefix = match &self.token {
            Either::Left(token) => (*token).clone().try_into().expect("Invalid Token Found"),
            Either::Right(chars) => chars.clone(),
        };

        let children_suffixes: Vec<String> =
            children_suffixes.iter().map(|c| c.to_string()).collect();

        let full_strings = children_suffixes
            .iter()
            .map(|child| prefix.chars().chain(child.chars()).collect::<String>());

        let lex_trees = full_strings.map(|s| LexNode {
            token: str_to_token_or_charlist(s),
            children: None,
        });

        let suffix_and_nodes = children_suffixes.iter().cloned().zip(lex_trees).collect();
        self.children = Some(suffix_and_nodes);

        self
    }

    pub fn with_branch(mut self, child_node: LexNode) -> Self {
        let tok_str = token_or_charlist_to_str(&child_node.token);

        match self.children {
            Some(ref mut children) => {
                children.insert(tok_str, child_node);
            }
            None => {
                self.children = Some(HashMap::new());
                self.children.as_mut().unwrap().insert(tok_str, child_node);
            }
        };

        self
    }

    pub fn build() {}
}

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
    println!("here");
    let mut tokens = Vec::with_capacity(code.len() / 3);
    let mut curr_code = code.chars();

    while let Some(token) = lex_token(&mut curr_code, base_tree) {
        tokens.push(token);
    }

    tokens
}

fn lex_token(curr_code: &mut impl Iterator<Item = char>, base_tree: &LexNode) -> Option<Token> {
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
    let mut n = 0;
    while n < 4 {
        n += 1;
        println!("\n\n\n");
        println!("parse_stack: {:?}", stack);
        if let Either::Left(token) = &current_tree.token {
            println!("on stack");
            stack.push(token.clone());
        }

        if current_tree.children.is_none() {
            println!("no children");
            break;
        }

        let children = current_tree.children.as_ref().unwrap();
        println!("children: {:?}", children);
        let max_suffix_length = children
            .iter()
            .map(|child| token_or_charlist_to_str(&child.1.token).len())
            .max()
            .unwrap_or_default()
            - 1;
        println!("max_suffix_length {:?}", max_suffix_length);
        let mut current_suffix = String::with_capacity(3);

        // NOTE
        // This should loop once for each character in the node being lexed
        loop {
            let c = match curr_code.next() {
                Some(c) if c == ' ' => continue,
                Some(c) => c,
                None => break,
            };

            current_suffix.push(c);
            println!("current_suffix {:?}", current_suffix);
            if let Some(new_tree) = children.get(&current_suffix) {
                println!("match {:?}", new_tree);
                current_tree = new_tree;
                break;
            }

            // TODO Find way to put the current_suffix back if we don't find anything
            if current_suffix.len() >= max_suffix_length {
                break;
            }
        }
    }

    stack.pop()
}
