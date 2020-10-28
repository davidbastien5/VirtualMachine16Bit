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
    AddLitReg(Expr, Register),
    AddRegReg(Register, Register),
    AndLitReg(Expr, Register),
    AndRegReg(Register, Register),
    CalLit(Expr),
    CalReg(Register),
    DecReg(Register),
    Hlt,
    IncReg(Register),
    JeqLitMem(Expr, Expr),
    JeqRegMem(Register, Expr),
    JgeLitMem(Expr, Expr),
    JgeRegMem(Register, Expr),
    JgtLitMem(Expr, Expr),
    JgtRegMem(Register, Expr),
    JleLitMem(Expr, Expr),
    JleRegMem(Register, Expr),
    JltLitMem(Expr, Expr),
    JltRegMem(Register, Expr),
    JneLitMem(Expr, Expr),
    JneRegMem(Register, Expr),
    LsfRegLit(Register, Expr),
    LsfRegReg(Register, Register),
    MovLitMem(Expr, Expr),
    MovLitOffsetReg(Expr, Register, Register),
    MovLitReg(Expr, Register),
    MovMemReg(Expr, Register),
    MovRegMem(Register, Expr),
    MovRegReg(Register, Register),
    MovRegPtrReg(Register, Register),
    MulLitReg(Expr, Register),
    MulRegReg(Register, Register),
    NotReg(Register),
    OrLitReg(Expr, Register),
    OrRegReg(Register, Register),
    PopReg(Register),
    PshLit(Expr),
    PshReg(Register),
    Ret,
    RsfRegLit(Register, Expr),
    RsfRegReg(Register, Register),
    SubLitReg(Expr, Register),
    SubRegLit(Register, Expr),
    SubRegReg(Register, Register),
    XorLitReg(Expr, Register),
    XorRegReg(Register, Register),
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
