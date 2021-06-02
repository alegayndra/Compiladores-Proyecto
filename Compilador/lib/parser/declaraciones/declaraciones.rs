//! Módulo que se encarga de las diferentes declaraciones.

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

/// Función auxiliar para todos las diferentes declaraciones.
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match diferentes_declaraciones("entero nombre;") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn diferentes_declaraciones(input: &str) -> IResult<&str, &str> {
  match alt((clase, funcion, variables))(input) {
    Ok((next_input, res)) => {
      // Resetea las variables globales
      let mut contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();
      *contexto_funcion = ID_PROGRAMA.lock().unwrap().to_string();
      let mut contexto_clase = CONTEXTO_CLASE.lock().unwrap();
      *contexto_clase = "".to_owned();

      resetear_direcciones_locales();
      
      VARIABLES.lock().unwrap().tabla.drain();
      
      unsafe {
        RETURN_EXISTENTE = false;
      }
      Ok((next_input, res))
    },
    Err(err) => Err(err)
  }
}

/// Función auxiliar para tener una lista de declaraciones.
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match lista_declaraciones("entero nombre; void funcion hola() {} clase Persona {}") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn lista_declaraciones(input: &str) -> IResult<&str, &str> {
  tuple((diferentes_declaraciones, many0(tuple((ws, diferentes_declaraciones)))))(input)
  .map(|(next_input, _)| {
    (next_input, "lista_declaraciones")
  })
}

/// No terminal de las diferentes declaraciones (clases, funciones y variables) que existen.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// CLASE | FUNCION | VARIABLES 
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match declaraciones("entero nombre;") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn declaraciones(input: &str) -> IResult<&str, &str> {
  opt(lista_declaraciones)(input)
  .map(|(next_input, _)| {
    (next_input, "declaraciones")
  })
}

#[cfg(test)]
mod tests {
  use super::*;

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
