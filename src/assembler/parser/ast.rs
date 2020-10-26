#[derive(Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    Address(u16),
    Binary(Box<Expr>, Operator, Box<Expr>),
    Bracket(Box<Expr>),
    HexLiteral(u16),
    SquareBracket(Box<Expr>),
    Variable(String),
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub kind: InstructionKind,
}

#[derive(Debug, PartialEq)]
pub enum InstructionKind {
    MovLitMem(Expr, Expr),
    MovLitOffsetReg(Expr, Register, Register),
    MovLitReg(Expr, Register),
    MovMemReg(Expr, Register),
    MovRegMem(Register, Expr),
    MovRegReg(Register, Register),
    MovRegPtrReg(Register, Register),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    OpPlus,
    OpMinus,
    OpMultiply,
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
