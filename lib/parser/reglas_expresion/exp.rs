use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
  multi::many0
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;

pub fn exp(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
  tuple((tag("termino"), many0(tuple((ws, op_sumsub, ws, tag("termino"))))))(input)
  .map(|(next_input, res)| {
    let (termino, terminos) = res;
    let mut lista_terminos = Vec::new();
    lista_terminos.push(("+", termino));
    for term in terminos {
      let (_, op, _, t) = term;
      lista_terminos.push((op, t));
    }
    (next_input, lista_terminos)
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
    assert_eq!(exp("termino"), Ok(("", vec![("+", "termino")])));
    assert_eq!(exp("termino + termino - termino - termino"), Ok(("", 
      vec![
        ("+", "termino"),
        ("+", "termino"),
        ("-", "termino"),
        ("-", "termino")
      ]
    )));
  }
}