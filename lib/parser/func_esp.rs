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

pub fn leer(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
  tuple((tag("lee"), ws, tag("("), ws, lista_ids_con_dim, ws, tag(")"), tag(";")))
  (input)
  .map(|(next_input, res)| {
    let (_, _, _, _, list_ids, _, _, _) = res;
    (next_input, list_ids)
  })
}

fn cte_texto(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  texto(input)
  .map(|(next_input, res)| {
    (next_input, (res, vec![]))
  })
}

fn expresion_escribe(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  alt((expresion, cte_texto))(input)
}

fn lista_textos(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
  tuple((expresion_escribe, many0(tuple((ws, tag(","), ws, expresion_escribe)))))
  (input)
  .map(|(next_input, res)| {
    let (expresion_escribe, lista_val) = res;
    let mut lista = Vec::new();
    lista.push(expresion_escribe);
    for val in lista_val {
      let (_, _, _, value) = val;
      lista.push(value);
    }
    (next_input, lista)
  })
}

pub fn escribir(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
  tuple((tag("escribe"), ws, tag("("), ws, lista_textos, ws, tag(")"), tag(";")))
  (input)
  .map(|(next_input, res)| {
    let (_, _, _, _, lista, _, _, _) = res;
    (next_input, lista)
  })
}


// pub fn funcion_esp(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
pub fn funcion_esp(input: &str) -> IResult<&str, &str> {
  alt((leer,escribir))(input)
  .map(|(next_input, _res)| {
    (next_input, "funcion_esp")
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
    assert_eq!(leer("lee(id);"), Ok(("", vec![("id", vec![])])));
    assert_eq!(leer("lee ( id );"), Ok(("", vec![("id", vec![])])));
    assert_eq!(leer("lee ( id, id );"), Ok(("", vec![("id", vec![]), ("id", vec![])])));
    // assert_eq!(leer("lee()"), Ok(("", vec![])));
  }

  #[test]
  fn test_escribir() {
    assert_eq!(escribir("escribe(id);"), Ok(("", vec![("id", vec![])])));
    assert_eq!(escribir("escribe(\"abr\");"), Ok(("", vec![("abr", vec![])])));
    assert_eq!(escribir("escribe ( id );"), Ok(("", vec![("id", vec![])])));
    assert_eq!(escribir("escribe(\"abr\", id, id, \"abr\");"), Ok(("", vec![("abr", vec![]),("id", vec![]),("id", vec![]),("abr", vec![])])));
    // assert_eq!(escribir("escribe()"), Ok(("", vec![])));
  }
}
