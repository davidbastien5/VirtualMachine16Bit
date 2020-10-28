use crate::assembler::parser::ast;
use nom::{branch::alt, IResult};

mod formats;

pub fn instruction(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        add,
        and,
        cal,
        dec,
        hlt,
        inc,
        jeq,
        jge,
        jgt,
        jle,
        jlt,
        jne,
        lsf,
        mov,
        mul,
        not,
        or,
        pop,
        psh,
        ret,
        alt((rsf, sub, xor)), // alt only accepts 21 elements maximum
    ))(input)
}

fn add(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit_reg(String::from("add"), |literal_expr, register| {
            ast::Instruction {
                kind: ast::InstructionKind::AddLitReg(literal_expr, register),
            }
        }),
        formats::reg_reg(String::from("add"), |register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::AddRegReg(register1, register2),
            }
        }),
    ))(input)
}

fn and(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit_reg(String::from("and"), |literal_expr, register| {
            ast::Instruction {
                kind: ast::InstructionKind::AndLitReg(literal_expr, register),
            }
        }),
        formats::reg_reg(String::from("and "), |register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::AndRegReg(register1, register2),
            }
        }),
    ))(input)
}

fn cal(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit(String::from("cal"), |literal_expr| ast::Instruction {
            kind: ast::InstructionKind::CalLit(literal_expr),
        }),
        formats::reg(String::from("cal"), |register| ast::Instruction {
            kind: ast::InstructionKind::CalReg(register),
        }),
    ))(input)
}

fn dec(input: &str) -> IResult<&str, ast::Instruction> {
    formats::reg(String::from("dec"), |register| ast::Instruction {
        kind: ast::InstructionKind::DecReg(register),
    })(input)
}

fn hlt(input: &str) -> IResult<&str, ast::Instruction> {
    formats::no_arg(String::from("hlt"), || ast::Instruction {
        kind: ast::InstructionKind::Hlt,
    })(input)
}

fn inc(input: &str) -> IResult<&str, ast::Instruction> {
    formats::reg(String::from("inc"), |register| ast::Instruction {
        kind: ast::InstructionKind::IncReg(register),
    })(input)
}

fn jeq(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::reg_mem(String::from("jeq"), |register, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JeqRegMem(register, address_expr),
            }
        }),
        formats::lit_mem(String::from("jeq"), |literal_expr, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JeqLitMem(literal_expr, address_expr),
            }
        }),
    ))(input)
}

fn jge(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::reg_mem(String::from("jge"), |register, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JgeRegMem(register, address_expr),
            }
        }),
        formats::lit_mem(String::from("jge"), |literal_expr, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JgeLitMem(literal_expr, address_expr),
            }
        }),
    ))(input)
}

fn jgt(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::reg_mem(String::from("jgt"), |register, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JgtRegMem(register, address_expr),
            }
        }),
        formats::lit_mem(String::from("jgt"), |literal_expr, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JgtLitMem(literal_expr, address_expr),
            }
        }),
    ))(input)
}

fn jle(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::reg_mem(String::from("jle"), |register, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JleRegMem(register, address_expr),
            }
        }),
        formats::lit_mem(String::from("jle"), |literal_expr, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JleLitMem(literal_expr, address_expr),
            }
        }),
    ))(input)
}

fn jlt(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::reg_mem(String::from("jlt"), |register, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JltRegMem(register, address_expr),
            }
        }),
        formats::lit_mem(String::from("jlt"), |literal_expr, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JltLitMem(literal_expr, address_expr),
            }
        }),
    ))(input)
}

fn jne(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::reg_mem(String::from("jne"), |register, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JneRegMem(register, address_expr),
            }
        }),
        formats::lit_mem(String::from("jne"), |literal_expr, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::JneLitMem(literal_expr, address_expr),
            }
        }),
    ))(input)
}

fn lsf(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit_reg(String::from("lsf"), |literal_expr, register| {
            ast::Instruction {
                kind: ast::InstructionKind::LsfLitReg(literal_expr, register),
            }
        }),
        formats::reg_reg(String::from("lsf"), |register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::LsfRegReg(register1, register2),
            }
        }),
    ))(input)
}

fn mov(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit_mem(String::from("mov"), |literal_expr, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::MovLitMem(literal_expr, address_expr),
            }
        }),
        formats::lit_offset_reg(String::from("mov"), |literal_expr, register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::MovLitOffsetReg(literal_expr, register1, register2),
            }
        }),
        formats::lit_reg(String::from("mov"), |literal_expr, register| {
            ast::Instruction {
                kind: ast::InstructionKind::MovLitReg(literal_expr, register),
            }
        }),
        formats::mem_reg(String::from("mov"), |address_expr, register| {
            ast::Instruction {
                kind: ast::InstructionKind::MovMemReg(address_expr, register),
            }
        }),
        formats::reg_mem(String::from("mov"), |register, address_expr| {
            ast::Instruction {
                kind: ast::InstructionKind::MovRegMem(register, address_expr),
            }
        }),
        formats::reg_reg(String::from("mov"), |register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::MovRegReg(register1, register2),
            }
        }),
        formats::reg_ptr_reg(String::from("mov"), |register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::MovRegPtrReg(register1, register2),
            }
        }),
    ))(input)
}

fn mul(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit_reg(String::from("mul"), |literal_expr, register| {
            ast::Instruction {
                kind: ast::InstructionKind::MulLitReg(literal_expr, register),
            }
        }),
        formats::reg_reg(String::from("mul"), |register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::MulRegReg(register1, register2),
            }
        }),
    ))(input)
}

