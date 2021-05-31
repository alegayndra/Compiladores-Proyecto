use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::globales::*;

pub fn regresa(input: &str) -> IResult<&str, &str> {
  tuple((tag("regresa"), ws, exp, ws, tag(";")))(input)
  .map(|(next_input, _)| {
    unsafe {
      RETURN_EXISTENTE = true;
      CUADRUPLOS.lock().unwrap().agregar_cuadruplo_return(PILA_VALORES.lock().unwrap().pop().unwrap(), DIRECCION_CONTEXTO_FUNCION);
    }
    (next_input, "regresa")
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
  fn test_regresa() {
    assert_eq!(regresa("regresa  a;"),  Ok(("", "regresa")));
    assert_eq!(regresa("regresa 0;"),   Ok(("", "regresa")));
  }
}
