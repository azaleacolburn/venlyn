use std::collections::HashMap;

use crate::compiler::lexer::LexNode;

pub mod lexer;
pub mod token;

pub fn lex_test() {
    // TODO
    // Right now, there isn't a way to construct '+' and '+= and requential tokens'
    let plus_tokens = vec!["="];
    let plus_base = LexNode::trunk("+").with_children_leaf(&plus_tokens);

    let equal_tokens = vec!["="];
    let equal_base = LexNode::trunk("=").with_children_leaf(&equal_tokens);

    let base_tokens = vec!["let", ";"];
    let base = LexNode::root()
        .with_children_leaf(&base_tokens)
        .with_branch(plus_base)
        .with_branch(equal_base);

    let code = "let=;";
    lexer::tokenize(code, &base);
}
