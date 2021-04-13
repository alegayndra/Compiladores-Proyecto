use nom::{
  bytes::complete::{tag, take_while1},
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;

pub fn id(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric())
  (input)
}

pub fn lista_ids(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((id, many0(tuple((ws, tag(","), ws, id)))))(input)
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
  fn test_lista_ids() {
    assert_eq!(lista_ids("id"), Ok(("", vec!["id"])));
    assert_eq!(lista_ids("id, aa"), Ok(("", vec!["id", "aa"])));
  }
}