use nom::{
  IResult,
  sequence::tuple,
  combinator::opt,
  branch::alt,
  multi::many0
};

use crate::scanners::ws::*;
use crate::parser::declaraciones::clase::*;
use crate::parser::declaraciones::funcion::*;
use crate::parser::declaraciones::variables::*;
use crate::semantica::tabla_funciones::*;

fn diferentes_declaraciones(input: &str) -> IResult<&str, &str> {
  alt((clase, funcion, variables))(input)
}

fn lista_declaraciones(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((diferentes_declaraciones, many0(tuple((ws, diferentes_declaraciones)))))(input)
  .map(|(next_input, res)| {
    let (decl, declaraciones) = res;
    let mut lista = Vec::new();
    lista.push(decl);
    for d in declaraciones {
      let (_, de) = d;
      lista.push(de);
    }
    (next_input, lista)
  })
}

pub fn declaraciones(input: &str) -> IResult<&str, Vec<&str>> {
  opt(lista_declaraciones)(input)
  .map(|(next_input, res)| {
    (next_input, res.unwrap_or(vec![]))
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
  fn test_diferentes_declaraciones() {
    assert_eq!(diferentes_declaraciones("entero id;"),                          Ok(("", "variables")));
    assert_eq!(diferentes_declaraciones("clase Estudiante {};"),                Ok(("", "clase")));
    assert_eq!(diferentes_declaraciones("void funcion func() { regresa 10; }"), Ok(("", "funcion")));
  }

  #[test]
  fn test_lista_declaraciones() {
    assert_eq!(lista_declaraciones("entero id;"),                                                          Ok(("", vec!["variables"])));
    assert_eq!(lista_declaraciones("clase Estudiante {};"),                                                Ok(("", vec!["clase"])));
    assert_eq!(lista_declaraciones("void funcion func() { regresa 10; }"),                                 Ok(("", vec!["funcion"])));
    assert_eq!(lista_declaraciones("entero id; clase Estudiante {}; void funcion func() { regresa 10; }"), Ok(("", vec!["variables", "clase", "funcion"])));
  }

  #[test]
  fn test_declaraciones() {
    assert_eq!(declaraciones("entero id;"),                                                           Ok(("", vec!["variables"])));
    assert_eq!(declaraciones("clase Estudiante {};"),                                                 Ok(("", vec!["clase"])));
    assert_eq!(declaraciones("void funcion func () { regresa expresion ; }"),                         Ok(("", vec!["funcion"])));
    assert_eq!(declaraciones("entero id; clase Estudiante {}; void funcion func() { regresa 10; }"),  Ok(("", vec!["variables", "clase", "funcion"])));
    assert_eq!(declaraciones(""),                                                                     Ok(("", vec![])));
    assert_eq!(declaraciones("asd"),                                                                  Ok(("asd", vec![])));
  }
}
