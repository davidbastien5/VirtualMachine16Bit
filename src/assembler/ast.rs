#[derive(Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind
}

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    Binary(Box<Expr>, Operator, Box<Expr>),
    Bracket(Box<Expr>),
    HexLiteral(u16),
    SquareBracket(Box<Expr>),
    Variable(Box<Variable>)
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub kind: InstructionKind,
}

#[derive(Debug, PartialEq)]
pub enum InstructionKind {
    MovLitReg(Expr, Register),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    OpPlus,
    OpMinus,
    OpMultiply
}

#[derive(Clone, Debug, PartialEq)]
pub enum Register {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    Sp,
    Fp,
    Ip,
    Acc,
}

#[derive(Debug, PartialEq)]
pub struct Variable(pub String);
