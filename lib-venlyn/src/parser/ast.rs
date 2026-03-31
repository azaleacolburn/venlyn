type Block = Vec<StatementNode>;

pub enum Type {
    U8,
    U16,
    U32,
    U64,

    I8,
    I16,
    I32,
    I64,

    Usize,
}

pub struct ArgNode {
    id: String,
    r#type: Type,
}

pub enum BaseNode {
    Statement(StatementNode),
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

pub enum StatementNode {
    Declaration {
        id: String,
        r#type: Type,
        expr: Option<CondExpr>,
    },
    Assignment {
        id: String,
        expr: CondExpr,
    },
    If {
        condition: CondExpr,
        body: Block,
    },
    Loop {
        body: Block,
    },
    Break,
    Expression(CondExpr),
}

pub enum CondExpr {
    Or((CondTerm, CondTerm)),
    And((CondTerm, CondTerm)),
}

pub enum CondTerm {
    Eq((CondFactor, CondFactor)),
    NEq((CondFactor, CondFactor)),
}

pub enum CondFactor {
    BoolLit(bool),
    Var(String),
    Expr(Box<CondExpr>),
}

pub enum ArithExpr {
    Mul((ArithTerm, ArithTerm)),
    Div((ArithTerm, ArithTerm)),
}

pub enum ArithTerm {
    Add((ArithFactor, ArithFactor)),
    Sub((ArithFactor, ArithFactor)),
}

pub enum ArithFactor {
    IntLit(i32),
    FloatLit(f32),
    Var(String),
    FunctionCall { name: String, args: Vec<CondExpr> },

    Expr(Box<CondExpr>),
}
