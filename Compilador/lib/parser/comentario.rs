//! Módulo que se encarga de los comentarios.

use nom::{
  bytes::complete::{tag, take_while},
  IResult,
  sequence::tuple,
};

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
pub fn comentario(input: &str) -> IResult<&str, &str> {
  tuple((tag("%%"), take_while(|c: char| c != '%'), tag("%%")))
  (input)
  .map(|(next_input, _)| {
    (next_input, "comentario")
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comentario() {
    assert_eq!(comentario("%%%%"),      Ok(("", "comentario")));
    assert_eq!(comentario("%%  %%"),    Ok(("", "comentario")));
    assert_eq!(comentario("%% aaa %%"), Ok(("", "comentario")));
    assert_eq!(comentario("%%
      aaa
    %%"), Ok(("", "comentario")));
  }
}
