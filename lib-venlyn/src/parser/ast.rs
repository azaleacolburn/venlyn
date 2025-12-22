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

    USIZE,
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
        expr: Option<ExpressionNode>,
    },
    Assignment {
        id: String,
        expr: Option<ExpressionNode>,
    },
    If {
        condition: ConditionExpr,
        body: Block,
    },
    Loop {
        body: Block,
    },
    Break,
    Expression(ExpressionNode),
}

pub enum ConditionExpr {
    ConditionTerm(ConditionTerm),

    Operator(ConditionOperator),
}

pub enum ConditionTerm {
    Literal(bool),
    ConditionExpr(Box<ConditionExpr>),
}

pub enum ConditionOperator {
    Or,
    And,
}

pub enum ExpressionNode {}
