use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};
  
use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::exp::*;

// pub fn asignacion(input: &str) -> IResult<&str, ((&str,Vec<&str>), &str)> {
pub fn asignacion(input: &str) -> IResult<&str, &str> {
  tuple((id_parser, ws, tag("="), ws, exp, ws, tag(";")))(input)
  .map(|(next_input, _res)| {
    // let (id_parser, _, _, _, exp, _, _) = res;
    // (next_input, (id_parser, exp))
    (next_input, "asignacion")
  })
}

#[test]
  fn test_asignacion() {
    assert_eq!(asignacion("id = 10;"), Ok(("", "asignacion")));
  }
  