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

fn dimension(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((
    tag("["), ws,
    tag("id"), ws,
    tag("}")
  ))
  (input)
  .map(|(next_input, res)| {
    let (_, _, dimension, _, _,) = res;
    let mut lista_dimensiones = Vec::new();
    lista_dimensiones.push(dimension);
    (
      next_input,
      lista_dimensiones
    )
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
    (
      next_input,
      lista_dimensiones
    )
  })
}

fn con_dim(input: &str) -> IResult<&str, Vec<&str>> {
  alt((dimension, dos_dimensiones))
  (input)
}

pub fn variables(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((
    tipo_compuesto, necessary_ws,
    tag("id"),
    many0(tuple((
      ws, tag(","), 
      ws, tag("id")
    ))), ws,
    // con_dim, ws,
    tag(";")
  ))
  (input)
  .map(|(next_input, res)| {
    // let (tipo, _, id, ids, _, dimensiones, _, semicolon) = res;
    let (tipo, _, id, ids, _, _) = res;
    let mut lista_ids = Vec::new();
    lista_ids.push(id);
    for sid in ids {
        let (_, _, _, sid2) = sid;
        lista_ids.push(sid2);
    }
    (
      next_input,
      (
        tipo,
        lista_ids
      )
    )
  })
}

// pub fn variables(input: &str) -> IResult<&str, (&str, Vec<&str>, Vec<&str>, &str)> {
//   tuple((
//     tipo_compuesto, necessary_ws
//     many0(tuple((
//       ws, tag(","), 
//       ws, tag("id")
//     ))),
//     tag((dos_dimensiones, dimension, ws)),
//     tag(";")
//   ))
//   (input)
//   .map(|(next_input, res)| {
//     let (id, ids) = res;
//     let mut lista_ids = Vec::new();
//     lista_ids.push(id);
//     for sid in ids {
//         let (_, _, _, sid2) = sid;
//         lista_ids.push(sid2);
//     }
//     (
//       next_input,
//       lista_ids
//     )
//   })
// }

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