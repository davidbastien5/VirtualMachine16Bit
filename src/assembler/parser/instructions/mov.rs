use crate::assembler::parser::{ast, expressions, types};
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, space0, space1},
    combinator::map,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

pub fn mov(input: &str) -> IResult<&str, ast::Instruction> {
    alt((
        mov_lit_mem,
        mov_lit_offset_reg,
        mov_lit_reg,
        mov_mem_reg,
        mov_reg_mem,
        mov_reg_reg,
        mov_reg_ptr_reg,
    ))(input)
}

fn mov_lit_mem(input: &str) -> IResult<&str, ast::Instruction> {
    fn space_delimited_comma(input: &str) -> IResult<&str, char> {
        delimited(space0, char(','), space0)(input)
    }

    map(
        delimited(
            tuple((tag_no_case("mov"), space1)),
            separated_pair(
                expressions::literal_expr,
                space_delimited_comma,
                expressions::address_expr,
            ),
            space0,
        ),
        |(literal_expr, address_expr)| ast::Instruction {
            kind: ast::InstructionKind::MovLitMem(literal_expr, address_expr),
        },
    )(input)
}

fn mov_lit_offset_reg(input: &str) -> IResult<&str, ast::Instruction> {
    fn space_delimited_comma(input: &str) -> IResult<&str, char> {
        delimited(space0, char(','), space0)(input)
    }

    map(
        delimited(
            tuple((tag_no_case("mov"), space1)),
            separated_pair(
                expressions::literal_expr,
                space_delimited_comma,
                separated_pair(
                    types::register_pointer,
                    space_delimited_comma,
                    types::register,
                ),
            ),
            space0,
        ),
        |(literal_expr, (register1, register2))| ast::Instruction {
            kind: ast::InstructionKind::MovLitOffsetReg(literal_expr, register1, register2),
        },
    )(input)
}

fn mov_lit_reg(input: &str) -> IResult<&str, ast::Instruction> {
    fn space_delimited_comma(input: &str) -> IResult<&str, char> {
        delimited(space0, char(','), space0)(input)
    }

    map(
        delimited(
            tuple((tag_no_case("mov"), space1)),
            separated_pair(
                expressions::literal_expr,
                space_delimited_comma,
                types::register,
            ),
            space0,
        ),
        |(literal_expr, register)| ast::Instruction {
            kind: ast::InstructionKind::MovLitReg(literal_expr, register),
        },
    )(input)
}

fn mov_mem_reg(input: &str) -> IResult<&str, ast::Instruction> {
    fn space_delimited_comma(input: &str) -> IResult<&str, char> {
        delimited(space0, char(','), space0)(input)
    }

    map(
        delimited(
            tuple((tag_no_case("mov"), space1)),
            separated_pair(
                expressions::address_expr,
                space_delimited_comma,
                types::register,
            ),
            space0,
        ),
        |(address_expr, register)| ast::Instruction {
            kind: ast::InstructionKind::MovMemReg(address_expr, register),
        },
    )(input)
}

fn mov_reg_mem(input: &str) -> IResult<&str, ast::Instruction> {
    fn space_delimited_comma(input: &str) -> IResult<&str, char> {
        delimited(space0, char(','), space0)(input)
    }

    map(
        delimited(
            tuple((tag_no_case("mov"), space1)),
            separated_pair(
                types::register,
                space_delimited_comma,
                expressions::address_expr,
            ),
            space0,
        ),
        |(register, address_expr)| ast::Instruction {
            kind: ast::InstructionKind::MovRegMem(register, address_expr),
        },
    )(input)
}

fn mov_reg_reg(input: &str) -> IResult<&str, ast::Instruction> {
    fn space_delimited_comma(input: &str) -> IResult<&str, char> {
        delimited(space0, char(','), space0)(input)
    }

    map(
        delimited(
            tuple((tag_no_case("mov"), space1)),
            separated_pair(types::register, space_delimited_comma, types::register),
            space0,
        ),
        |(register1, register2)| ast::Instruction {
            kind: ast::InstructionKind::MovRegReg(register1, register2),
        },
    )(input)
}

fn mov_reg_ptr_reg(input: &str) -> IResult<&str, ast::Instruction> {
    fn space_delimited_comma(input: &str) -> IResult<&str, char> {
        delimited(space0, char(','), space0)(input)
    }

    map(
        delimited(
            tuple((tag_no_case("mov"), space1)),
            separated_pair(
                types::register_pointer,
                space_delimited_comma,
                types::register,
            ),
            space0,
        ),
        |(register1, register2)| ast::Instruction {
            kind: ast::InstructionKind::MovRegPtrReg(register1, register2),
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mov_lit_mem_test() {
        assert_eq!(
            mov_lit_mem("mov $1, &2"),
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
            mov_lit_offset_reg("mov [$12], &r3, r8"),
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
            mov_lit_reg("mov $1234, R1"),
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
            mov_lit_reg("mOV $99,acc "),
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
            mov_lit_reg("mOV [!a - $4],acc "),
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
            mov_mem_reg("mov &89, ACC"),
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
            mov_reg_mem("mov R1, &[$12 * $34]"),
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
            mov_reg_reg("mov R1, r3"),
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
            mov_reg_ptr_reg("mov &r8, r6"),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovRegPtrReg(ast::Register::R8, ast::Register::R6)
                }
            ))
        );
    }
}
