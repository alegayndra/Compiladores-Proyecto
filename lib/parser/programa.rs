use nom::{
    // branch::alt,
    // bytes::complete::{tag, tag_no_case, take},
    bytes::complete::tag_no_case,
    // character::complete::{alpha1, alphanumeric1, one_of},
    // combinator::opt,
    // error::{context, ErrorKind, VerboseError},
    error::{context, VerboseError},
    // multi::{count, many0, many1, many_m_n},
    // sequence::{preceded, separated_pair, terminated, tuple},
    sequence::tuple,
    // AsChar, Err as NomErr, IResult, InputTakeAtPosition,
    IResult,
};

use crate::lexer::*;
use crate::parser::bloque::*;
use crate::parser::vars::*;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct PROGRAMA<'a> {
    programter: &'a str,
    id: &'a str,
    semicolon: &'a str,
    vars: Option<VARS<'a>>,
    bloque: BLOQUE<'a>,
}

fn programter(input: &str) -> Res<&str, &str> {
    context(
        "programter",
        tag_no_case("program"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn program(input: &str) -> Res<&str, PROGRAMA> {
    context(
        "program",
        tuple((
            programter,
            space,
            url_code_points,
            space,
            semicolon,
            space,
            vars,
            space,
            bloque,
        )),
    )(input)
    .map(|(next_input, res)| {
        let (programter, _, id, _, semicolon, _, vars, _, bloque) = res;
        (
            next_input,
            PROGRAMA {
                programter,
                id,
                semicolon,
                vars: Some(vars),
                bloque,
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
    fn test_programa() {
        assert_eq!(
            program("program idprograma ; var id : int ; { aaaa }"),
            Ok((
                "", 
                PROGRAMA {
                    programter: "program",
                    id: "idprograma",
                    semicolon: ";",
                    vars: Some(
                        VARS {
                            varter: "var",
                            ids: vec!["id"],
                            colon: ":",
                            tipo: Tipo::INT,
                            semicolon: ";",
                        }
                    ),
                    bloque: BLOQUE {
                        abrirllave: "{",
                        estatuto: vec!["aaaa"],
                        cerrarllave: "}"
                    }
                },
            ))
        );

        // assert_eq!(
        //     program("program idprograma ; aaaa"),
        //     Ok((
        //         "", 
        //         PROGRAMA {
        //             programter: "program",
        //             id: "idprograma",
        //             semicolon: ";",
        //             vars: None,
        //             bloque: "aaaa",
        //         },
        //     ))
        // );
    }
}