use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
// use crate::scanners::id::*;

fn dimension(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("["), ws, tag("id"), ws, tag("]")))
  (input)
  .map(|(next_input, res)| {
    let (_, _, dimension, _, _,) = res;
    let mut lista_dimensiones = Vec::new();
    lista_dimensiones.push(dimension);
    (next_input, lista_dimensiones)
  })
}

fn dos_dimensiones(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((dimension, dimension))
  (input)
  .map(|(next_input, res)| {
    let (dimension_1, dimension_2) = res;
    let mut lista_dimensiones = Vec::new();
    lista_dimensiones.push(dimension_1[0]);
    lista_dimensiones.push(dimension_2[0]);
    (next_input, lista_dimensiones)
  })
}

pub fn ws_vec(input: &str) -> IResult<&str, Vec<&str>> {
  ws(input)
  .map(|(next_input, _res)| {
    // let mut vector = Vec::new();
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
    assert_eq!(dimension("[id]"), Ok(("", vec!["id"])));
    assert_eq!(dimension("[ id ]"), Ok(("", vec!["id"])));
    assert_eq!(dimension("[  id  ]"), Ok(("", vec!["id"])));
  }

  #[test]
  fn test_dos_dimensiones() {
    assert_eq!(dos_dimensiones("[id][id]"), Ok(("", vec!["id", "id"])));
    assert_eq!(dos_dimensiones("[ id ][ id ]"), Ok(("", vec!["id", "id"])));
    assert_eq!(dos_dimensiones("[  id  ][  id  ]"), Ok(("", vec!["id", "id"])));
  }

  #[test]
  fn test_ws_vec() {
    assert_eq!(ws_vec("aaaa"), Ok(("aaaa", vec![])));
    assert_eq!(ws_vec("bbbb"), Ok(("bbbb", vec![])));
    assert_eq!(ws_vec("cccc"), Ok(("cccc", vec![])));
  }
}