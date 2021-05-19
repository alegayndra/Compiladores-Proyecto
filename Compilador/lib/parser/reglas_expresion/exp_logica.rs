use nom::{
  IResult,
  sequence::tuple,
  multi::many0
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::expresion::*;

pub fn exp_logica(input: &str) -> IResult<&str, &str> {
  tuple((
    expresion,
    many0(
      tuple((ws, op_logica, ws, expresion))
    )
  ))
  (input)
  .map(|(next_input, _res)| {
    (next_input, "exp_logica")
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
  fn test_exp() {
    assert_eq!(exp_logica("abr  "),                               Ok(("  ", "exp_logica")));
    assert_eq!(exp_logica("num_entero"),                          Ok(("", "exp_logica")));
    assert_eq!(exp_logica("id"),                                  Ok(("", "exp_logica")));
    assert_eq!(exp_logica("id  "),                                Ok(("  ", "exp_logica")));
    assert_eq!(exp_logica("10  "),                                Ok(("  ", "exp_logica")));
    assert_eq!(exp_logica("id & num_entero"),                     Ok(("", "exp_logica")));
    assert_eq!(exp_logica("id | num_entero"),                     Ok(("", "exp_logica")));
    assert_eq!(exp_logica("id | id > 2 * ( - num_entero + id )"), Ok(("", "exp_logica")));
  }
}
