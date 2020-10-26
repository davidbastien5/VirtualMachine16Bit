use crate::assembler::parser::ast;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take},
    character::complete::{alpha1, alphanumeric1, char, hex_digit1},
    combinator::{map, map_parser, map_res, value},
    multi::fold_many0,
    sequence::{pair, preceded},
    IResult,
};
use std::num::ParseIntError;

pub fn hex_literal(input: &str) -> IResult<&str, ast::Expr> {
    preceded(
        char('$'),
        map_res(hex_digit1, |input| -> Result<ast::Expr, ParseIntError> {
            let hex_lit = u16::from_str_radix(input, 16)?;

            Ok(ast::Expr {
                kind: ast::ExprKind::HexLiteral(hex_lit),
            })
        }),
    )(input)
}

fn identifier(input: &str) -> IResult<&str, String> {
    let one_alpha = map_parser(take(1usize), alpha1);

    map(
        pair(
            alt((one_alpha, tag("_"))),
            fold_many0(
                alt((alphanumeric1, tag("_"))),
                String::new(),
                |mut accumulator, item| {
                    accumulator.push_str(item);
                    accumulator
                },
            ),
        ),
        |(first, second): (&str, String)| format!("{}{}", first, second),
    )(input)
}

pub fn register(input: &str) -> IResult<&str, ast::Register> {
    alt((
        value(ast::Register::R1, tag_no_case("r1")),
        value(ast::Register::R2, tag_no_case("r2")),
        value(ast::Register::R3, tag_no_case("r3")),
        value(ast::Register::R4, tag_no_case("r4")),
        value(ast::Register::R5, tag_no_case("r5")),
        value(ast::Register::R6, tag_no_case("r6")),
        value(ast::Register::R7, tag_no_case("r7")),
        value(ast::Register::R8, tag_no_case("r8")),
        value(ast::Register::Sp, tag_no_case("sp")),
        value(ast::Register::Fp, tag_no_case("fp")),
        value(ast::Register::Ip, tag_no_case("ip")),
        value(ast::Register::Acc, tag_no_case("acc")),
    ))(input)
}

pub fn variable(input: &str) -> IResult<&str, ast::Expr> {
    map(preceded(tag("!"), identifier), |identifier| ast::Expr {
        kind: ast::ExprKind::Variable(identifier),
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, Err::Error};

    #[test]
    fn hex_literal_test() {
        assert_eq!(
            hex_literal("$1234"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::HexLiteral(0x1234)
                }
            ))
        );
        assert_eq!(
            hex_literal("$0"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::HexLiteral(0x00)
                }
            ))
        );
        assert_eq!(
            hex_literal("$89"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::HexLiteral(0x89)
                }
            ))
        );
    }

    #[test]
    fn identifier_test() {
        assert_eq!(identifier("_"), Ok(("", String::from("_"))));
        assert_eq!(identifier("d"), Ok(("", String::from("d"))));
        assert_eq!(identifier("wad1_23"), Ok(("", String::from("wad1_23"))));
        assert_eq!(identifier("_12fe2_"), Ok(("", String::from("_12fe2_"))));
        assert_eq!(identifier("9"), Err(Error(("9", ErrorKind::Tag))));
        assert_eq!(identifier(" "), Err(Error((" ", ErrorKind::Tag))));
        assert_eq!(identifier(""), Err(Error(("", ErrorKind::Tag))));
    }

    #[test]
    fn register_test() {
        assert_eq!(register("R1"), Ok(("", ast::Register::R1)));
        assert_eq!(register("r4"), Ok(("", ast::Register::R4)));
        assert_eq!(register("aCc"), Ok(("", ast::Register::Acc)));
    }

    #[test]
    fn variable_test() {
        assert_eq!(
            variable("!abc"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Variable(String::from("abc"))
                }
            ))
        );
        assert_eq!(
            variable("!_"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Variable(String::from("_"))
                }
            ))
        );
        assert_eq!(
            variable("!ab1_cd2"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Variable(String::from("ab1_cd2"))
                }
            ))
        );
        assert_eq!(variable("abc"), Err(Error(("abc", ErrorKind::Tag))));
    }
}
