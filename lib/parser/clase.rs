use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::funcion::*;
use crate::parser::variables::*;

fn herencia(input: &str) -> IResult<&str, &str> {
  tuple((tag("<"), ws, id, ws, tag(">")))(input)
  .map(|(next_input, res)| {
    let (_, _, id, _, _,) = res;
    (next_input, id)
  })
}

fn posible_herencia(input: &str) -> IResult<&str, &str> {
  alt((herencia, ws))(input)
}

// fn atributos(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>)> {
fn atributos(input: &str) -> IResult<&str, (&str, &str, &str)> {
  variables(input)
  .map(|(next_input, _res)| {
    // (next_input, ("null", "variables", vec![res]))
    (next_input, ("null", "variables", "variables"))
  })
}

// fn metodos(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>)> {
fn metodos(input: &str) -> IResult<&str, (&str, &str, &str)> {
  funcion(input)
  .map(|(next_input, res)| {
    let (tipo, id, params) = res;
    let mut lista_params = Vec::new();
    for par in params {
      let (tipo_param, param) = par;
      lista_params.push((tipo_param, vec![param]))
    } 
    // (next_input, (tipo, id, lista_params))
    (next_input, (tipo, id, "funcion"))
  })
}

// fn variable_funcion(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>)> {
fn variable_funcion(input: &str) -> IResult<&str, (&str, &str, &str)> {
  alt((atributos, metodos))(input)
}

// pub fn clase(input: &str) -> IResult<&str, (&str, &str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>))> {
pub fn clase(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, &str, &str)>)> {
  tuple((
    tag("clase"), necessary_ws, id, posible_herencia, tag("{"), many0(variable_funcion), tag("}"), ws, tag(";") 
  ))
  (input)
  .map(|(next_input, res)| {
    let (_, _, id, padre, _, declaraciones, _, _, _) = res;
    (next_input, (id, padre, declaraciones))
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
  fn test_herencia() {
    assert_eq!(herencia("<Persona>"), Ok(("", "Persona")));
    assert_eq!(herencia("< Persona >"), Ok(("", "Persona")));
  }

  #[test]
  fn test_posible_herencia() {
    assert_eq!(posible_herencia("<Persona>"), Ok(("", "Persona")));
    assert_eq!(posible_herencia("< Persona >"), Ok(("", "Persona")));
    assert_eq!(posible_herencia("< Persona"), Ok(("< Persona", "")));
    assert_eq!(posible_herencia(":{}"), Ok((":{}", "")));
  }

  #[test]
  fn test_atributos() {
    // assert_eq!(atributos("Persona id, id;"), Ok(("",    ("Persona", vec![("id", vec![]), ("id", vec![])]))));
    // assert_eq!(atributos("entero id[id][id];"), Ok(("", ("entero",  vec![("id", vec!["id","id"])]))));
    assert_eq!(atributos("Persona id, id;"), Ok(("", ("null", "variables", "variables"))));
    assert_eq!(atributos("entero id[id][id];"), Ok(("", ("null", "variables", "variables"))));
  }

  #[test]
  fn test_metodos() {
    // assert_eq!(atributos("Persona id, id;"), Ok(("",    ("Persona", vec![("id", vec![]), ("id", vec![])]))));
    // assert_eq!(atributos("entero id[id][id];"), Ok(("", ("entero",  vec![("id", vec!["id","id"])]))));
    assert_eq!(metodos("void funcion func (entero var): { estatuto; regresa expresion ; }"), Ok(("", ("void", "func", "funcion"))));
  }
}