fn not(input: &str) -> IResult<&str, ast::Instruction> {
    formats::reg(String::from("not"), |register| ast::Instruction {
        kind: ast::InstructionKind::NotReg(register),
    })(input)
}

fn or(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit_reg(String::from("or"), |literal_expr, register| {
            ast::Instruction {
                kind: ast::InstructionKind::OrLitReg(literal_expr, register),
            }
        }),
        formats::reg_reg(String::from("or"), |register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::OrRegReg(register1, register2),
            }
        }),
    ))(input)
}

fn pop(input: &str) -> IResult<&str, ast::Instruction> {
    formats::reg(String::from("pop"), |register| ast::Instruction {
        kind: ast::InstructionKind::PopReg(register),
    })(input)
}

fn psh(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit(String::from("psh"), |literal_expr| ast::Instruction {
            kind: ast::InstructionKind::PshLit(literal_expr),
        }),
        formats::reg(String::from("psh"), |register| ast::Instruction {
            kind: ast::InstructionKind::PshReg(register),
        }),
    ))(input)
}

fn ret(input: &str) -> IResult<&str, ast::Instruction> {
    formats::no_arg(String::from("ret"), || ast::Instruction {
        kind: ast::InstructionKind::Ret,
    })(input)
}

fn rsf(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit_reg(String::from("rsf"), |literal_expr, register| {
            ast::Instruction {
                kind: ast::InstructionKind::RsfLitReg(literal_expr, register),
            }
        }),
        formats::reg_reg(String::from("rsf"), |register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::RsfRegReg(register1, register2),
            }
        }),
    ))(input)
}

fn sub(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit_reg(String::from("sub"), |literal_expr, register| {
            ast::Instruction {
                kind: ast::InstructionKind::SubLitReg(literal_expr, register),
            }
        }),
        formats::reg_reg(String::from("sub"), |register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::SubRegReg(register1, register2),
            }
        }),
    ))(input)
}

fn xor(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        formats::lit_reg(String::from("xor"), |literal_expr, register| {
            ast::Instruction {
                kind: ast::InstructionKind::XorLitReg(literal_expr, register),
            }
        }),
        formats::reg_reg(String::from("xor"), |register1, register2| {
            ast::Instruction {
                kind: ast::InstructionKind::XorRegReg(register1, register2),
            }
        }),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mov_lit_mem_test() {
        assert_eq!(
            mov("mov $1, &2"),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovLitMem(
                        ast::Expr {
                            kind: ast::ExprKind::HexLiteral(0x1)
                        },
                        ast::Expr {
                            kind: ast::ExprKind::Address(0x2)
                        }
                    )
                }
            ))
        )
    }

    #[test]
    fn mov_lit_offset_reg_test() {
        assert_eq!(
            mov("mov [$12], &r3, r8"),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovLitOffsetReg(
                        ast::Expr {
                            kind: ast::ExprKind::SquareBracket(Box::new(ast::Expr {
                                kind: ast::ExprKind::HexLiteral(0x12)
                            }))
                        },
                        ast::Register::R3,
                        ast::Register::R8
                    )
                }
            ))
        );
    }

    #[test]
    fn mov_lit_reg_test() {
        assert_eq!(
            mov("mov $1234, R1"),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovLitReg(
                        ast::Expr {
                            kind: ast::ExprKind::HexLiteral(0x1234)
                        },
                        ast::Register::R1
                    )
                }
            ))
        );
        assert_eq!(
            mov("mOV $99,acc "),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovLitReg(
                        ast::Expr {
                            kind: ast::ExprKind::HexLiteral(0x99)
                        },
                        ast::Register::Acc
                    )
                }
            ))
        );
        assert_eq!(
            mov("mOV [!a - $4],acc "),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovLitReg(
                        ast::Expr {
                            kind: ast::ExprKind::SquareBracket(Box::new(ast::Expr {
                                kind: ast::ExprKind::Binary(
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::Variable(String::from("a"))
                                    }),
                                    ast::Operator::OpMinus,
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::HexLiteral(0x4)
                                    }),
                                )
                            }))
                        },
                        ast::Register::Acc
                    )
                }
            ))
        );
    }

    #[test]
    fn mov_mem_reg_test() {
        assert_eq!(
            mov("mov &89, ACC"),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovMemReg(
                        ast::Expr {
                            kind: ast::ExprKind::Address(0x89)
                        },
                        ast::Register::Acc,
                    )
                }
            ))
        );
    }

    #[test]
    fn mov_reg_mem_test() {
        assert_eq!(
            mov("mov R1, &[$12 * $34]"),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovRegMem(
                        ast::Register::R1,
                        ast::Expr {
                            kind: ast::ExprKind::SquareBracket(Box::new(ast::Expr {
                                kind: ast::ExprKind::Binary(
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::HexLiteral(0x12)
                                    }),
                                    ast::Operator::OpMultiply,
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::HexLiteral(0x34)
                                    })
                                )
                            }))
                        }
                    )
                }
            ))
        );
    }

    #[test]
    fn mov_reg_reg_test() {
        assert_eq!(
            mov("mov R1, r3"),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovRegReg(ast::Register::R1, ast::Register::R3)
                }
            ))
        );
    }

    #[test]
    fn mov_reg_ptr_reg_test() {
        assert_eq!(
            mov("mov &r8, r6"),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovRegPtrReg(ast::Register::R8, ast::Register::R6)
                }
            ))
        );
    }
}
