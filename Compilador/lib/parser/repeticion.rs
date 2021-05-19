use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::expresion::*;
use crate::parser::reglas_expresion::exp::*;
use crate::parser::bloque::*;
use crate::parser::dimensiones::*;

pub fn mientras(input: &str) -> IResult<&str, &str> {
  tuple((tag("mientras"), ws, tag("("), ws, expresion, ws, tag(")")))(input)
  .map(|(next_input, _res)| {
    (next_input, "mientras")
  })
}

pub fn desde_id(input: &str) -> IResult<&str, &str> {
  tuple((id, many0(tuple((ws, tag("."), ws, id))), con_dim))(input)
  .map(|(next_input, _res)| {
    (next_input, "desde_id")
  })
}

pub fn desde(input: &str) -> IResult<&str, &str> {
  tuple((tag("desde"), necessary_ws, desde_id, ws, tag("="), ws, exp, necessary_ws, tag("hasta"), necessary_ws, exp))(input)
  .map(|(next_input, _res)| {
    (next_input, "desde")
  })
}

pub fn repeticion(input: &str) -> IResult<&str, &str> {
  tuple((alt((mientras, desde)), necessary_ws, bloque))(input)
  .map(|(next_input, _res)| {
    (next_input, "repeticion")
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
    assert_eq!(mientras("mientras(expresion)"),    Ok(("", "mientras")));
    assert_eq!(mientras("mientras ( expresion )"), Ok(("", "mientras")));
  }

  #[test]
  fn test_desde() {
    assert_eq!(desde("desde id = 10 hasta 20"),         Ok(("", "desde")));
    // assert_eq!(desde("desde id = num_entero hasta num_entero"), Ok(("", "desde")));
    assert_eq!(desde("desde id[id] = 10 hasta 20"),     Ok(("", "desde")));
    assert_eq!(desde("desde id[id][id] = 10 hasta 20"), Ok(("", "desde")));
    assert_eq!(desde("desde id.id[id] = 10 hasta 20"),  Ok(("", "desde")));
    assert_eq!(desde("desde id.id = 15 hasta 25"),      Ok(("", "desde")));
  }

  #[test]
  fn test_repeticion() {
    // assert_eq!(repeticion("mientras(expresion) bloque"), Ok(("", ("mientras", "bloque"))));
    // assert_eq!(repeticion("desde id = num_entero hasta num_entero bloque"), Ok(("", ("desde", "bloque"))));

    assert_eq!(repeticion("mientras(expresion) {}"),    Ok(("", "repeticion")));
    assert_eq!(repeticion("desde id = 10 hasta 20 {}"), Ok(("", "repeticion")));
  }
}
