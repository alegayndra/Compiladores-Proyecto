use nom::{
  IResult,
  sequence::tuple,
  multi::many0
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::termino::*;

// pub fn exp(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
pub fn exp(input: &str) -> IResult<&str, &str> {
  // tuple((tag("termino"), many0(tuple((ws, op_sumsub, ws, tag("termino"))))))(input)
  tuple((
    termino,
    many0(
      tuple((ws, op_sumsub, ws, termino))
    )
  ))
  (input)
  .map(|(next_input, res)| {
    let (termino, terminos) = res;
    let mut lista_terminos = Vec::new();
    lista_terminos.push(("+", termino));
    for term in terminos {
      let (_, op, _, t) = term;
      lista_terminos.push((op, t));
    }
    // (next_input, lista_terminos)
    (next_input, "exp")
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
    // assert_eq!(exp("termino"), Ok(("", vec![("+", "termino")])));
    // assert_eq!(exp("termino + termino - termino - termino"), Ok(("", 
    //   vec![
    //     ("+", "termino"),
    //     ("+", "termino"),
    //     ("-", "termino"),
    //     ("-", "termino")
    //   ]
    // )));
    assert_eq!(exp("num_entero"), Ok(("", "exp")));
    assert_eq!(exp("id"), Ok(("", "exp")));
    assert_eq!(exp("id * num_entero"), Ok(("", "exp")));
    assert_eq!(exp("id + num_entero"), Ok(("", "exp")));
    assert_eq!(exp("id + num_entero * id2 - num_entero - termino"), Ok(("", "exp")));
  }
}