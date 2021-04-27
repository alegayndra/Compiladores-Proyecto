use nom::{
  IResult,
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

fn lista_declaraciones<'a>(funciones: &mut TablaFunciones) -> impl FnMut(&'a str) -> IResult<&str, Vec<&str>> {
  move |input| {
    many0(diferentes_declaraciones)(input)
    .map(|(next_input, result)| (next_input, result))
  }
}

pub fn declaraciones<'a>(input: &'a str, funciones: &mut TablaFunciones) -> IResult<&'a str, Vec<&'a str>> {
  opt(lista_declaraciones(funciones))(input)
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
    assert_eq!(diferentes_declaraciones("entero id;"), Ok(("", "variables")));
    assert_eq!(diferentes_declaraciones("clase Estudiante {};"), Ok(("", "clase")));
    assert_eq!(diferentes_declaraciones("void funcion func() { regresa 10; }"), Ok(("", "funcion")));
  }

  #[test]
  fn test_lista_declaraciones() {
    let mut funciones: TablaFunciones = TablaFunciones {tabla: vec![]};
    assert_eq!(lista_declaraciones(&mut funciones.clone())("entero id;"), Ok(("", vec!["variables"])));
    assert_eq!(lista_declaraciones(&mut funciones.clone())("clase Estudiante {};"), Ok(("", vec!["clase"])));
    assert_eq!(lista_declaraciones(&mut funciones.clone())("void funcion func() { regresa 10; }"), Ok(("", vec!["funcion"])));
    assert_eq!(lista_declaraciones(&mut funciones.clone())("entero id; clase Estudiante {}; void funcion func() { regresa 10; }"), Ok(("", vec!["variables", "clase", "funcion"])));
  }

  #[test]
  fn test_declaraciones() {
    let mut funciones: TablaFunciones = TablaFunciones {tabla: vec![]};
    assert_eq!(declaraciones("entero id;", &mut funciones.clone()),                                                           Ok(("", vec!["variables"])));
    assert_eq!(declaraciones("clase Estudiante {};", &mut funciones.clone()),                                                 Ok(("", vec!["clase"])));
    assert_eq!(declaraciones("void funcion func () { regresa expresion ; }", &mut funciones.clone()),                         Ok(("", vec!["funcion"])));
    assert_eq!(declaraciones("entero id; clase Estudiante {}; void funcion func() { regresa 10; }", &mut funciones.clone()), Ok(("", vec!["variables", "clase", "funcion"])));
    assert_eq!(declaraciones("", &mut funciones.clone()),                                                                     Ok(("", vec![])));
    assert_eq!(declaraciones("asd", &mut funciones.clone()),                                                                  Ok(("asd", vec![])));
  }
}
