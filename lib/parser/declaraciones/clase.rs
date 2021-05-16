use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::declaraciones::funcion::*;
use crate::parser::declaraciones::variables::*;

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
  .map(|(next_input, _res)| {
    // let (tipo, id, params) = res;
    // let mut lista_params = Vec::new();
    // for par in params {
    //   let (tipo_param, param) = par;
    //   lista_params.push((tipo_param, vec![param]))
    // } 
    // (next_input, (tipo, id, lista_params))
    // (next_input, (tipo, id, "funcion"))
    (next_input, ("tipo", "id", "funcion"))
  })
}

// fn variable_funcion(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>)> {
fn variable_funcion(input: &str) -> IResult<&str, (&str, &str, &str)> {
  alt((atributos, metodos))(input)
}

fn lista_variable_funcion(input: &str) -> IResult<&str, Vec<(&str, &str, &str)>> {
  many0(tuple((variable_funcion, ws)))(input)
  .map(|(next_input, res)| {
    let mut lista = Vec::new();
    for r in res {
      let (cont, _) = r;
      lista.push(cont);
    }
    // (next_input, (id, padre, declaraciones))
    (next_input, lista)
  })
}

// pub fn clase(input: &str) -> IResult<&str, (&str, &str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>))> {
// pub fn clase(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, &str, &str)>)> {
pub fn clase(input: &str) -> IResult<&str, &str> {
  tuple((
    ws, tag("clase"), necessary_ws,
    id, ws, posible_herencia, ws,
    tag("{"), ws, lista_variable_funcion, ws, tag("}"), ws, tag(";"), ws
  ))
  (input)
  .map(|(next_input, _res)| {
    // let (_, _, id, _, padre, _, _, _, declaraciones, _, _, _, _) = res;
    // (next_input, (id, padre, declaraciones))
    (next_input, "clase")
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
    assert_eq!(herencia("<Persona>"),   Ok(("", "Persona")));
    assert_eq!(herencia("< Persona >"), Ok(("", "Persona")));
  }

  #[test]
  fn test_posible_herencia() {
    assert_eq!(posible_herencia("<Persona>"),   Ok(("", "Persona")));
    assert_eq!(posible_herencia("< Persona >"), Ok(("", "Persona")));
    assert_eq!(posible_herencia("< Persona"),   Ok(("< Persona", "")));
    assert_eq!(posible_herencia(":{}"),         Ok((":{}", "")));
  }

  #[test]
  fn test_atributos() {
    assert_eq!(atributos("Persona id, id;"),   Ok(("", ("null", "variables", "variables"))));
    assert_eq!(atributos("entero id[10][7];"), Ok(("", ("null", "variables", "variables"))));
  }

  #[test]
  fn test_metodos() {
    // assert_eq!(atributos("Persona id, id;"), Ok(("",    ("Persona", vec![("id", vec![]), ("id", vec![])]))));
    // assert_eq!(atributos("entero id[id][id];"), Ok(("", ("entero",  vec![("id", vec!["id","id"])]))));
    // assert_eq!(metodos("void funcion func (entero var): { estatuto; regresa expresion ; }"), Ok(("", ("void", "func", "funcion"))));
    assert_eq!(metodos("void funcion func (entero var) {  regresa expresion ; }"), Ok(("", ("tipo", "id", "funcion"))));
  }

  #[test]
  fn test_variable_funcion() {
    assert_eq!(variable_funcion("Persona id, id;"),                                    Ok(("", ("null", "variables", "variables"))));
    assert_eq!(variable_funcion("void funcion func (entero var){regresa expresion;}"), Ok(("", ("tipo", "id", "funcion"))));
  }

  #[test]
  fn test_lista_variable_funcion() {
    assert_eq!(lista_variable_funcion("Persona id, id;"),                                    Ok(("", vec![("null", "variables", "variables")])));
    assert_eq!(lista_variable_funcion("void funcion func (entero var){regresa expresion;}"), Ok(("", vec![("tipo", "id", "funcion")])));
    // assert_eq!(lista_variable_funcion(""),                                                                  Ok(("", ("null", "vacio", "vacio"))));
  }

  #[test]
  fn test_clase() {
    // assert_eq!(atributos("Persona id, id;"), Ok(("",    ("Persona", vec![("id", vec![]), ("id", vec![])]))));
    // assert_eq!(atributos("entero id[id][id];"), Ok(("", ("entero",  vec![("id", vec!["id","id"])]))));
    // assert_eq!(metodos("void funcion func (entero var): { estatuto; regresa expresion ; }"), Ok(("", ("void", "func", "funcion"))));
    assert_eq!(clase("clase Estudiante {};"),           Ok(("", "clase")));
    assert_eq!(clase("clase Estudiante <Persona> {};"), Ok(("", "clase")));
    assert_eq!(clase(
      "clase Estudiante <Persona> {
        char nombre[10], apellido[10];
      };"
    ), Ok(("", "clase")));
  }
}
