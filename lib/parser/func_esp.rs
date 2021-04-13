use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::scanners::texto::*;
use crate::parser::dimensiones::*;

pub fn leer(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("lee"), ws, tag("("), ws, alt((lista_ids, ws_vec)), ws, tag(")")))
  (input)
  .map(|(next_input, res)| {
    let (_, _, _, _, lista_ids, _, _) = res;
    (next_input, lista_ids)
  })
}

pub fn escribir(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("escribe"), ws, tag("("), ws, many0(alt((id, texto))), ws, tag(")")))
  (input)
  .map(|(next_input, res)| {
    let (_, _, _, _, lista_valores, _, _) = res;
    (next_input, lista_valores)
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
  fn test_leer() {
    assert_eq!(leer("lee()"), Ok(("", vec![])));
    assert_eq!(leer("lee(id)"), Ok(("", vec!["id"])));
    assert_eq!(leer("lee ( id )"), Ok(("", vec!["id"])));
    assert_eq!(leer("lee ( id, id )"), Ok(("", vec!["id", "id"])));
  }

  #[test]
  fn test_escribir() {
    assert_eq!(escribir("escribe()"), Ok(("", vec![])));
    assert_eq!(escribir("escribe(id)"), Ok(("", vec!["id"])));
    assert_eq!(escribir("escribe ( id )"), Ok(("", vec!["id"])));
  }
}