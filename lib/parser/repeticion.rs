use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;

pub fn mientras(input: &str) -> IResult<&str, &str> {
  tuple((tag("mientras"), ws, tag("("), ws, tag("expresion"), ws, tag(")")))(input)
  .map(|(next_input, _res)| {
    // let (_, _, _, _, expresion, _, bloque) = res;
    (next_input, "mientras")
  })
}

pub fn desde(input: &str) -> IResult<&str, &str> {
  tuple((tag("desde"), necessary_ws, id_con_dim, ws, tag("="), ws, tag("exp"), necessary_ws, tag("hasta"), necessary_ws, tag("exp")))(input)
  .map(|(next_input, _res)| {
    // let (_, id, _, _, _, exp, _, _, _, exp2) = res;
    (next_input, "desde")
  })
}

// pub fn repeticion(input: &str) -> IResult<&str, (&str, &str)> {
pub fn repeticion(input: &str) -> IResult<&str, &str> {
  tuple((alt((mientras, desde)), necessary_ws, tag("bloque")))(input)
  .map(|(next_input, __res)| {
    // let (repet, _, bloque) = res;
    // (next_input, (repet, bloque))
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
    assert_eq!(mientras("mientras(expresion)"), Ok(("", "mientras")));
    assert_eq!(mientras("mientras ( expresion )"), Ok(("", "mientras")));
  }

  #[test]
  fn test_desde() {
    assert_eq!(desde("desde id = exp hasta exp"), Ok(("", "desde")));
    assert_eq!(desde("desde id[id] = exp hasta exp"), Ok(("", "desde")));
    assert_eq!(desde("desde id[id][id] = exp hasta exp"), Ok(("", "desde")));
  }

  #[test]
  fn test_repeticion() {
    assert_eq!(repeticion("mientras(expresion) bloque"), Ok(("", ("mientras", "bloque"))));
    assert_eq!(repeticion("desde id = exp hasta exp bloque"), Ok(("", ("desde", "bloque"))));
  }
}
