use crate::compiler::lex_tree::LexNode;

pub mod char_iter;
pub mod lex_tree;
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

    let code = "let this=4;let that = 5; let somet hing      = 1 == 4;";
    let tokens = lexer::tokenize(code, &base);
    println!("{:?}", tokens)
}
