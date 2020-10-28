use crate::assembler::parser::{ast, expressions, types};
use nom::{
    bytes::complete::tag_no_case,
    character::complete::{char, space0, space1},
    combinator::map,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

fn space_delimited_comma(input: &str) -> IResult<&str, char> {
    delimited(space0, char(','), space0)(input)
}

pub fn lit<F>(mnemonic: String, mapper: F) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn(ast::Expr) -> ast::Instruction,
{
    move |input: &str| {
        map(
            delimited(
                tuple((tag_no_case(&mnemonic[..]), space1)),
                expressions::literal_expr,
                space0,
            ),
            |literal_expr| mapper(literal_expr),
        )(input)
    }
}

pub fn lit_mem<F>(mnemonic: String, mapper: F) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn(ast::Expr, ast::Expr) -> ast::Instruction,
{
    move |input: &str| {
        map(
            delimited(
                tuple((tag_no_case(&mnemonic[..]), space1)),
                separated_pair(
                    expressions::literal_expr,
                    space_delimited_comma,
                    expressions::address_expr,
                ),
                space0,
            ),
            |(literal_expr, address_expr)| mapper(literal_expr, address_expr),
        )(input)
    }
}

pub fn lit_offset_reg<F>(
    mnemonic: String,
    mapper: F,
) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn(ast::Expr, ast::Register, ast::Register) -> ast::Instruction,
{
    move |input: &str| {
        map(
            delimited(
                tuple((tag_no_case(&mnemonic[..]), space1)),
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
            |(literal_expr, (register1, register2))| mapper(literal_expr, register1, register2),
        )(input)
    }
}

pub fn lit_reg<F>(mnemonic: String, mapper: F) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn(ast::Expr, ast::Register) -> ast::Instruction,
{
    move |input: &str| {
        map(
            delimited(
                tuple((tag_no_case(&mnemonic[..]), space1)),
                separated_pair(
                    expressions::literal_expr,
                    space_delimited_comma,
                    types::register,
                ),
                space0,
            ),
            |(literal_expr, register)| mapper(literal_expr, register),
        )(input)
    }
}

pub fn mem_reg<F>(mnemonic: String, mapper: F) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn(ast::Expr, ast::Register) -> ast::Instruction,
{
    move |input: &str| {
        map(
            delimited(
                tuple((tag_no_case(&mnemonic[..]), space1)),
                separated_pair(
                    expressions::address_expr,
                    space_delimited_comma,
                    types::register,
                ),
                space0,
            ),
            |(address_expr, register)| mapper(address_expr, register),
        )(input)
    }
}

pub fn no_arg<F>(mnemonic: String, mapper: F) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn() -> ast::Instruction,
{
    move |input: &str| map(tuple((tag_no_case(&mnemonic[..]), space0)), |_| mapper())(input)
}

pub fn reg<F>(mnemonic: String, mapper: F) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn(ast::Register) -> ast::Instruction,
{
    move |input: &str| {
        map(
            delimited(
                tuple((tag_no_case(&mnemonic[..]), space1)),
                types::register,
                space0,
            ),
            |register| mapper(register),
        )(input)
    }
}

pub fn reg_lit<F>(mnemonic: String, mapper: F) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn(ast::Register, ast::Expr) -> ast::Instruction,
{
    move |input: &str| {
        map(
            delimited(
                tuple((tag_no_case(&mnemonic[..]), space1)),
                separated_pair(
                    types::register,
                    space_delimited_comma,
                    expressions::literal_expr,
                ),
                space0,
            ),
            |(register, literal_expr)| mapper(register, literal_expr),
        )(input)
    }
}

pub fn reg_mem<F>(mnemonic: String, mapper: F) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn(ast::Register, ast::Expr) -> ast::Instruction,
{
    move |input: &str| {
        map(
            delimited(
                tuple((tag_no_case(&mnemonic[..]), space1)),
                separated_pair(
                    types::register,
                    space_delimited_comma,
                    expressions::address_expr,
                ),
                space0,
            ),
            |(register, address_expr)| mapper(register, address_expr),
        )(input)
    }
}

pub fn reg_reg<F>(mnemonic: String, mapper: F) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn(ast::Register, ast::Register) -> ast::Instruction,
{
    move |input: &str| {
        map(
            delimited(
                tuple((tag_no_case(&mnemonic[..]), space1)),
                separated_pair(types::register, space_delimited_comma, types::register),
                space0,
            ),
            |(register1, register2)| mapper(register1, register2),
        )(input)
    }
}

pub fn reg_ptr_reg<F>(
    mnemonic: String,
    mapper: F,
) -> impl Fn(&str) -> IResult<&str, ast::Instruction>
where
    F: Fn(ast::Register, ast::Register) -> ast::Instruction,
{
    move |input: &str| {
        map(
            delimited(
                tuple((tag_no_case(&mnemonic[..]), space1)),
                separated_pair(
                    types::register_pointer,
                    space_delimited_comma,
                    types::register,
                ),
                space0,
            ),
            |(register1, register2)| mapper(register1, register2),
        )(input)
    }
}
