use nom::{
  IResult,
  combinator::{recognize, opt},
  multi::{many0, many1},
  character::complete::{one_of, char},
  sequence::{terminated, tuple, preceded,delimited},
  branch::alt,
  bytes::complete::{tag, take_while_m_n}
};

pub fn num_entero(input: &str) -> IResult<&str, &str> {
  recognize(
    many1(
      terminated(one_of("0123456789"), many0(char('_')))
    )
  )(input)
}

pub fn caracter(input: &str) -> IResult<&str, &str> {
  delimited(tag("\""), take_while_m_n(1, 1, |c| c != ' '), tag("\""))(input)
}

pub fn num_flotante(input: &str) -> IResult<&str, &str> {
  alt((
    // Case one: .42
    recognize(
      tuple((
        char('.'),
        num_entero,
        opt(tuple((
          one_of("eE"),
          opt(one_of("+-")),
          num_entero
        )))
      ))
    )
    , // Case two: 42e42 and 42.42e42
    recognize(
      tuple((
        num_entero,
        opt(preceded(
          char('.'),
          num_entero,
        )),
        one_of("eE"),
        opt(one_of("+-")),
        num_entero
      ))
    )
    , // Case three: 42. and 42.42
    recognize(
      tuple((
        num_entero,
        char('.'),
        opt(num_entero)
      ))
    )
  ))(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_num_caracter() {
    assert_eq!(caracter("\"a\""), Ok(("", "a")));
    assert_eq!(caracter("\"-\""), Ok(("", "-")));
  }

  #[test]
  fn test_num_entero() {
    assert_eq!(num_entero("1"), Ok(("", "1")));
    assert_eq!(num_entero("11"), Ok(("", "11")));
    assert_eq!(num_entero("1123131"), Ok(("", "1123131")));
  }

  #[test]
  fn test_num_flotante() {
    assert_eq!(num_flotante("1.1"), Ok(("", "1.1")));
    assert_eq!(num_flotante("11.23"), Ok(("", "11.23")));
    assert_eq!(num_flotante("112.3131"), Ok(("", "112.3131")));
  }
}
