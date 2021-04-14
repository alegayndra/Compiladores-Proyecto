use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::parser::dimensiones::*;

pub fn id(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric())
  (input)
}

fn id_sin_dim(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((id, ws_vec))
  (input)
}

fn id_con_dim(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((id, con_dim))
  (input)
}

//Permite leer variables con o sin dimensiones
pub fn id_parser(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  alt((id_con_dim, id_sin_dim))(input)
}


pub fn lista_ids_sin_dim(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
  tuple((id_sin_dim, many0(tuple((ws, tag(","), ws, id_sin_dim)))))(input)
  .map(|(next_input, res)| {
    let (id, ids) = res;
    let mut lista_ids = Vec::new();
    lista_ids.push(id);
    for sid in ids {
      let (_, _, _, sid2) = sid;
      lista_ids.push(sid2);
    }
    (next_input, lista_ids)
  })
}

pub fn lista_ids_con_dim(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
  tuple((id_parser, many0(tuple((ws, tag(","), ws, id_parser)))))(input)
  .map(|(next_input, res)| {
    let (id, ids) = res;
    let mut lista_ids = Vec::new();
    lista_ids.push(id);
    for sid in ids {
      let (_, _, _, sid2) = sid;
      lista_ids.push(sid2);
    }
    (next_input, lista_ids)
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
  fn test_id() {
    assert_eq!(id("id"), Ok(("", "id")));
    assert_eq!(id("aaa123"), Ok(("", "aaa123")));
    assert_eq!(id("1aa123"), Ok(("", "1aa123")));
  }

  #[test]
  fn test_id_sin_dim() {
    assert_eq!(id_sin_dim("id"), Ok(("", ("id", vec![]))));
    assert_eq!(id_sin_dim("aaa123"), Ok(("", ("aaa123", vec![]))));
    assert_eq!(id_sin_dim("1aa123"), Ok(("", ("1aa123", vec![]))));
  }

  #[test]
  fn test_id_con_dim() {
    assert_eq!(id_con_dim("id"), Ok(("", ("id", vec![]))));
    assert_eq!(id_con_dim("id[id]"), Ok(("", ("id", vec!["id"]))));
  }

  #[test]
  fn test_id_parser() {
    assert_eq!(id_parser("id"), Ok(("", ("id", vec![]))));
    assert_eq!(id_parser("aaa123"), Ok(("", ("aaa123", vec![]))));
    assert_eq!(id_parser("1aa123"), Ok(("", ("1aa123", vec![]))));
    assert_eq!(id_parser("id[id]"), Ok(("", ("id", vec!["id"]))));
  }

  #[test]
  fn test_lista_ids_sin_dim() {
    assert_eq!(lista_ids_sin_dim("id"), Ok(("", vec![("id", vec![])])));
    assert_eq!(lista_ids_sin_dim("id, aa"), Ok(("", vec![("id", vec![]), ("aa", vec![])])));
  }

  #[test]
  fn test_lista_ids_con_dim() {
    assert_eq!(lista_ids_con_dim("id"), Ok(("", vec![("id", vec![])])));
    assert_eq!(lista_ids_con_dim("id[1]"), Ok(("", vec![("id", vec!["1"])])));
    assert_eq!(lista_ids_con_dim("id[1][2]"), Ok(("", vec![("id", vec!["1", "2"])])));
    assert_eq!(lista_ids_con_dim("id, aa"), Ok(("", vec![("id", vec![]), ("aa", vec![])])));
    assert_eq!(lista_ids_con_dim("id[1], aa"), Ok(("", vec![("id", vec!["1"]), ("aa", vec![])])));
    assert_eq!(lista_ids_con_dim("id, aa[1]"), Ok(("", vec![("id", vec![]), ("aa", vec!["1"])])));
    assert_eq!(lista_ids_con_dim("id[3][4], aa[1]"), Ok(("", vec![("id", vec!["3", "4"]), ("aa", vec!["1"])])));
  }
}
