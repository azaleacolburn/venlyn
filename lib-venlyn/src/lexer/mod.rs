use crate::lexer::{lex_tree::LexNode, token::Token};

pub mod char_iter;
pub mod lex_tree;
#[cfg(test)]
pub mod test;
pub mod token;
pub mod tokenizer;

fn construct_lex_tree() -> LexNode {
    let plus_tokens = vec!["="];
    let plus_base = LexNode::trunk("+").with_children_leaf(&plus_tokens);

    let equal_tokens = vec!["="];
    let equal_base = LexNode::trunk("=").with_children_leaf(&equal_tokens);

    let base_tokens = vec!["let", ";"];
    let base = LexNode::root()
        .with_children_leaf(&base_tokens)
        .with_branch(plus_base)
        .with_branch(equal_base);

    base
}

pub fn lex(code: impl ToString) -> Vec<Token> {
    let tree = construct_lex_tree();

    tokenizer::tokenize(&code.to_string(), &tree)
}
