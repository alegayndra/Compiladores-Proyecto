use nom::{
  IResult,
  bytes::complete::tag,
  sequence::tuple,
  branch::alt,
  multi::many0
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
// use crate::parser::clase::*;
// use crate::parser::funcion::*;
// use crate::parser::variables::*;

fn diferentes_declaraciones(input: &str) -> IResult<&str, &str> {
  alt((tag("variables"), tag("funcion"), tag("clase")))(input)
}

fn lista_declaraciones(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((diferentes_declaraciones, many0(tuple((ws, tag(","), ws, diferentes_declaraciones)))))(input)
  .map(|(next_input, res)| {
    let (decl, declaraciones) = res;
    let mut lista = Vec::new();
    lista.push(decl);
    for d in declaraciones {
      let (_, _, _, de) = d;
      lista.push(de);
    }
    (next_input, lista)
  })
}

fn declaraciones_vacias(input: &str) -> IResult<&str, Vec<&str>> {
  ws(input)
  .map(|(next_input, _)| {
    (next_input, vec![])
  })
}

pub fn declaraciones(input: &str) -> IResult<&str, Vec<&str>> {
  alt((lista_declaraciones, declaraciones_vacias))(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_diferentes_declaraciones() {
    assert_eq!(diferentes_declaraciones("variables"), Ok(("", "variables")));
    assert_eq!(diferentes_declaraciones("clase"), Ok(("", "clase")));
    assert_eq!(diferentes_declaraciones("funcion"), Ok(("", "funcion")));
  }

  #[test]
  fn test_lista_declaraciones() {
    assert_eq!(lista_declaraciones("variables"), Ok(("", vec!["variables"])));
    assert_eq!(lista_declaraciones("clase"), Ok(("", vec!["clase"])));
    assert_eq!(lista_declaraciones("funcion"), Ok(("", vec!["funcion"])));
    assert_eq!(lista_declaraciones("variables, clase, funcion"), Ok(("", vec!["variables", "clase", "funcion"])));
  }

  #[test]
  fn test_declaraciones_vacias() {
    assert_eq!(declaraciones_vacias(""), Ok(("", vec![])));
  }

  #[test]
  fn test_declaraciones() {
    assert_eq!(declaraciones("variables"), Ok(("", vec!["variables"])));
    assert_eq!(declaraciones("clase"), Ok(("", vec!["clase"])));
    assert_eq!(declaraciones("funcion"), Ok(("", vec!["funcion"])));
    assert_eq!(declaraciones("variables, clase, funcion"), Ok(("", vec!["variables", "clase", "funcion"])));
    assert_eq!(declaraciones(""), Ok(("", vec![])));
  }
}
