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

pub fn lista_estatutos(input: &str) -> IResult<&str, &str> {
  many0(tuple((estatuto, ws)))(input)
  .map(|(next_input, _)| {
    (next_input, "lista_estatutos")
  })
}

pub fn bloque(input: &str) -> IResult<&str, &str> {
  tuple((tag("{"), ws, lista_estatutos, ws, tag("}")))(input)
  .map(|(next_input, _)| {
    (next_input, "bloque")
  })
}

pub fn lista_estatutos_funcion(input: &str) -> IResult<&str, &str> {
  many0(tuple((alt((estatuto, variables)), ws)))(input)
  .map(|(next_input, _)| {
    (next_input, "lista_estatutos")
  })
}

pub fn bloque_funcion(input: &str) -> IResult<&str, &str> {
  tuple((tag("{"), ws, lista_estatutos_funcion, ws, tag("}")))(input)
  .map(|(next_input, _)| {
    (next_input, "bloque")
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_lista_estatutos() {
    assert_eq!(lista_estatutos(""), Ok(("", "lista_estatutos")));
    assert_eq!(lista_estatutos("%% un comentario %%"), Ok(("", "lista_estatutos")));
    // assert_eq!(lista_estatutos("id + id"), Ok(("", "lista_estatutos")));
    // assert_eq!(lista_estatutos("mientras(expresion)"), Ok(("", "lista_estatutos")));
    // assert_eq!(lista_estatutos("mientras ( expresion )"), Ok(("", "lista_estatutos")));
  }

  #[test]
  fn test_bloque() {
    assert_eq!(bloque("{}"), Ok(("", "bloque")));
    assert_eq!(bloque("{  }"), Ok(("", "bloque")));
    assert_eq!(bloque("{ %% un comentario %% }"), Ok(("", "bloque")));
    // assert_eq!(lista_estatutos("id + id"), Ok(("", "lista_estatutos")));
    // assert_eq!(lista_estatutos("mientras(expresion)"), Ok(("", "lista_estatutos")));
    // assert_eq!(lista_estatutos("mientras ( expresion )"), Ok(("", "lista_estatutos")));
  }
}