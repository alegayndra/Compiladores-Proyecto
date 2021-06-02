//! Módulo que se encarga del _scannear_ las diferentes constantes.

use nom::{
  IResult,
  combinator::{recognize, opt},
  multi::{many0, many1},
  character::complete::{one_of, char},
  sequence::{terminated, tuple, delimited},
  branch::alt,
  bytes::complete::{tag, take_while_m_n, take_while}
};

/// Scanner para leer números enteros constantes.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match num_entero("100") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn num_entero(input: &str) -> IResult<&str, (&str, &str)> {
  recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
  .map(|(next_input, res)| {
    (next_input, (res, "entero"))
  })
}

/// Scanner para leer un carácter.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match caracter("\"a\"") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn caracter(input: &str) -> IResult<&str, (&str, &str)> {
  delimited(tag("\""), take_while_m_n(1, 1, |c| c != ' '), tag("\""))(input)
  .map(|(next_input, res)| {
    (next_input, (res, "char"))
  })
}

/// Scanner para leer números flotantes constantes.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match num_flotante("10.1") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn num_flotante(input: &str) -> IResult<&str, (&str, &str)> {
  alt((
    // Case one: .42
    recognize(
      tuple((
        opt(tag("0")),
        char('.'),
        num_entero
      ))
    ), // Case three: 42. and 42.42
    recognize(
      tuple((
        num_entero,
        char('.'),
        num_entero
      ))
    )
  ))(input)
  .map(|(next_input, res)| {
    (next_input, (res, "flotante"))
  })
}

/// Scanner para leer textos constantes.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match texto("\"texto\"") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn texto(input: &str) -> IResult<&str, &str> {
  match delimited(tag("\""), take_while(|c: char| c.is_alphanumeric()), tag("\""))(input) {
    Ok(res) => Ok(res),
    Err(err) => Err(err)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_num_caracter() {
    assert_eq!(caracter("\"a\""), Ok(("", ("a", "char"))));
    assert_eq!(caracter("\"-\""), Ok(("", ("-", "char"))));
  }

  #[test]
  fn test_num_entero() {
    assert_eq!(num_entero("1"),       Ok(("", ("1", "entero"))));
    assert_eq!(num_entero("11"),      Ok(("", ("11", "entero"))));
    assert_eq!(num_entero("1123131"), Ok(("", ("1123131", "entero"))));
  }

  #[test]
  fn test_num_flotante() {
    assert_eq!(num_flotante("1.1"),       Ok(("", ("1.1", "flotante"))));
    assert_eq!(num_flotante("11.23"),     Ok(("", ("11.23", "flotante"))));
    assert_eq!(num_flotante("112.3131"),  Ok(("", ("112.3131", "flotante"))));
    assert_eq!(num_flotante("0.3131"),    Ok(("", ("0.3131", "flotante"))));
    assert_eq!(num_flotante(".3131"),     Ok(("", (".3131", "flotante"))));
  }

  #[test]
  fn test_texto() {
    assert_eq!(texto("\"a\""),      Ok(("", "a")));
  }
}
