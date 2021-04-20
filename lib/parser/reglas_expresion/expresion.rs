use nom::{
  IResult,
  sequence::tuple,
  branch::alt
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::exp::*;

fn exp_extra(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((ws, op_relacional, ws, exp))(input)
  .map(|(next_input, res)| {
    let (_, op, _, exp) = res;
    (next_input, (op, exp))
  })
}

fn exp_vacio(input: &str) -> IResult<&str, (&str, &str)> {
  ws(input)
  .map(|(next_input, _)| {
    (next_input, ("", ""))
  })
}
fn exp_opcional(input: &str) -> IResult<&str, (&str, &str)> {
  alt((exp_extra, exp_vacio))(input)
}

// pub fn expresion(input: &str) -> IResult<&str, (&str, &str, &str)> {
  pub fn expresion(input: &str) -> IResult<&str, &str> {
  tuple((exp, exp_opcional))(input)
  .map(|(next_input, _res)| {
    // let (exp, exp_op) = res;
    // let (op, exp2) = exp_op;
    // (next_input, (exp, op, exp2))
    (next_input, "expresion")
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
  fn test_expresion() {
    // assert_eq!(expresion("exp"), Ok(("", ("exp", "", ""))));
    // assert_eq!(expresion("exp & exp"), Ok(("", ("exp", "&", "exp"))));
    assert_eq!(expresion("termino > termino"), Ok(("", "expresion")));
    assert_eq!(expresion("termino"), Ok(("", "expresion")));
    assert_eq!(expresion("id + num_entero * id2 - num_entero - termino"), Ok(("", "expresion")));
    assert_eq!(expresion("id + num_entero * id2 - num_entero - termino & id3"), Ok(("", "expresion")));
    assert_eq!(expresion("( id + num_entero * id2 - num_entero - termino & id3 )"), Ok(("", "expresion")));
  }
}