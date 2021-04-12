use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::tipos::*;
use crate::scanners::id::*;

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

fn ws_vec(input: &str) -> IResult<&str, Vec<&str>> {
  ws(input)
  .map(|(next_input, res)| {
    let mut vector = Vec::new();
    vector.push("");
    (next_input, vector)
  })
}

fn con_dim(input: &str) -> IResult<&str, Vec<&str>> {
  alt((dos_dimensiones, dimension, ws_vec))
  (input)
}

fn variable_compuesta(input: &str) -> IResult<&str, (&str, &str, Vec<&str>, &str, Vec<&str>, &str, &str)> {
  tuple((
    tag("id"), ws,
    list_ids, ws,
    ws_vec, ws,
    tag(";")
  ))
  (input)
}

fn variable_normal(input: &str) -> IResult<&str, (&str, &str, Vec<&str>, &str, Vec<&str>, &str, &str)> {
  tuple((
    tipo, ws,
    list_ids, ws,
    con_dim, ws,
    tag(";")
  ))
  (input)
}

pub fn variables(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  alt((variable_compuesta, variable_normal))
  (input)
  .map(|(next_input, res)| {
    let (tipo, _, lista_ids, _, dimensiones, _, _) = res;
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

  #[test]
  fn test_con_coma() {
    assert_eq!(con_coma("id"), Ok(("", vec!["id"])));
    assert_eq!(con_coma("id, id"), Ok(("", vec!["id", "id"])));
  }
}