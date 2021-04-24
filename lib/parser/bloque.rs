use nom::{
    bytes::complete::tag,
    multi::many0,
    IResult,
    sequence::tuple,
};

  
use crate::scanners::ws::*;
use crate::parser::estatuto::*;


// fn lista_estautos(input: &str) -> IResult<&str, Vec<&str>> {
fn lista_estatutos(input: &str) -> IResult<&str, &str> {
  many0(tuple((estatuto, ws)))(input)
  .map(|(next_input, _res)| {
    // let mut lista = Vec::new();
    // for val in res {
    //   let (estatuto, _) = val;
    //   lista.push(estatuto);
    // }
    // (next_input, lista)
    (next_input, "lista_estatutos")
  })
}

// pub fn bloque(input: &str) -> IResult<&str, Vec<&str>> {
pub fn bloque(input: &str) -> IResult<&str, &str> {
  tuple((tag("{"), ws, lista_estatutos, ws, tag("}")))(input)
  .map(|(next_input, res)| {
    let (_, _, _estatuto, _, _,) = res;
    // (next_input, estatuto)
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