use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};
  
use crate::scanners::ws::*;
use crate::parser::valor::*;

fn retorna_expresion(input: &str) -> IResult<&str, (&str,&str)> {
  tuple((tag("("), ws, tag("expresion"), ws, tag(")")))(input)
  .map(|(next_input, res)| {
    let (_, _, expresion, _, _) = res;
    (next_input, ("operacion", expresion))
  })
}


fn signo_valor(input: &str) -> IResult<&str, &str> {
  alt((tag("+"),  tag("-"), ws))(input)
}

fn valor_factor(input: &str) -> IResult<&str, (&str,&str)> {
  tuple((signo_valor, ws, valor))(input)
  .map(|(next_input, res)| {
   let (signo, _, valor) = res;
   (next_input, (signo, valor.0))
 })
}

pub fn factor(input: &str) -> IResult<&str, (&str, &str)> {
  alt((valor_factor,  retorna_expresion))(input)
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
    assert_eq!(valor_factor("- num_entero"), Ok(("", ("-", "num_entero"))));
    assert_eq!(valor_factor("+ \"soyUnaVariable\""), Ok(("", ("+", "soyUnaVariable"))));
    assert_eq!(valor_factor("+ Nombre . metodo ()"), Ok(("", ("+", "Nombre"))));
  }
}