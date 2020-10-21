#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub kind: InstructionKind,
}

#[derive(Debug, PartialEq)]
pub enum InstructionKind {
    MovLitReg(u16, Register),
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
