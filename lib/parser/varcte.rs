use nom::{
    // branch::alt,
    // bytes::complete::{tag, tag_no_case, take},
    // character::complete::{alpha1, alphanumeric1, one_of},
    // combinator::opt,
    // error::{context, ErrorKind, VerboseError},
    error::{context, VerboseError},
    // multi::{count, many0, many1, many_m_n},
    // sequence::{preceded, separated_pair, terminated, tuple},
    // AsChar, Err as NomErr, IResult, InputTakeAtPosition,
    IResult
};

use crate::lexer::*;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct VARCTE<'a> {
    pub constante: &'a str,
}

pub fn varcte(input: &str) -> Res<&str, VARCTE> {
    context(
        "varcte",
        url_code_points,
    )(input)
    .map(|(next_input, res)| {
        let constante = res;
        (
            next_input,
            VARCTE {
                constante
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    // use nom::{
    //     error::{ErrorKind, VerboseError, VerboseErrorKind},
    //     Err as NomErr,
    // };

    #[test]
    fn test_varcte() {
        assert_eq!(
            varcte("aaaaa"),
            Ok(("", VARCTE { constante: "aaaaa" }))
        );

    }
}