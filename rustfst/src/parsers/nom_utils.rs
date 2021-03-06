use nom::bytes::complete::take_while;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

use nom::error::ErrorKind;
use nom::error::ParseError;

#[derive(Debug, PartialEq)]
pub enum NomCustomError<I> {
    SymbolTableError(String),
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for NomCustomError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        NomCustomError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn num(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(i)
}

pub fn word(i: &str) -> IResult<&str, String> {
    let (i, letters) = take_while(|c: char| (c != ' ') && (c != '\t') && (c != '\n'))(i)?;
    Ok((i, letters.to_string()))
}
