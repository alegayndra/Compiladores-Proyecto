use nom::{
    branch::alt,
    // bytes::complete::{tag, tag_no_case, take, take_while},
    bytes::complete::{tag, tag_no_case, take_while},
    // character::complete::{alpha1, alphanumeric1, one_of},
    // combinator::opt,
    error::{context, ErrorKind, VerboseError},
    // multi::{count, many0, many1, many_m_n},
    multi::many0,
    // sequence::{preceded, separated_pair, terminated, tuple},
    sequence::tuple,
    // AsChar, Err as NomErr, IResult, InputTakeAtPosition,
    AsChar, IResult, InputTakeAtPosition,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub enum SumaResta {
    SUM,
    SUB
}

impl From<&str> for SumaResta {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "+" => SumaResta::SUM,
            "-" => SumaResta::SUB,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}

pub fn sumaresta(input: &str) -> Res<&str, SumaResta> {
    context(
        "sumaresta",
        alt((tag_no_case("+"), tag_no_case("-"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

#[derive(Debug, PartialEq, Eq)]
pub enum MultDiv {
    MULT,
    DIV
}

impl From<&str> for MultDiv {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "*" => MultDiv::MULT,
            "/" => MultDiv::DIV,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}

pub fn multdiv(input: &str) -> Res<&str, MultDiv> {
    context(
        "multdiv",
        alt((tag_no_case("*"), tag_no_case("/"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn colon(input: &str) -> Res<&str, &str> {
    context(
        "colon",
        tag_no_case(":"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn coma(input: &str) -> Res<&str, &str> {
    context(
        "coma",
        tag_no_case(","),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn semicolon(input: &str) -> Res<&str, &str> {
    context(
        "semicolon",
        tag_no_case(";"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn abrirllave(input: &str) -> Res<&str, &str> {
    context(
        "abrirllave",
        tag_no_case("{"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn cerrarllave(input: &str) -> Res<&str, &str> {
    context(
        "cerrarllave",
        tag_no_case("}"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn space(input: &str) -> Res<&str, &str> {
    context(
        "space",
        take_while(|c| c == ' '),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn ids(input: &str) -> Res<&str, Vec<& str>> {
    context(
        "ids",
        tuple((
            url_code_points,
            many0(tuple((
                space,
                tag(","),
                space,
                url_code_points,
            ))),
        )),
    )(input)
    .map(|(next_input, res)| {
        let mut qps = Vec::new();
        qps.push(res.0);
        for qp in res.1 {
            qps.push(qp.3);
        }
        (next_input, qps)
    })
}

pub fn url_code_points(input: &str) -> Res<&str, &str> {
    input.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !(char_item == '-') && !char_item.is_alphanum() && !(char_item == '.')
        },
        ErrorKind::AlphaNumeric,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_multdiv() {
        assert_eq!(multdiv("*"), Ok(("", MultDiv::MULT)));
        assert_eq!(multdiv("/"), Ok(("", MultDiv::DIV)));
        assert_eq!(
            multdiv("laksl"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("laksl", VerboseErrorKind::Context("multdiv")),
                ]
            }))
        );
    }

    #[test]
    fn test_sumaresta() {
        assert_eq!(sumaresta("+"), Ok(("", SumaResta::SUM)));
        assert_eq!(sumaresta("-"), Ok(("", SumaResta::SUB)));
        assert_eq!(
            sumaresta("laksl"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("laksl", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("laksl", VerboseErrorKind::Context("sumaresta")),
                ]
            }))
        );
    }

    #[test]
    fn test_colon() {
        assert_eq!(colon(":"), Ok(("", ":")));
        assert_eq!(
            colon("a"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("a", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("a", VerboseErrorKind::Context("colon")),
                ]
            }))
        );
    }

    #[test]
    fn test_semicolon() {
        assert_eq!(semicolon(";"), Ok(("", ";")));
        assert_eq!(
            semicolon("a"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("a", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("a", VerboseErrorKind::Context("semicolon")),
                ]
            }))
        );
    }

    #[test]
    fn test_coma() {
        assert_eq!(coma(","), Ok(("", ",")));
        assert_eq!(
            coma("a"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("a", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("a", VerboseErrorKind::Context("coma")),
                ]
            }))
        );
    }

    #[test]
    fn test_abrirllave() {
        assert_eq!(abrirllave("{"), Ok(("", "{")));
        assert_eq!(
            abrirllave("a"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("a", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("a", VerboseErrorKind::Context("abrirllave")),
                ]
            }))
        );
    }

    #[test]
    fn test_cerrarllave() {
        assert_eq!(cerrarllave("}"), Ok(("", "}")));
        assert_eq!(
            cerrarllave("a"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("a", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("a", VerboseErrorKind::Context("cerrarllave")),
                ]
            }))
        );
    }

    #[test]
    fn test_space() {
        assert_eq!(space(" "), Ok(("", " ")));
        assert_eq!(space("    "), Ok(("", "    ")));
        assert_eq!(space(""), Ok(("", "")));
        assert_eq!(space("a"), Ok(("a", "")));
    }

    #[test]
    fn test_ids() {
        assert_eq!(ids("id"), Ok(("", vec!["id"])));
        assert_eq!(ids("id, abr"), Ok(("", vec!["id", "abr"])));
        // assert_eq!(
        //     ids("id, "),
        //     Err(NomErr::Error(VerboseError {
        //         errors: vec![
        //             ("id, ", VerboseErrorKind::Nom(ErrorKind::Tag)),
        //             ("id, ", VerboseErrorKind::Context("ids")),
        //         ]
        //     })) 
        // );
        assert_eq!(
            ids(":"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    (":", VerboseErrorKind::Nom(ErrorKind::AlphaNumeric)),
                    (":", VerboseErrorKind::Context("ids")),
                ]
            }))
        );
    }
}
