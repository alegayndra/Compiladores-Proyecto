//! Módulo que se encarga del _scannear_los diferentes operadores.

use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult
};

/// Scanner para leer operadores de suma y resta.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match op_sumsub("+") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn op_sumsub(input: &str) -> IResult<&str, &str> {
  alt((tag("+"), tag("-")))(input)
}

/// Scanner para leer operadores de multiplicación y división.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match op_multdiv("*") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn op_multdiv(input: &str) -> IResult<&str, &str> {
  alt((tag("/"), tag("*")))(input)
}

/// Scanner para leer operadores de relacional.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match op_relacional(">") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn op_relacional(input: &str) -> IResult<&str, &str> {
  alt((tag("<="), tag("=="), tag(">="), tag("!="), tag("<"), tag(">")))(input)
}

/// Scanner para leer operadores de lógico.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match op_logica("&") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn op_logica(input: &str) -> IResult<&str, &str> {
  alt((tag("&"), tag("|")))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_op_sumsub() {
      assert_eq!(op_sumsub("+"), Ok(("", "+")));
      assert_eq!(op_sumsub("-"), Ok(("", "-")));
  }

  #[test]
  fn test_op_multdiv() {
      assert_eq!(op_multdiv("/"), Ok(("", "/")));
      assert_eq!(op_multdiv("*"), Ok(("", "*")));
  }

  #[test]
  fn test_op_relacional() {
      assert_eq!(op_relacional("<="), Ok(("", "<=")));
      assert_eq!(op_relacional(">="), Ok(("", ">=")));
      assert_eq!(op_relacional("=="), Ok(("", "==")));
      assert_eq!(op_relacional("!="), Ok(("", "!=")));
      assert_eq!(op_relacional(">"), Ok(("", ">")));
      assert_eq!(op_relacional("<"), Ok(("", "<")));
  }

  #[test]
  fn test_op_logica() {
      assert_eq!(op_logica("&"), Ok(("", "&")));
      assert_eq!(op_logica("|"), Ok(("", "|")));
  }
}
