use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};
  
use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::exp::*;

pub fn asignacion(input: &str) -> IResult<&str, &str> {
  tuple((id_con_dim, ws, tag("="), ws, exp, ws, tag(";")))(input)
  .map(|(next_input, _res)| {
    (next_input, "asignacion")
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
  fn test_asignacion() {
    assert_eq!(asignacion("id = 10;"), Ok(("", "asignacion")));
  }
}
  