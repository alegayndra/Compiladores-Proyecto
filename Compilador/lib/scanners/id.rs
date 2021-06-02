//! Módulo que se encarga del _scannear_ los ids.

use nom::{
  bytes::complete::take_while1,
  IResult,
  sequence::tuple,
};

use crate::parser::dimensiones::*;

/// Scanner para leer ids;  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match id("id_variable") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn id(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

/// Función auxiliar para leer declaraciones de ids con dimensiones;  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match id_con_dim_decl("id[10][10]") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn id_con_dim_decl(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((id, con_dim_decl))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_id() {
    assert_eq!(id("id"),        Ok(("", "id")));
    assert_eq!(id("id["),       Ok(("[", "id")));
    assert_eq!(id("aaa123"),    Ok(("", "aaa123")));
    assert_eq!(id("1aa123"),    Ok(("", "1aa123")));
    assert_eq!(id("1aa_123"),   Ok(("", "1aa_123")));
    assert_eq!(id("1aa_123  "), Ok(("  ", "1aa_123")));
    assert_eq!(id("1aa_ 123"),  Ok((" 123", "1aa_")));
  }

  #[test]
  fn test_id_con_dim_decl() {
    assert_eq!(id_con_dim_decl("id"),     Ok(("", ("id", vec![]))));
    assert_eq!(id_con_dim_decl("id[10]"), Ok(("", ("id", vec!["10"]))));
    assert_eq!(id_con_dim_decl("id[10][20]"), Ok(("", ("id", vec!["10", "20"]))));
  }
}
