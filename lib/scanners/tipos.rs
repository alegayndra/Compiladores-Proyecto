use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult
};

use crate::scanners::id::*;

pub fn tipo(input: &str) -> IResult<&str, &str> {
  alt((tag("entero"), tag("flotante"), tag("char")))(input)
}

pub fn tipo_compuesto(input: &str) -> IResult<&str, &str> {
  alt((tipo, id))(input)
}

pub fn tipo_retorno(input: &str) -> IResult<&str, &str> {
  alt((tipo, tag("void")))(input)
}


#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

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
