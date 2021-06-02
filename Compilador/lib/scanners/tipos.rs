//! Módulo que se encarga del _scannear_ los diferentes tipos.

use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult
};

use crate::scanners::id::*;

/// Scanner para leer palabras reservadas de tipos primitivos.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match tipo("entero") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn tipo(input: &str) -> IResult<&str, &str> {
  alt((tag("entero"), tag("flotante"), tag("char")))(input)
}

/// Scanner para leer tipos compuestas.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match tipo_compuesto("Persona") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn tipo_compuesto(input: &str) -> IResult<&str, &str> {
  alt((tipo, id))(input)
}

/// Scanner para leer tipos de retorno.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match tipo_retorno("void") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn tipo_retorno(input: &str) -> IResult<&str, &str> {
  alt((tipo, tag("void")))(input)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tipo() {
    assert_eq!(tipo("entero"),   Ok(("", "entero")));
    assert_eq!(tipo("flotante"), Ok(("", "flotante")));
    assert_eq!(tipo("char"),     Ok(("", "char")));
  }

  #[test]
  fn test_tipo_compuesto() {
    assert_eq!(tipo_compuesto("entero"),      Ok(("", "entero")));
    assert_eq!(tipo_compuesto("flotante"),    Ok(("", "flotante")));
    assert_eq!(tipo_compuesto("char"),        Ok(("", "char")));
    assert_eq!(tipo_compuesto("soyUnObjeto"), Ok(("", "soyUnObjeto")));
  }

  #[test]
  fn test_tipo_retorno() {
    assert_eq!(tipo_retorno("entero"),   Ok(("", "entero")));
    assert_eq!(tipo_retorno("flotante"), Ok(("", "flotante")));
    assert_eq!(tipo_retorno("char"),     Ok(("", "char")));
    assert_eq!(tipo_retorno("void"),     Ok(("", "void")));
  }
}
