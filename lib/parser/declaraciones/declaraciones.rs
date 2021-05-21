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
use crate::semantica::globales::*;

fn diferentes_declaraciones(input: &str) -> IResult<&str, &str> {
  match alt((clase, funcion, variables))(input) {
    Ok((next_input, res)) => {
      // Reset de variables globales de semantica
      let mut contexto_funcion1 = CONTEXTO_FUNCION.lock().unwrap();
      *contexto_funcion1 = ID_PROGRAMA.lock().unwrap().to_string();
      
      VARIABLES.lock().unwrap().tabla.drain();
      Ok((next_input, res))
    },
    Err(err) => Err(err)
  }
}

fn lista_declaraciones(input: &str) -> IResult<&str, &str> {
  tuple((diferentes_declaraciones, many0(tuple((ws, diferentes_declaraciones)))))(input)
  .map(|(next_input, _)| {
    (next_input, "lista_declaraciones")
  })
}

pub fn declaraciones(input: &str) -> IResult<&str, &str> {
  opt(lista_declaraciones)(input)
  .map(|(next_input, _)| {
    (next_input, "declaraciones")
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
    assert_eq!(lista_declaraciones("entero id;"),                                                          Ok(("", "lista_declaraciones")));
    assert_eq!(lista_declaraciones("clase Estudiante {};"),                                                Ok(("", "lista_declaraciones")));
    assert_eq!(lista_declaraciones("void funcion func() { regresa 10; }"),                                 Ok(("", "lista_declaraciones")));
    assert_eq!(lista_declaraciones("entero id; clase Estudiante {}; void funcion func() { regresa 10; }"), Ok(("", "lista_declaraciones")));
  }

  #[test]
  fn test_declaraciones() {
    assert_eq!(declaraciones("entero id;"),                                                           Ok(("", "declaraciones")));
    assert_eq!(declaraciones("clase Estudiante {};"),                                                 Ok(("", "declaraciones")));
    assert_eq!(declaraciones("void funcion func () { regresa expresion ; }"),                         Ok(("", "declaraciones")));
    assert_eq!(declaraciones("entero id; clase Estudiante {}; void funcion func() { regresa 10; }"),  Ok(("", "declaraciones")));
    assert_eq!(declaraciones(""),                                                                     Ok(("", "declaraciones")));
    assert_eq!(declaraciones("asd"),                                                                  Ok(("asd", "declaraciones")));
  }
}
