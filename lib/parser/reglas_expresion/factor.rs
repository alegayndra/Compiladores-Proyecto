use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};
  
use crate::scanners::ws::*;
use crate::parser::reglas_expresion::valor::*;
use crate::parser::reglas_expresion::expresion::*;

fn retorna_expresion(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((tag("("), ws, expresion, ws, tag(")")))(input)
  .map(|(next_input, res)| {
    let (_, _, expresion, _, _) = res;
    (next_input, ("operacion", expresion))
  })
}

fn signo_valor(input: &str) -> IResult<&str, &str> {
  alt((tag("+"), tag("-"), ws))(input)
}

fn valor_factor(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((signo_valor, ws, valor))(input)
  .map(|(next_input, res)| {
    let (signo, _, valor) = res;
    (next_input, (signo, valor.0))
  })
}

// pub fn factor(input: &str) -> IResult<&str, (&str, &str)> {
pub fn factor(input: &str) -> IResult<&str, &str> {
  alt((retorna_expresion, valor_factor))(input)
  .map(|(next_input, _)| {
    // let (signo, _, valor) = res;
    // (next_input, (signo, valor.0))
    (next_input, "factor")
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
  fn test_signo_valor() {
    assert_eq!(signo_valor("+"), Ok(("", "+")));
    assert_eq!(signo_valor("-"), Ok(("", "-")));
    assert_eq!(signo_valor("  "), Ok(("", "  ")));
  }

  #[test]
  fn test_valor_factor() {
    assert_eq!(valor_factor("num_entero"), Ok(("", ("", "num_entero"))));
    assert_eq!(valor_factor("- num_entero"), Ok(("", ("-", "num_entero"))));
    assert_eq!(valor_factor("+ \"soyUnaVariable\""), Ok(("", ("+", "soyUnaVariable"))));
    assert_eq!(valor_factor("+ Nombre . metodo ()"), Ok(("", ("+", "Nombre"))));
  }

  #[test]
  fn test_factor() {
    // assert_eq!(factor("- num_entero"), Ok(("", ("-", "num_entero"))));
    // assert_eq!(factor("+ \"soyUnaVariable\""), Ok(("", ("+", "soyUnaVariable"))));
    // assert_eq!(factor("+ Nombre . metodo ()"), Ok(("", ("+", "Nombre"))));
    // assert_eq!(factor("( expresion )"), Ok(("", ("operacion", "expresion"))));
    
    assert_eq!(factor("- num_entero"),         Ok(("", "factor")));
    assert_eq!(factor("+ \"soyUnaVariable\""), Ok(("", "factor")));
    assert_eq!(factor("+ Nombre . metodo ()"), Ok(("", "factor")));
    assert_eq!(factor("( expresion )"),        Ok(("", "factor")));
    assert_eq!(factor("( num_entero )"),        Ok(("", "factor")));
    assert_eq!(factor("( num_entero * id )"),        Ok(("", "factor")));
  }
}