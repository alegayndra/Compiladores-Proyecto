use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::constantes::*;

pub fn dimension_decl(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("["), ws, num_entero, ws, tag("]")))(input)
  .map(|(next_input, res)| {
    let (_, _, dimension, _, _) = res;
    (next_input, vec![dimension])
  })
}

fn dos_dimensiones_decl(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((dimension_decl, ws, dimension_decl))
  (input)
  .map(|(next_input, res)| {
    let (dimension_1, _, dimension_2) = res;
    (next_input, vec![dimension_1[0], dimension_2[0]])
  })
}

fn ws_vec_decl(input: &str) -> IResult<&str, Vec<&str>> {
  Ok((input, vec![]))
}

pub fn con_dim_decl(input: &str) -> IResult<&str, Vec<&str>> {
  alt((dos_dimensiones_decl, dimension_decl, ws_vec_decl))
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
  fn test_dimension_decl() {
    // assert_eq!(dimension("[id]"), Ok(("", vec!["id"])));
    // assert_eq!(dimension("[ id ]"), Ok(("", vec!["id"])));
    // assert_eq!(dimension("[  id  ]"), Ok(("", vec!["id"])));

    assert_eq!(dimension_decl("[1 ]"),     Ok(("", vec!["1"])));
    assert_eq!(dimension_decl("[ 78]"),   Ok(("", vec!["78"])));
    assert_eq!(dimension_decl("[  69  ]"), Ok(("", vec!["69"])));
  }

  #[test]
  fn test_dos_dimensiones_decl() {
    // assert_eq!(dos_dimensiones("[id][id]"), Ok(("", vec!["id", "id"])));
    // assert_eq!(dos_dimensiones("[ id ][ id ]"), Ok(("", vec!["id", "id"])));
    // assert_eq!(dos_dimensiones("[  id  ][  id  ]"), Ok(("", vec!["id", "id"])));

    assert_eq!(dos_dimensiones_decl("[420][2]"),         Ok(("", vec!["420", "2"])));
    assert_eq!(dos_dimensiones_decl("[ 69666][ 0 ]"),     Ok(("", vec!["69666", "0"])));
    assert_eq!(dos_dimensiones_decl("[  1  ][   2 ]"), Ok(("", vec!["1", "2"])));
  }

  #[test]
  fn test_ws_vec_decl() {
    assert_eq!(ws_vec_decl("aaaa"), Ok(("aaaa", vec![])));
    assert_eq!(ws_vec_decl("bbbb"), Ok(("bbbb", vec![])));
    assert_eq!(ws_vec_decl("cccc"), Ok(("cccc", vec![])));
    assert_eq!(ws_vec_decl("    "), Ok(("    ", vec![])));
  }

  #[test]
  fn test_con_dim_decl() {
    // assert_eq!(con_dim("[id]"), Ok(("", vec!["id"])));
    // assert_eq!(con_dim("[id][id]"), Ok(("", vec!["id", "id"])));
    // assert_eq!(con_dim("aaaa"), Ok(("aaaa", vec![])));

    assert_eq!(con_dim_decl("[7]"), Ok(("", vec!["7"])));
    assert_eq!(con_dim_decl("[3][13]"), Ok(("", vec!["3", "13"])));
    assert_eq!(con_dim_decl("aaaa"), Ok(("aaaa", vec![])));
  }
}
