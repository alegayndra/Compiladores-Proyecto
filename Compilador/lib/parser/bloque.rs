//! Módulo que se encarga de los diferentes bloques.

use nom::{
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
  branch::alt,
};

use crate::scanners::ws::*;
use crate::parser::estatuto::*;
use crate::parser::declaraciones::variables::*;

/// Función auxiliar para la lista de los diferentes estatutos del bloque;  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input`- Input a parsear
///
/// # Ejemplo
///
/// ```
/// match lista_estatutos("num = 10; %% comentario %%") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn lista_estatutos(input: &str) -> IResult<&str, &str> {
  many0(tuple((estatuto, ws)))(input)
  .map(|(next_input, _)| {
    (next_input, "lista_estatutos")
  })
}

/// No terminal de bloque.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input`- Input a parsear
///
/// # Gramática
///
/// ```
/// { ESTATUTOS }
/// ```
///
/// # Ejemplo
///
/// ```
/// match bloque("{ num = 10; }") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn bloque(input: &str) -> IResult<&str, &str> {
  tuple((tag("{"), ws, lista_estatutos, ws, tag("}")))(input)
  .map(|(next_input, _)| {
    (next_input, "bloque")
  })
}


/// Función auxiliar para la lista de los diferentes estatutos del bloque_función;  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input`- Input a parsear
///
/// # Ejemplo
///
/// ```
/// match lista_estatutos_funcion("num = 10; %% comentario %%") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn lista_estatutos_funcion(input: &str) -> IResult<&str, &str> {
  many0(tuple((alt((estatuto, variables)), ws)))(input)
  .map(|(next_input, _)| {
    (next_input, "lista_estatutos")
  })
}

/// No terminal de bloque_funcion. Es lo mismo que el no terminal bloque pero con el agregado de que se pueden definir variables;  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input`- Input a parsear
///
/// # Gramática
///
/// ```
/// { ESTATUTOS }
/// ```
///
/// # Ejemplo
///
/// ```
/// match bloque("{ num = 10; }") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn bloque_funcion(input: &str) -> IResult<&str, &str> {
  tuple((tag("{"), ws, lista_estatutos_funcion, ws, tag("}")))(input)
  .map(|(next_input, _)| {
    (next_input, "bloque")
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lista_estatutos() {
    assert_eq!(lista_estatutos(""),                    Ok(("", "lista_estatutos")));
    assert_eq!(lista_estatutos("%% un comentario %%"), Ok(("", "lista_estatutos")));
  }

  #[test]
  fn test_bloque() {
    assert_eq!(bloque("{}"),                      Ok(("", "bloque")));
    assert_eq!(bloque("{  }"),                    Ok(("", "bloque")));
    assert_eq!(bloque("{ %% un comentario %% }"), Ok(("", "bloque")));
  }
}