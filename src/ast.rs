#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub return_type: Type,
    pub name: Identifier,
    pub parameter: Option<Parameter>,
    pub variables: Vec<Variable>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub kind: Type,
    pub identifier: Identifier,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub kind: Type,
    pub identifier: Identifier,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment {
        lhs: Identifier,
        rhs: Exp,
    },
    If {
        condition: RelExp,
        if_part: Box<Statement>,
        else_part: Option<Box<Statement>>,
    },
    Return {
        value: Exp,
    },
    Compound(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub struct RelExp {
    pub lhs: Exp,
    pub operator: Relop,
    pub rhs: Exp,
}

#[derive(Debug, Clone)]
pub enum Exp {
    Literal(Literal),
    Identifier(Identifier),
    FunctionCall {
        name: Identifier,
        arg: Option<Box<Exp>>,
    },
    Binary(Box<Exp>, Arop, Box<Exp>),
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Int,
    Unsigned,
}

#[derive(Debug, Clone, Copy)]
pub enum Arop {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
pub enum Relop {
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    IntNum(i32),
    UintNum(u32),
}
