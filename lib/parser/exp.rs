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
use crate::parser::termino::*;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct EXP<'a> {
    pub termino: TERMINO<'a>,
    pub exp2: Vec<EXP2<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EXP2<'a> {
    pub sumaresta: SumaResta,
    pub termino: TERMINO<'a>,
}

pub fn exp(input: &str) -> Res<&str, EXP> {
    context(
        "exp",
        tuple((
            termino,
            many0(tuple((
                space,
                sumaresta,
                space,
                termino
            )))
        )),
    )(input)
    .map(|(next_input, res)| {
        let termino = res.0;
        let mut qps = Vec::new();
        for qp in res.1 {
            let (_, sumaresta, _, termino) = qp;
            qps.push(EXP2 {
                sumaresta,
                termino,
            });
        }
        (
            next_input,
            EXP {
                termino,
                exp2: qps
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
    fn test_exp() {
        assert_eq!(
            exp("aaa * aaa + aaa"),
            Ok((
                "",
                EXP {
                    termino: TERMINO {
                        factor: "aaa",
                        multdiv: MultDiv::MULT,
                        factor2: "aaa"
                    },
                    exp2: vec![
                        EXP2 {
                            sumaresta: SumaResta::SUM,
                            termino2: "aaa"
                        }
                    ] 
                },
            ))
        );

        assert_eq!(
            exp("aaa * aaa - aaa"),
            Ok((
                "",
                EXP {
                    termino: TERMINO {
                        factor: "aaa",
                        multdiv: MultDiv::MULT,
                        factor2: "aaa"
                    },
                    sumaresta: SumaResta::SUB,
                    termino2: "aaa"
                },
            ))
        );
    }
}