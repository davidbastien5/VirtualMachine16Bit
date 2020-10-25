use crate::assembler::ast;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take},
    character::complete::{alpha1, alphanumeric1, char, hex_digit1, space0, space1},
    combinator::{map, map_parser, map_res, value},
    multi::fold_many0,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};
use std::num::ParseIntError;

fn bracketed_expr(input: &str) -> IResult<&str, ast::Expr> {
    fn element(input: &str) -> IResult<&str, ast::Expr> {
        alt((bracketed_expr, hex_literal, variable))(input)
    }

    fn operator_separated(input: &str) -> IResult<&str, ast::Expr> {
        alt((binary, element))(input)
    }

    map(
        delimited(
            tuple((char('('), space0)),
            operator_separated,
            tuple((space0, char(')'))),
        ),
        |expression| ast::Expr {
            kind: ast::ExprKind::Bracket(Box::new(expression)),
        },
    )(input)
}

fn binary(input: &str) -> IResult<&str, ast::Expr> {
    fn element(input: &str) -> IResult<&str, ast::Expr> {
        alt((bracketed_expr, hex_literal, variable))(input)
    }

    fn space_delimited_operator(input: &str) -> IResult<&str, ast::Operator> {
        delimited(space0, operator, space0)(input)
    }

    map(
        tuple((element, space_delimited_operator, alt((binary, element)))),
        |(expression1, operator, expression2)| ast::Expr {
            kind: ast::ExprKind::Binary(Box::new(expression1), operator, Box::new(expression2)),
        },
    )(input)
}

fn hex_literal(input: &str) -> IResult<&str, ast::Expr> {
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

fn mov_lit_reg(input: &str) -> IResult<&str, ast::Instruction> {
    map(
        delimited(
            tuple((tag_no_case("mov"), space1)),
            separated_pair(hex_literal, tuple((space0, char(','), space0)), register),
            space0,
        ),
        |(literal, register)| ast::Instruction {
            kind: ast::InstructionKind::MovLitReg(literal, register),
        },
    )(input)
}

fn operator(input: &str) -> IResult<&str, ast::Operator> {
    alt((
        value(ast::Operator::OpPlus, char('+')),
        value(ast::Operator::OpMinus, char('-')),
        value(ast::Operator::OpMultiply, char('*')),
    ))(input)
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
        value(ast::Register::Sp, tag_no_case("sp")),
        value(ast::Register::Fp, tag_no_case("fp")),
        value(ast::Register::Ip, tag_no_case("ip")),
        value(ast::Register::Acc, tag_no_case("acc")),
    ))(input)
}

fn square_braket_expr(input: &str) -> IResult<&str, ast::Expr> {
    fn element(input: &str) -> IResult<&str, ast::Expr> {
        alt((bracketed_expr, hex_literal, variable))(input)
    }

    fn operator_separated(input: &str) -> IResult<&str, ast::Expr> {
        alt((binary, element))(input)
    }

    map(
        delimited(
            tuple((char('['), space0)),
            operator_separated,
            tuple((space0, char(']'))),
        ),
        |expression| ast::Expr {
            kind: ast::ExprKind::SquareBracket(Box::new(expression)),
        },
    )(input)
}

