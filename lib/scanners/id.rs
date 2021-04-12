use nom::{
  bytes::complete::{tag, take_while1},
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;

pub fn id(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric())(input)
}

pub fn list_ids(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("id"), many0(tuple((ws, tag(","), ws, tag("id"))))))(input)
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
  }

  #[test]
  fn test_list_ids() {
    assert_eq!(list_ids("id"), Ok(("", vec!["id"])));
    assert_eq!(list_ids("id, id"), Ok(("", vec!["id", "id"])));
  }
}