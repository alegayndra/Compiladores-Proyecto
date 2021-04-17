use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
  multi::many0
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;

pub fn termino(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
  tuple((tag("factor"), many0(tuple((ws, op_multdiv, ws, tag("factor"))))))(input)
  .map(|(next_input, res)| {
    let (factor, factores) = res;
    let mut lista_factores = Vec::new();
    lista_factores.push(("+", factor));
    for fac in factores {
      let (_, op, _, fact) = fac;
      lista_factores.push((op, fact));
    }
    (next_input, lista_factores)
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
    assert_eq!(termino("factor"), Ok(("", vec![("+", "factor")])));
    assert_eq!(termino("factor * factor * factor / factor"), Ok(("", 
      vec![
        ("+", "factor"),
        ("*", "factor"),
        ("*", "factor"),
        ("/", "factor")
      ]
    )));
  }
}