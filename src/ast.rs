use strum_macros::{Display, EnumString};

pub type Block = Vec<StatementNode>;

#[derive(Debug, Clone, PartialEq, Default, Display)]
pub enum Type {
    U8,
    U16,
    #[default]
    U32,
    U64,

    I8,
    I16,
    I32,
    I64,

    Usize,

    Struct(String),
}

pub struct ArgNode {
    pub id: String,
    pub r#type: Type,
}

pub struct VarInfo {
    name: String,
}

pub struct ProgramNode {
    pub base_nodes: Vec<BaseNode>,
    // variables: Vec<VarInfo>,
    // structs: Vec<String>,
}

pub enum BaseNode {
    Statement(ConstStatementNode),
    FunctionDeclaration {
        id: String,
        args: Vec<ArgNode>,
        body: Block,
    },
    StructDeclaration {
        id: String,
        fields: Vec<ArgNode>,
    },
}

pub enum ConstStatementNode {
    Declaration {
        id: String,
        r#type: Option<Type>,
        expr: Option<Expr>,
    },
}

pub enum StatementNode {
    Declaration {
        id: String,
        r#type: Option<Type>,
        expr: Option<Expr>,
    },
    Assignment {
        id: String,
        expr: Expr,
    },
    If {
        condition: Expr,
        body: Block,
    },
    Loop {
        body: Block,
    },
    Break,
    Expression(Expr),
}

pub enum Expr {
    Or((Term, Term)),
    And((Term, Term)),
    Term(Term),
}

pub enum Term {
    Eq((ArithExpr, ArithExpr)),
    NEq((ArithExpr, ArithExpr)),
    Expr(ArithExpr),
}

pub enum ArithExpr {
    Mul((ArithTerm, ArithTerm)),
    Div((ArithTerm, ArithTerm)),
    Term(ArithTerm),
}

pub enum ArithTerm {
    Add((Factor, Factor)),
    Sub((Factor, Factor)),
    Factor(Factor),
}

pub enum Factor {
    IntLit(i32),
    FloatLit(f32),
    BoolLit(bool),
    Var(String),
    FunctionCall { name: String, args: Vec<Expr> },

    Expr(Box<Expr>),
}
