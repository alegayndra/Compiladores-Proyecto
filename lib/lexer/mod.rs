use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while},
    combinator::value,
    multi::many0,
    IResult,
    sequence::tuple,
};

pub mod operadores;

use crate::lexer::operadores::*;

pub fn tipo_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("entero"), tag("flotante"), tag("char")))(input)
}

pub fn arit(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((sumsub_parser, multdiv_parser))(input)
}

pub fn id(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric())(input)
}

pub fn ws(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c == ' ')(input)
}

pub fn necessary_ws(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == ' ')(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err,
    };

    #[test]
    fn test_ws() {
        assert_eq!(ws(""), Ok(("", "")));
        assert_eq!(ws("  "), Ok(("", "  ")));
        assert_eq!(ws("a"), Ok(("a", "")));
    }

    #[test]
    fn test_necessary_ws() {
        assert_eq!(necessary_ws("  "), Ok(("", "  ")));
        assert_eq!(necessary_ws(" "), Ok(("", " ")));
        assert_eq!(necessary_ws(" a"), Ok(("a", " ")));

    }
}
