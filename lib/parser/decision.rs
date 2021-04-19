use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;

fn sino(input: &str) -> IResult<&str, &str> {
  alt((
    tuple((ws, tag("sino"), ws, tag("bloque"))),
    tuple((ws, ws, ws, ws))
  ))(input)
  .map(|(next_input, _res)| {
    (next_input, "sino")
  })

}

// pub fn decision(input: &str) -> IResult<&str, &str> {
  pub fn decision(input: &str) -> IResult<&str, &str> {
  tuple((tag("si"), ws, tag("("), ws, tag("expresion"), ws, tag(")"), ws, tag("bloque"), sino))
  (input)
  .map(|(next_input, __res)| {
    // let (_, _, _, _, exp, _, _, _, _, _sino) = res;
    // (next_input, exp)
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
    assert_eq!(decision("si ( expresion ) bloque "), Ok(("", "expresion")));
    assert_eq!(decision("si ( expresion ) bloque sino bloque"), Ok(("", "expresion")));
    // assert_eq!(leer("lee()"), Ok(("", vec![])));
  }
}
