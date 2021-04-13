use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::tipos::*;
use crate::scanners::id::*;
use crate::parser::dimensiones::*;

fn variable_compuesta(input: &str) -> IResult<&str, (&str, &str, Vec<&str>, &str, Vec<&str>, &str, &str)> {
  tuple((
    id, ws,
    lista_ids, ws,
    ws_vec, ws,
    tag(";")
  ))
  (input)
}

fn variable_normal(input: &str) -> IResult<&str, (&str, &str, Vec<&str>, &str, Vec<&str>, &str, &str)> {
  tuple((
    tipo, ws,
    lista_ids, ws,
    con_dim, ws,
    tag(";")
  ))
  (input)
}

pub fn variables(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  alt((variable_compuesta, variable_normal))
  (input)
  .map(|(next_input, res)| {
    let (tipo, _, lista_ids, _, _dimensiones, _, _) = res;
    (next_input, (tipo, lista_ids))
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  // #[test]
  // fn test_variable_compuesta() {
  //   assert_eq!(variable_compuesta("id id;"), Ok(("", ("id", vec!["id"]))));
  //   assert_eq!(variable_compuesta("id id, id;"), Ok(("", ("id", vec!["id", "id"]))));
  // }

  #[test]
  fn test_variables() {
    assert_eq!(variables("Persona id;"), Ok(("",        ("Persona", vec!["id"]))));
    assert_eq!(variables("Persona id, id;"), Ok(("",    ("Persona", vec!["id", "id"]))));
    assert_eq!(variables("entero id;"), Ok(("",         ("entero",  vec!["id"]))));
    assert_eq!(variables("entero id[id];"), Ok(("",     ("entero",  vec!["id"]))));
    assert_eq!(variables("entero id[id][id];"), Ok(("", ("entero",  vec!["id"]))));
    assert_eq!(variables("entero id, id;"), Ok(("",     ("entero",  vec!["id", "id"]))));
  }
}