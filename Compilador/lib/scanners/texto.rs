use nom::{
  bytes::complete::{tag, take_while},
  IResult,
  sequence::{delimited},
};

/// Scanner para leer textos constantes.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input`- Input a parsear
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
  fn test_texto() {
    assert_eq!(texto("\"a\""),      Ok(("", "a")));
  }
}
