//! Módulo que se encarga del _scannear_ los espacios vacíos.

use nom::{
  bytes::complete::{take_while1, take_while},
  IResult,
};

/// Scanner para leer espacíos vacíos opcionales.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match ws("") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn ws(input: &str) -> IResult<&str, &str> {
  take_while(|c: char| c == ' ' || c == '\n' || c == '\t' || c == '\r')(input)
}

/// Scanner para leer al menos un espacío vacío.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match necessary_ws(" ") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn necessary_ws(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c == ' ' || c == '\n' || c == '\t' || c == '\r')(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ws() {
      assert_eq!(ws(""), Ok(("", "")));
      assert_eq!(ws("  "), Ok(("", "  ")));
      assert_eq!(ws("\n"), Ok(("", "\n")));
      assert_eq!(ws("\n   \t"), Ok(("", "\n   \t")));
      assert_eq!(ws("a"), Ok(("a", "")));
  }

  #[test]
  fn test_necessary_ws() {
      assert_eq!(necessary_ws("  "), Ok(("", "  ")));
      assert_eq!(necessary_ws(" "), Ok(("", " ")));
      assert_eq!(necessary_ws("\n"), Ok(("", "\n")));
      assert_eq!(necessary_ws("\n   \t"), Ok(("", "\n   \t")));
      assert_eq!(necessary_ws(" a"), Ok(("a", " ")));
  }
}
