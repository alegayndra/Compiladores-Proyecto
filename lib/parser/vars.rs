use nom::{
    branch::alt,
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

// // use lex::*;
// // mod lex;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct VARS<'a> {
    pub varter: &'a str,
    pub ids: Vec<&'a str>,
    pub colon: &'a str,
    pub tipo: Tipo,
    pub semicolon: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tipo {
    INT,
    FLOAT,
}

impl From<&str> for Tipo {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "int" => Tipo::INT,
            "float" => Tipo::FLOAT,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}

fn tipo(input: &str) -> Res<&str, Tipo> {
    context(
        "tipo",
        alt((tag_no_case("int"), tag_no_case("float"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn varter(input: &str) -> Res<&str, &str> {
    context(
        "varter",
        tag_no_case("var"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn vars(input: &str) -> Res<&str, VARS> {
    context(
        "vars",
        tuple((
            varter,
            space,
            ids,
            space,
            colon,
            space,
            tipo,
            space,
            semicolon,
        )),
    )(input)
    .map(|(next_input, res)| {
        let (varter, _, ids, _, colon, _, tipo, _, semicolon) = res;
        (
            next_input,
            VARS {
                varter,
                ids,
                colon,
                tipo,
                semicolon,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn is_working() {
    //     assert_eq!(2+2, 4);
    // }

    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_tipo() {
        assert_eq!(tipo("intaa"), Ok(("aa", Tipo::INT)));
        assert_eq!(tipo("floataa"), Ok(("aa", Tipo::FLOAT)));
        assert_eq!(
            tipo("laksl"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("laksl", VerboseErrorKind::Context("tipo")),
                ]
            }))
        );
    }

    #[test]
    fn test_varter() {
        assert_eq!(varter("var"), Ok(("", "var")));
        assert_eq!(
            varter("a"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("a", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("a", VerboseErrorKind::Context("varter")),
                ]
            }))
        );
    }

    #[test]
    fn test_vars() {
        assert_eq!(
            vars("var id : int ;"),
            Ok((
                "",
                VARS {
                    varter: "var",
                    ids: vec!["id"],
                    colon: ":",
                    tipo: Tipo::INT,
                    semicolon: ";",
                }
            ))
        );

        assert_eq!(
            vars("var variable, id, aaaa : float ;"),
            Ok((
                "",
                VARS {
                    varter: "var",
                    ids: vec!["variable" ,"id", "aaaa"],
                    colon: ":",
                    tipo: Tipo::FLOAT,
                    semicolon: ";",
                }
            ))
        );
    }
}