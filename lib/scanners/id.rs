use nom::{
  bytes::complete::{tag, take_while1},
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::parser::dimensiones::*;
use crate::parser::dimensiones_decl::*;

pub fn id(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '-')
  (input)
}

pub fn id_sin_dim(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((id, ws_vec))
  (input)
}

pub fn id_con_dim_decl(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((id, con_dim_decl))
  (input)
}

pub fn id_con_dim(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((id, con_dim))
  (input)
}

pub fn lista_ids(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
  tuple((id_con_dim, many0(tuple((ws, tag(","), ws, id_con_dim)))))(input)
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
  tuple((id_con_dim_decl, many0(tuple((ws, tag(","), ws, id_con_dim_decl)))))(input)
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

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_id() {
    assert_eq!(id("id"),        Ok(("", "id")));
    assert_eq!(id("id["),       Ok(("[", "id")));
    assert_eq!(id("aaa123"),    Ok(("", "aaa123")));
    assert_eq!(id("1aa123"),    Ok(("", "1aa123")));
    assert_eq!(id("1aa_123"),   Ok(("", "1aa_123")));
    assert_eq!(id("1aa_123  "), Ok(("  ", "1aa_123")));
    assert_eq!(id("1aa_ 123"),  Ok((" 123", "1aa_")));
  }

  #[test]
  fn test_id_sin_dim() {
    assert_eq!(id_sin_dim("id"),     Ok(("", ("id", vec![]))));
    assert_eq!(id_sin_dim("aaa123"), Ok(("", ("aaa123", vec![]))));
    assert_eq!(id_sin_dim("1aa123"), Ok(("", ("1aa123", vec![]))));
  }

  #[test]
  fn test_id_con_dim() {
    assert_eq!(id_con_dim("id"),     Ok(("", ("id", vec![]))));
    assert_eq!(id_con_dim("id[id]"), Ok(("", ("id", vec!["exp"]))));
  }
  
  #[test]
  fn test_lista_ids_sin_dim() {
    assert_eq!(lista_ids_sin_dim("id"),     Ok(("", vec![("id", vec![])])));
    assert_eq!(lista_ids_sin_dim("id, aa"), Ok(("", vec![("id", vec![]), ("aa", vec![])])));
  }

  #[test]
  fn test_lista_ids_con_dim() {
    assert_eq!(lista_ids_con_dim("id"),               Ok(("", vec![("id", vec![])])));
    assert_eq!(lista_ids_con_dim("id[1]"),            Ok(("", vec![("id", vec!["1"])])));
    assert_eq!(lista_ids_con_dim("id[1][2]"),         Ok(("", vec![("id", vec!["1", "2"])])));
    assert_eq!(lista_ids_con_dim("id, aa"),           Ok(("", vec![("id", vec![]), ("aa", vec![])])));
    assert_eq!(lista_ids_con_dim("id[1], aa"),        Ok(("", vec![("id", vec!["1"]), ("aa", vec![])])));
    assert_eq!(lista_ids_con_dim("id, aa[1]"),        Ok(("", vec![("id", vec![]), ("aa", vec!["1"])])));
    assert_eq!(lista_ids_con_dim("id[3][4], aa[1]"),  Ok(("", vec![("id", vec!["3", "4"]), ("aa", vec!["1"])])));
  }

  #[test]
  fn test_lista_ids() {
    assert_eq!(lista_ids("id"),               Ok(("", vec![("id", vec![])])));
    assert_eq!(lista_ids("id[1]"),            Ok(("", vec![("id", vec!["exp"])])));
    assert_eq!(lista_ids("id[1][2]"),         Ok(("", vec![("id", vec!["exp", "exp"])])));
    assert_eq!(lista_ids("id, aa"),           Ok(("", vec![("id", vec![]), ("aa", vec![])])));
    assert_eq!(lista_ids("id[1], aa"),        Ok(("", vec![("id", vec!["exp"]), ("aa", vec![])])));
    assert_eq!(lista_ids("id, aa[1]"),        Ok(("", vec![("id", vec![]), ("aa", vec!["exp"])])));
    assert_eq!(lista_ids("id[3][4], aa[1]"),  Ok(("", vec![("id", vec!["exp", "exp"]), ("aa", vec!["exp"])])));
  }
}
