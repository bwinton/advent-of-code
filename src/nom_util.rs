use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::opt,
    sequence::tuple, IResult,
};

fn opt_sign(i: &str) -> IResult<&str, bool> {
    let (input, sign) = opt(alt((tag("-"), tag("+"))))(i)?;
    let result = if let Some(sign) = sign {
        sign == "+"
    } else {
        true
    };
    Ok((input, result))
}

pub fn opt_signed_number(i: &str) -> IResult<&str, i64> {
    let (input, (sign, digits)) = tuple((opt_sign, digit1))(i)?;
    let mut result: i64 = digits.parse().unwrap();
    if !sign {
        result = -result;
    };
    Ok((input, result))
}
