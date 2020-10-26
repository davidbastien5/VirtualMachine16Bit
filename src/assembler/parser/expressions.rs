use crate::assembler::parser::{ast, types};
use nom::{
    branch::alt,
    character::complete::{char, space0},
    combinator::{map, opt, value},
    sequence::{delimited, preceded, tuple},
    IResult,
};

pub fn address_expr(input: &str) -> IResult<&str, ast::Expr> {
    fn address_square_bracket_expr(input: &str) -> IResult<&str, ast::Expr> {
        preceded(char('&'), square_braket_expr)(input)
    }

    alt((types::address, address_square_bracket_expr))(input)
}

pub fn bracketed_expr(input: &str) -> IResult<&str, ast::Expr> {
    fn element(input: &str) -> IResult<&str, ast::Expr> {
        alt((bracketed_expr, types::hex_literal, types::variable))(input)
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

pub fn binary(input: &str) -> IResult<&str, ast::Expr> {
    fn plus_minus_operator(input: &str) -> IResult<&str, ast::Operator> {
        delimited(
            space0,
            alt((
                value(ast::Operator::OpPlus, char('+')),
                value(ast::Operator::OpMinus, char('-')),
            )),
            space0,
        )(input)
    }

    fn mult_operator(input: &str) -> IResult<&str, ast::Operator> {
        delimited(space0, value(ast::Operator::OpMultiply, char('*')), space0)(input)
    }

    fn factor(input: &str) -> IResult<&str, ast::Expr> {
        alt((bracketed_expr, types::hex_literal, types::variable))(input)
    }

    fn term(input: &str) -> IResult<&str, ast::Expr> {
        let (mut input, mut node) = factor(input)?;

        loop {
            match opt(tuple((mult_operator, factor)))(input)? {
                (remaining_input, Some((operator, right))) => {
                    input = remaining_input;
                    node = ast::Expr {
                        kind: ast::ExprKind::Binary(Box::new(node), operator, Box::new(right)),
                    };
                }
                _ => break,
            }
        }

        Ok((input, node))
    }

    fn expr(input: &str) -> IResult<&str, ast::Expr> {
        let (mut input, mut node) = term(input)?;

        loop {
            match opt(tuple((plus_minus_operator, term)))(input)? {
                (remaining_input, Some((operator, right))) => {
                    input = remaining_input;
                    node = ast::Expr {
                        kind: ast::ExprKind::Binary(Box::new(node), operator, Box::new(right)),
                    };
                }
                _ => break,
            }
        }

        Ok((input, node))
    }

    expr(input)
}

pub fn literal_expr(input: &str) -> IResult<&str, ast::Expr> {
    alt((square_braket_expr, types::hex_literal))(input)
}

pub fn square_braket_expr(input: &str) -> IResult<&str, ast::Expr> {
    fn element(input: &str) -> IResult<&str, ast::Expr> {
        alt((bracketed_expr, types::hex_literal, types::variable))(input)
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

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, Err::Error};

    #[test]
    fn address_expr_test() {
        assert_eq!(
            address_expr("&89"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Address(0x89)
                }
            ))
        );
        assert_eq!(
            address_expr("&[$12 * $34]"),
            Ok((
                "",
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
            ))
        )
    }

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
                            kind: ast::ExprKind::Variable(String::from("z"))
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
                            kind: ast::ExprKind::Binary(
                                Box::new(ast::Expr {
                                    kind: ast::ExprKind::HexLiteral(0x1234)
                                }),
                                ast::Operator::OpMultiply,
                                Box::new(ast::Expr {
                                    kind: ast::ExprKind::Variable(String::from("abc"))
                                })
                            )
                        }),
                        ast::Operator::OpPlus,
                        Box::new(ast::Expr {
                            kind: ast::ExprKind::HexLiteral(0x23)
                        })
                    )
                }
            ))
        );
        assert_eq!(
            binary("$12 + (!a - (!b)) - $34"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::Binary(
                        Box::new(ast::Expr {
                            kind: ast::ExprKind::Binary(
                                Box::new(ast::Expr {
                                    kind: ast::ExprKind::HexLiteral(0x12)
                                }),
                                ast::Operator::OpPlus,
                                Box::new(ast::Expr {
                                    kind: ast::ExprKind::Bracket(Box::new(ast::Expr {
                                        kind: ast::ExprKind::Binary(
                                            Box::new(ast::Expr {
                                                kind: ast::ExprKind::Variable(String::from("a"))
                                            }),
                                            ast::Operator::OpMinus,
                                            Box::new(ast::Expr {
                                                kind: ast::ExprKind::Bracket(Box::new(ast::Expr {
                                                    kind: ast::ExprKind::Variable(String::from(
                                                        "b"
                                                    ))
                                                }))
                                            }),
                                        )
                                    }))
                                })
                            )
                        }),
                        ast::Operator::OpMinus,
                        Box::new(ast::Expr {
                            kind: ast::ExprKind::HexLiteral(0x34)
                        }),
                    )
                }
            ))
        );
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
                                kind: ast::ExprKind::Variable(String::from("abc"))
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
                                            kind: ast::ExprKind::Variable(String::from("z"))
                                        }),
                                    )
                                }))
                            }),
                            ast::Operator::OpPlus,
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::Variable(String::from("dfg"))
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
                                kind: ast::ExprKind::Binary(
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::Variable(String::from("fg"))
                                    }),
                                    ast::Operator::OpMinus,
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
                                                                        String::from("a")
                                                                    )
                                                                }),
                                                                ast::Operator::OpMinus,
                                                                Box::new(ast::Expr {
                                                                    kind: ast::ExprKind::Variable(
                                                                        String::from("b")
                                                                    )
                                                                })
                                                            )
                                                        }
                                                    ))
                                                })
                                            )
                                        }))
                                    })
                                )
                            }),
                            ast::Operator::OpPlus,
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::HexLiteral(0x5)
                            })
                        )
                    }))
                }
            ))
        )
    }

    #[test]
    fn literal_expr_test() {
        assert_eq!(
            literal_expr("$12"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::HexLiteral(0x12)
                }
            ))
        );
        assert_eq!(
            literal_expr("[$12]"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::SquareBracket(Box::new(ast::Expr {
                        kind: ast::ExprKind::HexLiteral(0x12)
                    }))
                }
            ))
        );
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
                                kind: ast::ExprKind::Variable(String::from("abc"))
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
                                kind: ast::ExprKind::Binary(
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::HexLiteral(0x9876)
                                    }),
                                    ast::Operator::OpMultiply,
                                    Box::new(ast::Expr {
                                        kind: ast::ExprKind::Variable(String::from("zyx"))
                                    })
                                )
                            }),
                            ast::Operator::OpPlus,
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::HexLiteral(0x43)
                            })
                        )
                    }))
                }
            ))
        );
        assert_eq!(
            square_braket_expr("[!a + ($2 * $3)]"),
            Ok((
                "",
                ast::Expr {
                    kind: ast::ExprKind::SquareBracket(Box::new(ast::Expr {
                        kind: ast::ExprKind::Binary(
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::Variable(String::from("a"))
                            }),
                            ast::Operator::OpPlus,
                            Box::new(ast::Expr {
                                kind: ast::ExprKind::Bracket(Box::new(ast::Expr {
                                    kind: ast::ExprKind::Binary(
                                        Box::new(ast::Expr {
                                            kind: ast::ExprKind::HexLiteral(0x2)
                                        }),
                                        ast::Operator::OpMultiply,
                                        Box::new(ast::Expr {
                                            kind: ast::ExprKind::HexLiteral(0x3)
                                        }),
                                    )
                                }))
                            }),
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
}