fn variable(input: &str) -> IResult<&str, ast::Expr> {
    map(preceded(tag("!"), identifier), |identifier| ast::Expr {
        kind: ast::ExprKind::Variable(Box::new(ast::Variable(identifier))),
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, Err::Error};

    #[test]
    fn binary_test() {
        assert_eq!(
            binary("$12+ $34"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Binary(
                        Box::new(ast::Expr {
                            kind: ast::ExprKind::HexLiteral(0x12)
                        }),
                        ast::Operator::OpPlus,
                        Box::new(ast::Expr {
                            kind: ast::ExprKind::HexLiteral(0x34)
                        })
                    )
                }
            ))
        );
        assert_eq!(
            binary("!z-$0123"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Binary(
                        Box::new(ast::Expr {
                            kind: ast::ExprKind::Variable(Box::new(ast::Variable(String::from(
                                "z"
                            ))))
                        }),
                        ast::Operator::OpMinus,
                        Box::new(ast::Expr {
                            kind: ast::ExprKind::HexLiteral(0x0123)
                        })
                    )
                }
            ))
        );
        assert_eq!(
            binary("$1234*!abc + $23"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Binary(
                        Box::new(ast::Expr {
                            kind: ast::ExprKind::HexLiteral(0x1234)
                        }),
                        ast::Operator::OpMultiply,
                        Box::new(ast::Expr {
                            kind: ast::ExprKind::Binary(
                                Box::new(ast::Expr {
                                    kind: ast::ExprKind::Variable(Box::new(ast::Variable(
                                        String::from("abc")
                                    )))
                                }),
                                ast::Operator::OpPlus,
                                Box::new(ast::Expr {
                                    kind: ast::ExprKind::HexLiteral(0x23)
                                })
                            )
                        })
                    )
                }
            ))
        );
        assert_eq!(binary("!a"), Err(Error(("", ErrorKind::Char))));
        assert_eq!(binary("$01+-!cd"), Err(Error(("-!cd", ErrorKind::Tag))));
    }

    #[test]
    fn bracketed_expr_test() {
        assert_eq!(
            bracketed_expr("($01)"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Bracket(Box::new(ast::Expr {
                        kind: ast::ExprKind::HexLiteral(0x01)
                    }))
                }
            ))
        );
        assert_eq!(
            bracketed_expr("( !abc- $5678 )"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Bracket(Box::new(ast::Expr {
                        kind: ast::ExprKind::Binary(
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::Variable(Box::new(ast::Variable(
                                    String::from("abc")
                                )))
                            }),
                            ast::Operator::OpMinus,
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::HexLiteral(0x5678)
                            })
                        )
                    }))
                }
            ))
        );
        assert_eq!(
            bracketed_expr("( ($10 *!z ) +!dfg )"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Bracket(Box::new(ast::Expr {
                        kind: ast::ExprKind::Binary(
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::Bracket(Box::new(ast::Expr {
                                    kind: ast::ExprKind::Binary(
                                        Box::new(ast::Expr {
                                            kind: ast::ExprKind::HexLiteral(0x10)
                                        }),
                                        ast::Operator::OpMultiply,
                                        Box::new(ast::Expr {
                                            kind: ast::ExprKind::Variable(Box::new(ast::Variable(
                                                String::from("z")
                                            )))
                                        }),
                                    )
                                }))
                            }),
                            ast::Operator::OpPlus,
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::Variable(Box::new(ast::Variable(
                                    String::from("dfg")
                                )))
                            })
                        )
                    }))
                }
            ))
        );
        assert_eq!(
            bracketed_expr("(  !fg - ( $8 *(!a-!b) ) + $5)"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Bracket(Box::new(ast::Expr {
                        kind: ast::ExprKind::Binary(
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::Variable(Box::new(ast::Variable(
                                    String::from("fg")
                                )))
                            }),
                            ast::Operator::OpMinus,
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::Binary(
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::Bracket(Box::new(ast::Expr {
                                            kind: ast::ExprKind::Binary(
                                                Box::new(ast::Expr {
                                                    kind: ast::ExprKind::HexLiteral(0x8)
                                                }),
                                                ast::Operator::OpMultiply,
                                                Box::new(ast::Expr {
                                                    kind: ast::ExprKind::Bracket(Box::new(
                                                        ast::Expr {
                                                            kind: ast::ExprKind::Binary(
                                                                Box::new(ast::Expr {
                                                                    kind: ast::ExprKind::Variable(
                                                                        Box::new(ast::Variable(
                                                                            String::from("a")
                                                                        ))
                                                                    )
                                                                }),
                                                                ast::Operator::OpMinus,
                                                                Box::new(ast::Expr {
                                                                    kind: ast::ExprKind::Variable(
                                                                        Box::new(ast::Variable(
                                                                            String::from("b")
                                                                        ))
                                                                    )
                                                                })
                                                            )
                                                        }
                                                    ))
                                                })
                                            )
                                        }))
                                    }),
                                    ast::Operator::OpPlus,
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::HexLiteral(0x5)
                                    })
                                )
                            })
                        )
                    }))
                }
            ))
        )
    }

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
    }

    #[test]
    fn operator_test() {
        assert_eq!(operator("+"), Ok(("", ast::Operator::OpPlus)));
        assert_eq!(operator("-"), Ok(("", ast::Operator::OpMinus)));
        assert_eq!(operator("*"), Ok(("", ast::Operator::OpMultiply)));
    }

    #[test]
    fn register_test() {
        assert_eq!(register("R1"), Ok(("", ast::Register::R1)));
        assert_eq!(register("r4"), Ok(("", ast::Register::R4)));
        assert_eq!(register("aCc"), Ok(("", ast::Register::Acc)));
    }

    #[test]
    fn square_braket_expr_test() {
        assert_eq!(
            square_braket_expr("[ $01+ $02]"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::SquareBracket(Box::new(ast::Expr {
                        kind: ast::ExprKind::Binary(
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::HexLiteral(0x01)
                            }),
                            ast::Operator::OpPlus,
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::HexLiteral(0x02)
                            })
                        )
                    }))
                }
            ))
        );
        assert_eq!(
            square_braket_expr("[ !abc-$1234 ]"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::SquareBracket(Box::new(ast::Expr {
                        kind: ast::ExprKind::Binary(
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::Variable(Box::new(ast::Variable(
                                    String::from("abc")
                                )))
                            }),
                            ast::Operator::OpMinus,
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::HexLiteral(0x1234)
                            })
                        )
                    }))
                }
            ))
        );
        assert_eq!(
            square_braket_expr("[$9876*!zyx + $43 ]"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::SquareBracket(Box::new(ast::Expr {
                        kind: ast::ExprKind::Binary(
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::HexLiteral(0x9876)
                            }),
                            ast::Operator::OpMultiply,
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::Binary(
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::Variable(Box::new(ast::Variable(
                                            String::from("zyx")
                                        )))
                                    }),
                                    ast::Operator::OpPlus,
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::HexLiteral(0x43)
                                    })
                                )
                            })
                        )
                    }))
                }
            ))
        );
        assert_eq!(
            square_braket_expr("[*$01]"),
            Err(Error(("*$01]", ErrorKind::Tag)))
        );
        assert_eq!(
            square_braket_expr("[!ab +$02- ]"),
            Err(Error(("- ]", ErrorKind::Char)))
        );
        assert_eq!(
            square_braket_expr("[ $01+-!cd]"),
            Err(Error(("+-!cd]", ErrorKind::Char)))
        );
    }

    #[test]
    fn variable_test() {
        assert_eq!(
            variable("!abc"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Variable(Box::new(ast::Variable(String::from("abc"))))
                }
            ))
        );
        assert_eq!(
            variable("!_"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Variable(Box::new(ast::Variable(String::from("_"))))
                }
            ))
        );
        assert_eq!(
            variable("!ab1_cd2"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Variable(Box::new(ast::Variable(String::from("ab1_cd2"))))
                }
            ))
        );
        assert_eq!(variable("abc"), Err(Error(("abc", ErrorKind::Tag))));
    }
}
