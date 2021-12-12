use nom::{character::complete::satisfy, IResult};

pub fn single_letter(i: &str) -> IResult<&str, char> {
    let (input, letter) = satisfy(|c| c.is_ascii_alphabetic())(i)?;
    Ok((input, letter))
}
