use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  multi::many0,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::tipos::*;
use crate::scanners::id::*;
// use crate::parser::dimensiones::*;

fn parametro(input: &str) -> IResult<&str, (&str, (&str, Vec<&str>))> {
  alt((
    tuple((tipo, ws, id_con_dim)),
    tuple((id, ws, id_sin_dim)),
  ))(input)
  .map(|(next_input, res)| {
    let (tipo, _, id) = res;
    (next_input, (tipo, id))
  })
}

fn parametros_vacios(input: &str) -> IResult<&str, Vec<(&str, (&str, Vec<&str>))>> {
  ws(input)
  .map(|(next_input, _res)| {
    (next_input, vec![("", ("", vec![]))])
  })
}

fn parametros_varios(input: &str) -> IResult<&str, Vec<(&str, (&str, Vec<&str>))>> {
  tuple((parametro, many0(tuple((ws, tag(","), ws, parametro)))))(input)
  .map(|(next_input, res)| {
    let (param, params) = res;
    let mut lista_params = Vec::new();
    lista_params.push(param);
    for param in params {
      let (_, _, _, par) = param;
      lista_params.push(par);
    }
    (next_input, lista_params)
  })
}

fn lista_parametros(input: &str) -> IResult<&str, Vec<(&str, (&str, Vec<&str>))>> {
  alt((parametros_varios, parametros_vacios))(input)
}

fn bloque_funcion(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((
    tag("{"), ws,
    tag("estatuto;"), ws,
    tag("regresa"), necessary_ws, tag("expresion"), ws, tag(";"), ws,
    tag("}")
  ))(input)
  .map(|(next_input, res)| {
    let (_, _, estatutos, _, _, _, valor_retorno, _, _, _, _) = res;
    (next_input, (estatutos, valor_retorno))
  })
}

// pub fn funcion(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>)> {
pub fn funcion(input: &str) -> IResult<&str, &str> {
  tuple((
    tipo_retorno, necessary_ws,
    tag("funcion"), necessary_ws,
    id, ws,
    tag("("), ws, lista_parametros, ws, tag(")"), ws, tag(":"), ws,
    bloque_funcion
  ))
  (input)
  .map(|(next_input, _res)| {
    // let (tipo, _, _, _, id, _, _, _, lista_params, _, _, _, _, _, _bloque) = res;
    // (next_input, (tipo, id, lista_params))
    (next_input, "funcion")
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
  fn test_parametro() {
    assert_eq!(parametro("Persona id"), Ok(("", ("Persona", ("id", vec![])))));
    assert_eq!(parametro("entero id"), Ok(("", ("entero", ("id", vec![])))));
    assert_eq!(parametro("entero id[id]"), Ok(("", ("entero", ("id", vec!["id"])))));
  }

  #[test]
  fn test_parametros_vacios() {
    assert_eq!(parametros_vacios("Persona id"), Ok(("Persona id", vec![("", ("", vec![]))])));
    assert_eq!(parametros_vacios("entero id"), Ok(("entero id", vec![("", ("", vec![]))])));
    assert_eq!(parametros_vacios("entero id[id]"), Ok(("entero id[id]", vec![("", ("", vec![]))])));
  }

  #[test]
  fn test_funcion() {
    // assert_eq!(funcion("void funcion func (entero var): { estatuto; regresa expresion ; }"), Ok(("", ("void", "func", vec![("entero", ("var", vec![]))]))));
    assert_eq!(funcion("void funcion func (entero var): { estatuto; regresa expresion ; }"), Ok(("", "funcion")));
  }
}
