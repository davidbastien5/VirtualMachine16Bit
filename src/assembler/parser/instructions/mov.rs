use crate::assembler::parser::{ast, expressions, types};
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case},
    character::complete::{char, space0, space1},
    combinator::map,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

pub fn mov_lit_reg(input: &str) -> IResult<&str, ast::Instruction> {
    fn element(input: &str) -> IResult<&str, ast::Expr> {
        alt((expressions::square_braket_expr, types::hex_literal))(input)
    }

    fn space_delimited_comma(input: &str) -> IResult<&str, char> {
        delimited(space0, char(','), space0)(input)
    }

    map(
        delimited(
            tuple((tag_no_case("mov"), space1)),
            separated_pair(element, space_delimited_comma, types::register),
            space0,
        ),
        |(literal, register)| ast::Instruction {
            kind: ast::InstructionKind::MovLitReg(literal, register),
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mov_lit_to_reg_test() {
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
}
