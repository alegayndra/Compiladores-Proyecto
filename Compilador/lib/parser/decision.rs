use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::exp_logica::*;
use crate::parser::bloque::*;

fn sino(input: &str) -> IResult<&str, &str> {
  alt((
    tuple((ws, tag("sino"), ws, bloque)),
    tuple((ws, ws, ws, ws))
  ))(input)
  .map(|(next_input, _res)| {
    (next_input, "sino")
  })

}

pub fn decision(input: &str) -> IResult<&str, &str> {
  tuple((tag("si"), ws, tag("("), ws, exp_logica, ws, tag(")"), ws, bloque, sino))
  (input)
  .map(|(next_input, __res)| {
    (next_input, "decision")
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
  fn test_decision() {
    // assert_eq!(decision("si ( expresion ) bloque "), Ok(("", "expresion")));
    // assert_eq!(decision("si ( expresion ) bloque sino bloque"), Ok(("", "expresion")));

    assert_eq!(decision("si ( expresion ) {} "),            Ok(("", "decision")));
    assert_eq!(decision("si ( expresion ) {} sino {}"), Ok(("", "decision")));
  }
}
