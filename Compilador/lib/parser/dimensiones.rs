use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::{tuple, delimited},
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::exp::*;

pub fn dimension(input: &str) -> IResult<&str, Vec<&str>> {
  delimited(tuple((tag("["), ws)), exp, tuple((ws, tag("]"))))(input)
  .map(|(next_input, dimension)| {
    (next_input, vec![dimension])
  })
}

fn dos_dimensiones(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((dimension, ws, dimension))(input)
  .map(|(next_input, (dimension_1, _, dimension_2))| {
    (next_input, vec![dimension_1[0], dimension_2[0]])
  })
}

pub fn ws_vec(input: &str) -> IResult<&str, Vec<&str>> {
  Ok((input, vec![]))
}

pub fn con_dim(input: &str) -> IResult<&str, Vec<&str>> {
  alt((dos_dimensiones, dimension, ws_vec))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dimension() {
    assert_eq!(dimension("[termino]"),     Ok(("", vec!["exp"])));
    assert_eq!(dimension("[num_float]"),   Ok(("", vec!["exp"])));
    assert_eq!(dimension("[  id  ]"),      Ok(("", vec!["exp"])));
  }

  #[test]
  fn test_dos_dimensiones() {
    assert_eq!(dos_dimensiones("[id][id]"),         Ok(("", vec!["exp", "exp"])));
    assert_eq!(dos_dimensiones("[ id ][ id ]"),     Ok(("", vec!["exp", "exp"])));
    assert_eq!(dos_dimensiones("[  id  ][  id  ]"), Ok(("", vec!["exp", "exp"])));
  }

  #[test]
  fn test_ws_vec() {
    assert_eq!(ws_vec("aaaa"), Ok(("aaaa", vec![])));
    assert_eq!(ws_vec("bbbb"), Ok(("bbbb", vec![])));
    assert_eq!(ws_vec("cccc"), Ok(("cccc", vec![])));
    assert_eq!(ws_vec("    "), Ok(("    ", vec![])));
  }

  #[test]
  fn test_con_dim() {
    assert_eq!(con_dim("[id]"),     Ok(("", vec!["exp"])));
    assert_eq!(con_dim("[id][id]"), Ok(("", vec!["exp", "exp"])));
    assert_eq!(con_dim("aaaa"),     Ok(("aaaa", vec![])));
  }
}
