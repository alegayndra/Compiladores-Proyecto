use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::expresion::*;

pub fn dimension(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("["), ws, expresion, ws, tag("]")))(input)
  .map(|(next_input, res)| {
    let (_, _, dimension, _, _) = res;
    (next_input, vec![dimension])
  })
}

fn dos_dimensiones(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((dimension, ws, dimension))
  (input)
  .map(|(next_input, res)| {
    let (dimension_1, _, dimension_2) = res;
    (next_input, vec![dimension_1[0], dimension_2[0]])
  })
}

pub fn ws_vec(input: &str) -> IResult<&str, Vec<&str>> {
  ws(input)
  .map(|(next_input, _res)| {
    (next_input, vec![])
  })
}

pub fn con_dim(input: &str) -> IResult<&str, Vec<&str>> {
  alt((dos_dimensiones, dimension, ws_vec))
  (input)
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_dimension() {
    // assert_eq!(dimension("[id]"), Ok(("", vec!["id"])));
    // assert_eq!(dimension("[ id ]"), Ok(("", vec!["id"])));
    // assert_eq!(dimension("[  id  ]"), Ok(("", vec!["id"])));

    assert_eq!(dimension("[termino]"),     Ok(("", vec!["expresion"])));
    assert_eq!(dimension("[num_float]"),   Ok(("", vec!["expresion"])));
    assert_eq!(dimension("[  id  ]"), Ok(("", vec!["expresion"])));
  }

  #[test]
  fn test_dos_dimensiones() {
    // assert_eq!(dos_dimensiones("[id][id]"), Ok(("", vec!["id", "id"])));
    // assert_eq!(dos_dimensiones("[ id ][ id ]"), Ok(("", vec!["id", "id"])));
    // assert_eq!(dos_dimensiones("[  id  ][  id  ]"), Ok(("", vec!["id", "id"])));

    assert_eq!(dos_dimensiones("[id][id]"),         Ok(("", vec!["expresion", "expresion"])));
    assert_eq!(dos_dimensiones("[ id ][ id ]"),     Ok(("", vec!["expresion", "expresion"])));
    assert_eq!(dos_dimensiones("[  id  ][  id  ]"), Ok(("", vec!["expresion", "expresion"])));
  }

  #[test]
  fn test_ws_vec() {
    assert_eq!(ws_vec("aaaa"), Ok(("aaaa", vec![])));
    assert_eq!(ws_vec("bbbb"), Ok(("bbbb", vec![])));
    assert_eq!(ws_vec("cccc"), Ok(("cccc", vec![])));
  }

  #[test]
  fn test_con_dim() {
    // assert_eq!(con_dim("[id]"), Ok(("", vec!["id"])));
    // assert_eq!(con_dim("[id][id]"), Ok(("", vec!["id", "id"])));
    // assert_eq!(con_dim("aaaa"), Ok(("aaaa", vec![])));

    assert_eq!(con_dim("[id]"), Ok(("", vec!["expresion"])));
    assert_eq!(con_dim("[id][id]"), Ok(("", vec!["expresion", "expresion"])));
    assert_eq!(con_dim("aaaa"), Ok(("aaaa", vec![])));
  }
}
