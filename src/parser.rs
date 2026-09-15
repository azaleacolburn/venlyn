use crate::{
    ast::{ArgNode, BaseNode, Block, Expr, ProgramNode, StatementNode},
    token::Token,
};
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
    pub fn parse(&mut self) -> Result<ProgramNode> {
        let mut base_nodes = Vec::new();

        while let Some(tok) = self.get() {
            let base_node = match tok {
                Token::Fn => self.parse_function(),
                Token::Let => self.parse_declaration(),
            };

            base_nodes.push(base_node);
        }

        if base_nodes.is_empty() {
            bail!("Empty program");
        }

        Ok(ProgramNode { base_nodes })
    }

    fn parse_function(&mut self) -> Result<BaseNode> {
        assert_eq!(self.get(), Some(Token::Fn).as_ref());

        let Some(Token::Id(id)) = self.next() else {
            bail!("Expected Id");
        };

        let args = self.parse_arguments()?;

        let body = self.parse_block()?;

        Ok(BaseNode::FunctionDeclaration {
            id: id.clone(),
            args,
            body,
        })
    }

    fn parse_arguments(&mut self) -> Result<Vec<ArgNode>> {
        let Some(Token::LeftParen) = self.next() else {
            bail!("Expected '('");
        };

        let mut args = Vec::new();

        while let Some(next) = self.next().cloned()
            && next != Token::RightParen
        {
            let Token::Id(id) = next else {
                bail!("Expected Id");
            };

            if self.next() != Some(Token::Colon).as_ref() {
                bail!("Expected ':'");
            };

            let Some(Token::Type(r#type)) = self.next() else {
                bail!("Expected Type");
            };

            args.push(ArgNode {
                id: id.clone(),
                r#type: r#type.clone(),
            });
        }

        let Some(Token::RightParen) = self.next() else {
            bail!("Expected ')'");
        };

        Ok(args)
    }

    fn parse_block(&mut self) -> Result<Block> {
        if self.next() != Some(&Token::LeftBracket) {
            bail!("Expected '{{'");
        };

        let mut statements = Vec::new();

        while let Some(tok) = self.next().cloned()
            && tok != Token::RightBracket
        {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    /// First node of the statement should be the current node
    fn parse_statement(&mut self) -> Result<StatementNode> {
        let Some(tok) = self.get().cloned() else {
            bail!("Expected Statement");
        };

        match tok {
            Token::Let => self.parse_declaration(),
            Token::Id(_) => self.parse_assignment(),
            _ => bail!("Invalid Statement token"),
        }
    }

    fn parse_declaration(&mut self) -> Result<StatementNode> {
        assert_eq!(self.get(), Some(&Token::Let));

        let Some(Token::Id(id)) = self.next().cloned() else {
            bail!("Expected Id");
        };

        let r#type = if self.next().cloned() == Some(Token::Colon) {
            let Some(Token::Type(t)) = self.next().cloned() else {
                bail!("Expected Type");
            };

            self.next();

            Some(t)
        } else {
            None
        };

        let expr: Option<Expr> = if self.get() == Some(&Token::Eq) {
            self.next();
            Some(self.parse_expr()?)
        } else {
            None
        };

        if self.get().cloned() != Some(Token::Semi) {
            bail!("Expected ';'");
        }

        self.next();

        Ok(StatementNode::Declaration {
            id: id.clone(),
            r#type,
            expr,
        })
    }

    /// The current token should be the id
    fn parse_assignment(&mut self, id: String) -> Result<StatementNode> {
        if self.next().cloned() != Some(Token::Eq) {
            bail!("Expected Eq");
        }
    }

    fn parse_expr(&mut self) -> Result<Expr> {}

    fn get(&self) -> Option<&Token> {
        self.tokens.get(self.idx)
    }

    /// Gets the next token in the list, incrementing the counter
    /// Subsequently calling `self.get` should return the same reference
    fn next(&mut self) -> Option<&Token> {
        self.idx += 1;
        self.get()
    }
}
