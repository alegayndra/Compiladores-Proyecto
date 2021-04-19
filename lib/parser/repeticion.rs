use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::expresion::*;
use crate::parser::reglas_expresion::exp::*;

pub fn mientras(input: &str) -> IResult<&str, &str> {
  tuple((tag("mientras"), ws, tag("("), ws, expresion, ws, tag(")")))(input)
  .map(|(next_input, _res)| {
    // let (_, _, _, _, expresion, _, bloque) = res;
    (next_input, "mientras")
  })
}

pub fn desde(input: &str) -> IResult<&str, &str> {
  tuple((tag("desde"), necessary_ws, id_con_dim, ws, tag("="), ws, exp, necessary_ws, tag("hasta"), necessary_ws, exp))(input)
  .map(|(next_input, _res)| {
    // let (_, id, _, _, _, exp, _, _, _, exp2) = res;
    (next_input, "desde")
  })
}

pub fn repeticion(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((alt((mientras, desde)), necessary_ws, tag("bloque")))(input)
  .map(|(next_input, res)| {
    let (repet, _, bloque) = res;
    (next_input, (repet, bloque))
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
  fn test_mientras() {
    assert_eq!(mientras("mientras(expresion)"), Ok(("", "mientras")));
    assert_eq!(mientras("mientras ( expresion )"), Ok(("", "mientras")));
  }

  #[test]
  fn test_desde() {
    assert_eq!(desde("desde id = num_entero hasta num_entero"), Ok(("", "desde")));
    assert_eq!(desde("desde id[id] = num_entero hasta num_entero"), Ok(("", "desde")));
    assert_eq!(desde("desde id[id][id] = num_entero hasta num_entero"), Ok(("", "desde")));
  }

  #[test]
  fn test_repeticion() {
    assert_eq!(repeticion("mientras(expresion) bloque"), Ok(("", ("mientras", "bloque"))));
    assert_eq!(repeticion("desde id = num_entero hasta num_entero bloque"), Ok(("", ("desde", "bloque"))));
  }
}
