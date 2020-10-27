use crate::assembler::parser::ast;
use nom::{branch::alt, IResult};

mod formats;

pub fn mov(input: &str) -> IResult<&str, ast::Instruction> {
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
