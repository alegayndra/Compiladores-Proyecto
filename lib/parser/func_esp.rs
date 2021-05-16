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
use crate::parser::reglas_expresion::expresion::*;

// pub fn leer(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
pub fn leer(input: &str) -> IResult<&str, &str> {
  tuple((tag("lee"), ws, tag("("), ws, lista_ids, ws, tag(")"), tag(";")))
  (input)
  .map(|(next_input, _res)| {
    (next_input, "leer")
  })
}

// fn expresion_escribe(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
fn expresion_escribe(input: &str) -> IResult<&str, &str> {
  // alt((expresion, cte_texto))(input)
  alt((expresion, texto))(input)
}

// fn lista_textos(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
fn lista_textos(input: &str) -> IResult<&str, Vec<&str>> {
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

// pub fn escribir(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
// pub fn escribir(input: &str) -> IResult<&str, Vec<&str>> {
pub fn escribir(input: &str) -> IResult<&str, &str> {
  tuple((tag("escribe"), ws, tag("("), ws, lista_textos, ws, tag(")"), tag(";")))
  (input)
  .map(|(next_input, _res)| {
    // let (_, _, _, _, lista, _, _, _) = res;
    // (next_input, lista)
    (next_input, "escribir")
  })
}


// pub fn funcion_esp(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
pub fn funcion_esp(input: &str) -> IResult<&str, &str> {
  alt((leer, escribir))(input)
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
    // assert_eq!(leer("lee(id);"), Ok(("", vec![("id", vec![])])));
    // assert_eq!(leer("lee ( id );"), Ok(("", vec![("id", vec![])])));
    // assert_eq!(leer("lee ( id, id );"), Ok(("", vec![("id", vec![]), ("id", vec![])])));
    // assert_eq!(leer("lee()"), Ok(("", vec![])));

    assert_eq!(leer("lee(id);"),        Ok(("", "leer")));
    assert_eq!(leer("lee ( id );"),     Ok(("", "leer")));
    assert_eq!(leer("lee ( id, id );"), Ok(("", "leer")));
  }

  #[test]
  fn test_escribir() {
    // assert_eq!(escribir("escribe(id);"), Ok(("", vec![("id", vec![])])));
    // assert_eq!(escribir("escribe(\"abr\");"), Ok(("", vec![("abr", vec![])])));
    // assert_eq!(escribir("escribe ( id );"), Ok(("", vec![("id", vec![])])));
    // assert_eq!(escribir("escribe(\"abr\", id, id, \"abr\");"), Ok(("", vec![("abr", vec![]),("id", vec![]),("id", vec![]),("abr", vec![])])));

    assert_eq!(escribir("escribe(id);"),                        Ok(("", "escribir")));
    assert_eq!(escribir("escribe(\"abr\");"),                   Ok(("", "escribir")));
    assert_eq!(escribir("escribe ( id );"),                     Ok(("", "escribir")));
    assert_eq!(escribir("escribe(\"abr\", id, id, \"abr\");"),  Ok(("", "escribir")));
  }
}
