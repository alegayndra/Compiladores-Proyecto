use nom::{
  IResult,
  sequence::tuple,
  multi::many0
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::factor::*;

// pub fn termino(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
pub fn termino(input: &str) -> IResult<&str, &str> {
  // tuple((tag("factor"), many0(tuple((ws, op_multdiv, ws, tag("factor"))))))(input)
  tuple((factor, many0(tuple((ws, op_multdiv, ws, factor)))))(input)
  .map(|(next_input, res)| {
    (next_input, "termino")
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
  fn test_termino() {
    // assert_eq!(termino("factor"), Ok(("", vec![("+", "factor")])));
    // assert_eq!(termino("factor * factor * factor / factor"), Ok(("", 
    //   vec![
    //     ("+", "factor"),
    //     ("*", "factor"),
    //     ("*", "factor"),
    //     ("/", "factor")
    //   ]
    // )));

    assert_eq!(termino("factor"), Ok(("", "termino")));
    assert_eq!(termino("factor * factor * factor / factor"), Ok(("", "termino")));

    assert_eq!(termino("num_entero"), Ok(("", "termino")));
    assert_eq!(termino("id"), Ok(("", "termino")));
    assert_eq!(termino("id * num_entero * id2 / id3"), Ok(("", "termino")));
  }
}