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

pub fn unsigned_number(i: &str) -> IResult<&str, u64> {
    let (input, digits) = digit1(i)?;
    Ok((input, digits.parse().unwrap()))
}

pub fn opt_signed_number(i: &str) -> IResult<&str, i64> {
    let (input, (sign, result)) = tuple((opt_sign, unsigned_number))(i)?;
    let mut result = result as i64;
    if !sign {
        result = -result;
    };
    Ok((input, result))
}
