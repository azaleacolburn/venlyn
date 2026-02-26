use crate::lexer::token::Token;

pub struct GrammarTree {
    token: Option,
    children: Vec<GrammarTree>,
}

fn parse(token: Vec<Token>, grammar: GrammarTree) -> Ast {}
