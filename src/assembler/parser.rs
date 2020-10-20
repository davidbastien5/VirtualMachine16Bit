use crate::assembler::ast;
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, hex_digit1, space0, space1},
    combinator::{map, map_res, value},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn hex_literal(input: &str) -> IResult<&str, u16> {
    preceded(
        char('$'),
        map_res(hex_digit1, |input| u16::from_str_radix(input, 16)),
    )(input)
}

fn register(input: &str) -> IResult<&str, ast::Register> {
    alt((
        value(ast::Register::R1, tag_no_case("r1")),
        value(ast::Register::R2, tag_no_case("r2")),
        value(ast::Register::R3, tag_no_case("r3")),
        value(ast::Register::R4, tag_no_case("r4")),
        value(ast::Register::R5, tag_no_case("r5")),
        value(ast::Register::R6, tag_no_case("r6")),
        value(ast::Register::R7, tag_no_case("r7")),
        value(ast::Register::R8, tag_no_case("r8")),
        value(ast::Register::SP, tag_no_case("sp")),
        value(ast::Register::FP, tag_no_case("fp")),
        value(ast::Register::IP, tag_no_case("ip")),
        value(ast::Register::ACC, tag_no_case("acc")),
    ))(input)
}

fn mov_lit_reg(input: &str) -> IResult<&str, ast::Instruction> {
    map(
        delimited(
            tuple((tag_no_case("mov"), space1)),
            separated_pair(hex_literal, tuple((space0, char(','), space0)), register),
            space0
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
    fn hex_literal_test() {
        assert_eq!(hex_literal("$1234"), Ok(("", 0x1234)));
        assert_eq!(hex_literal("$0"), Ok(("", 0x00)));
        assert_eq!(hex_literal("$89"), Ok(("", 0x89)));
    }

    #[test]
    fn mov_lit_to_reg_test() {
        assert_eq!(
            mov_lit_reg("mov $1234, R1"),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovLitReg(0x1234, ast::Register::R1)
                }
            ))
        );
        assert_eq!(
            mov_lit_reg("mOV $99,acc "),
            Ok((
                "",
                ast::Instruction {
                    kind: ast::InstructionKind::MovLitReg(0x99, ast::Register::ACC)
                }
            ))
        );
    }

    #[test]
    fn register_test() {
        assert_eq!(register("R1"), Ok(("", ast::Register::R1)));
        assert_eq!(register("r4"), Ok(("", ast::Register::R4)));
        assert_eq!(register("aCc"), Ok(("", ast::Register::ACC)));
    }
}
