use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::tipos::*;
// use crate::scanners::id::*;

pub fn variables(input: &str) -> IResult<&str, (&str, Vec<&str>, Vec<&str>, &str)> {
  tuple((
    tipo_compuesto, necessary_ws
    many0(tuple((
      ws, tag(","), 
      ws, tag("id")
    ))),
    con_dim,
    tag(";")
  ))
  (input)
  .map(|(next_input, res)| {
    let (id, ids) = res;
    let mut lista_ids = Vec::new();
    lista_ids.push(id);
    for sid in ids {
        let (_, _, _, sid2) = sid;
        lista_ids.push(sid2);
    }
    (
      next_input,
      lista_ids
    )
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
  fn test_con_coma() {
    assert_eq!(con_coma("id"), Ok(("", vec!["id"])));
    assert_eq!(con_coma("id, id"), Ok(("", vec!["id", "id"])));
  }
}