use nom::{
    branch::alt,
    // bytes::complete::{tag, tag_no_case, take},
    bytes::complete::tag_no_case,
    // character::complete::{alpha1, alphanumeric1, one_of},
    combinator::opt,
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
use crate::parser::exp::*;

// // use lex::*;
// // mod lex;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct EXPRESION<'a> {
    pub exp: EXP<'a>,
    pub signo: Option<Signos>,
    pub exp2: Option<EXP<'a>>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Signos {
    GT,
    LT,
    NE
}

impl From<&str> for Signos {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "<>" => Signos::NE,
            ">" => Signos::GT,
            "<" => Signos::LT,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}

fn signos(input: &str) -> Res<&str, Signos> {
    context(
        "signos",
        alt((tag_no_case("<>"), tag_no_case("<"), tag_no_case(">"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn expresion(input: &str) -> Res<&str, EXPRESION> {
    context(
        "expresion",
        tuple((
            exp,
            opt(space),
            opt(signos),
            opt(space),
            opt(exp)
        )),
    )(input)
    .map(|(next_input, res)| {
        let (exp, _, signo, _, exp2) = res;
        (
            next_input,
            EXPRESION {
                exp,
                signo,
                exp2
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    use crate::parser::termino::*;

    #[test]
    fn test_signos() {
        assert_eq!(signos(">"), Ok(("", Signos::GT)));
        assert_eq!(signos("<"), Ok(("", Signos::LT)));
        assert_eq!(signos("<>"), Ok(("", Signos::NE)));
        assert_eq!(
            signos("laksl"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("laksl", VerboseErrorKind::Context("signos")),
                ]
            }))
        );
    }

    #[test]
    fn test_expresion() {
        assert_eq!(
            expresion("aaa * aaa + aaaa < aaa"),
            Ok((
                "",
                EXPRESION {
                    exp: EXP {
                        termino: TERMINO {
                            factor: "aaa",
                            multdiv: MultDiv::MULT,
                            factor2: "aaa"
                        },
                        sumaresta: SumaResta::SUM,
                        termino2: "aaaa",
                    },
                    expresion2: vec![
                        EXPRESION2 {
                            signo: Signos::LT,
                            exp: "aaa"
                        }
                    ]
                },
            ))
        );

        assert_eq!(
            expresion("aaa * aaa + aaaa > aaa"),
            Ok((
                "",
                EXPRESION {
                    exp: EXP {
                        termino: TERMINO {
                            factor: "aaa",
                            multdiv: MultDiv::MULT,
                            factor2: "aaa"
                        },
                        sumaresta: SumaResta::SUM,
                        termino2: "aaaa",
                    },
                    expresion2: vec![
                        EXPRESION2 {
                            signo: Signos::GT,
                            exp: "aaa"
                        }
                    ]
                },
            ))
        );

        assert_eq!(
            expresion("aaa * aaa - aaaa <> aaa"),
            Ok((
                "",
                EXPRESION {
                    exp: EXP {
                        termino: TERMINO {
                            factor: "aaa",
                            multdiv: MultDiv::MULT,
                            factor2: "aaa"
                        },
                        sumaresta: SumaResta::SUB,
                        termino2: "aaaa",
                    },
                    expresion2: vec![
                        EXPRESION2 {
                            signo: Signos::NE,
                            exp: "aaa"
                            exp: EXP {

                            }
                        }
                    ]
                },
            ))
        );
    }
}