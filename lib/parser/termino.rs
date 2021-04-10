use nom::{
    // branch::alt,
    // bytes::complete::{tag, tag_no_case, take},
    // character::complete::{alpha1, alphanumeric1, one_of},
    // combinator::opt,
    // error::{context, ErrorKind, VerboseError},
    error::{context, VerboseError},
    // multi::{count, many0, many1, many_m_n},
    multi::many0,
    // sequence::{preceded, separated_pair, terminated, tuple},
    sequence::tuple,
    // AsChar, Err as NomErr, IResult, InputTakeAtPosition,
    IResult,
};

use crate::lexer::*;
use crate::parser::factor::*;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TERMINO<'a> {
    pub factor: FACTOR<'a>,
    pub termino2: Vec<TERMINO2<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TERMINO2<'a> {
    pub multdiv: MultDiv,
    pub factor: FACTOR<'a>,
}

pub fn termino(input: &str) -> Res<&str, TERMINO> {
    context(
        "termino",
        tuple((
            factor,
            many0(tuple((
                space,
                multdiv,
                space,
                factor
            )))
        )),
    )(input)
    .map(|(next_input, res)| {
        let factor = res.0;
        let mut qps = Vec::new();
        for qp in res.1 {
            let (_, multdiv, _, factor) = qp;
            qps.push(TERMINO2 {
                multdiv,
                factor,
            });
        }
        (
            next_input,
            TERMINO {
                factor,
                termino2: qps
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
    fn test_termino() {
        assert_eq!(
            termino("aaa * aaa"),
            Ok((
                "",
                TERMINO {
                    factor: "aaa",
                    multdiv: MultDiv::MULT,
                    factor2: "aaa"
                },
            ))
        );

        assert_eq!(
            termino("aaa / aaa"),
            Ok((
                "",
                TERMINO {
                    factor: "aaa",
                    multdiv: MultDiv::DIV,
                    factor2: "aaa"
                },
            ))
        );
    }
}