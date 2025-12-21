use std::collections::HashMap;

use either::Either;

use crate::compiler::{
    lexer::{str_to_token_or_charlist, token_or_charlist_to_str},
    token::Token,
};

// The idea is that the IncompleteCharList would always fail to be converted to a Token
pub type IncompleteCharList = String;

#[derive(Debug, Clone)]
pub struct LexNode {
    pub token: Either<Token, IncompleteCharList>,
    // TODO
    // IDEA:
    // Instead of having a string that we match against, we have a condition that the n next chars
    // have to meet
    // This would be:
    // - cool
    // - expensive
    // - difficult because n would sometimes be variable (actually maybe not)
    pub children: Option<HashMap<String, LexNode>>,
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
