use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, hex_digit1, space0, space1},
    combinator::map_res,
    sequence::preceded,
    IResult,
};

#[derive(Debug, PartialEq)]
struct MovLitReg {
    literal: u16,
    register: String,
}

fn hex_literal(input: &str) -> IResult<&str, u16> {
    preceded(
        char('$'),
        map_res(hex_digit1, |input| u16::from_str_radix(input, 16)),
    )(input)
}

fn register(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("r1"),
        tag_no_case("r2"),
        tag_no_case("r3"),
        tag_no_case("r4"),
        tag_no_case("r5"),
        tag_no_case("r6"),
        tag_no_case("r7"),
        tag_no_case("r8"),
        tag_no_case("sp"),
        tag_no_case("fp"),
        tag_no_case("ip"),
        tag_no_case("acc"),
    ))(input)
}

fn mov_lit_to_reg(input: &str) -> IResult<&str, MovLitReg> {
    let (input, _) = tag_no_case("mov")(input)?;
    let (input, _) = space1(input)?;
    let (input, literal) = hex_literal(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space0(input)?;
    let (input, register) = register(input)?;
    let (input, _) = space0(input)?;

    Ok((
        input,
        MovLitReg {
            literal,
            register: register.to_string(),
        },
    ))
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
            mov_lit_to_reg("mov $1234, R1"),
            Ok((
                "",
                MovLitReg {
                    literal: 0x1234,
                    register: "R1".to_string()
                }
            ))
        );
        assert_eq!(
            mov_lit_to_reg("mOV $99,acc "),
            Ok((
                "",
                MovLitReg {
                    literal: 0x99,
                    register: "acc".to_string()
                }
            ))
        );
    }

    #[test]
    fn register_test() {
        assert_eq!(register("R1"), Ok(("", "R1")));
        assert_eq!(register("r4"), Ok(("", "r4")));
        assert_eq!(register("aCc"), Ok(("", "aCc")));
    }
}
