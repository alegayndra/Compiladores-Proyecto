use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::exp::*;

pub fn regresa(input: &str) -> IResult<&str, &str> {
  tuple((tag("regresa"), ws, exp, ws, tag(";")))(input)
  .map(|(next_input, _)| {
    (next_input, "regresa")
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_regresa() {
    assert_eq!(regresa("regresa  a;"),  Ok(("", "regresa")));
    assert_eq!(regresa("regresa 0;"),   Ok(("", "regresa")));
  }
}
