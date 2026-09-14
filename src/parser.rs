use crate::token::Token;
use anyhow::{Result, bail};

pub enum Scope {
    Global,
    Function,
    Loop,
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    idx: usize,
    scope: Scope,
}

impl<'a> Parser<'a> {
    pub fn parse(&self) -> Result<ProgramNode> {
        let Some(tok) = self.get() else {
            bail!("Empty Program");
        };

        match self.get(). {
            Token::Fn => self.parse_function(),
            Token::Fn => self.parse_function(),
        }
    }

    fn parse_function(&self) {}

    fn get(&self) -> Option<&Token> {
        self.tokens.get(self.idx)
    }
}
