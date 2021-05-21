use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::valor::*;
use crate::parser::reglas_expresion::exp_logica::*;

fn retorna_expresion(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((tag("("), ws, exp_logica, ws, tag(")")))(input)
  .map(|(next_input, res)| {
    let (_, _, expresion, _, _) = res;
    (next_input, ("operacion", expresion))
  })
}

fn op_vacio(input: &str) -> IResult<&str, &str> {
  Ok((input, ""))
}

fn valor_factor(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((alt((op_sumsub, op_vacio)), ws, valor))(input)
  .map(|(next_input, res)| {
    let (signo, _, valor) = res;
    (next_input, (signo, valor.0))
  })
}

pub fn factor(input: &str) -> IResult<&str, &str> {
  alt((retorna_expresion, valor_factor))(input)
  .map(|(next_input, _)| {
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
  fn test_valor_factor() {
    assert_eq!(valor_factor("10"),                   Ok(("", ("", "10"))));
    assert_eq!(valor_factor("- 10"),                 Ok(("", ("-", "10"))));
    assert_eq!(valor_factor("+ \"s\""), Ok(("", ("+", "s"))));
    assert_eq!(valor_factor("+ Nombre.metodo()"),    Ok(("", ("+", "Nombre"))));
    assert_eq!(valor_factor("+ Nombre . metodo()"),  Ok(("", ("+", "Nombre"))));
  }

  #[test]
  fn test_factor() {
    assert_eq!(factor("- num_entero"),          Ok(("", "factor")));
    assert_eq!(factor("+ \"s\""),               Ok(("", "factor")));
    assert_eq!(factor("+ Nombre . metodo()"),   Ok(("", "factor")));
    assert_eq!(factor("( expresion )"),         Ok(("", "factor")));
    assert_eq!(factor("( num_entero )"),        Ok(("", "factor")));
    assert_eq!(factor("( num_entero * id )"),   Ok(("", "factor")));
    assert_eq!(factor("( num_entero & id )"),   Ok(("", "factor")));
    assert_eq!(factor("( 1 | 0 )"),             Ok(("", "factor")));
  }
}
