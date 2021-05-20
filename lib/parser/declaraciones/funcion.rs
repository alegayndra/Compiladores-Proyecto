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
use crate::parser::bloque::*;

fn parametro(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((tipo, ws, id))(input)
  .map(|(next_input, res)| {
    let (tipo, _, id) = res;
    (next_input, (tipo, id))
  })
}

fn parametros_vacios(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
  Ok((input, vec![("", "")]))
}

fn parametros_varios(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
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

fn lista_parametros(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
  alt((parametros_varios, parametros_vacios))(input)
}

pub fn funcion(input: &str) -> IResult<&str, &str> {
  let mut next : &str;
  let tipo_func : &str;
  // let id_func : &str;

  next = match ws(input) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  next = match tipo_retorno(next) {
    Ok((next_input, tipo_f)) => {
      tipo_func = tipo_f;
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match tuple((ws, tag("funcion"), necessary_w))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  next = match id(next) {
    Ok((next_input, id_f)) => {
      // id_func = id_f;
      let contexto_clase = CONTEXTO_CLASE.lock().unwrap();
      let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();

      if contexto_clase.clone() != "".to_owned() {
        match CLASES.lock().unwrap().agregar_variable_metodo(contexto_clase.to_string(), contexto_funcion.to_string(), var.to_owned(), tipo_var.to_owned(), dims_string.clone()) {
          Ok(res) => {
            println!("{:?}", res);
            ()
          },
          Err(err) => {
            println!("{:?}", err);
            ()
          },
        }
      } else {
        match FUNCIONES.lock().unwrap().agregar_variable(contexto_funcion.to_string(), var.to_owned(), tipo_var.to_owned(), dims_string.clone()) {
          Ok(res) => {
            println!("{:?}", res);
            ()
          },
          Err(err) => {
            println!("{:?}", err);
            ()
          },
        }
      }
    
      match FUNCIONES.lock().unwrap().agregar_funcion(id_f.to_string()) {
        Ok(res) => {
          println!("{:?}", res);
          let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();
          *contexto_funcion = id_f;
          ()
        },
        Err(err) => {
          println!("{:?}", err);
          ()
        },
      }
      next_input
    },
    Err(err) => return Err(err)
  };

  tuple((
    ws, tipo_retorno, necessary_ws,
    tag("funcion"), necessary_ws,
    id, ws,
    tag("("), ws, lista_parametros, ws, tag(")"), ws,
    bloque_funcion, ws
  ))
  (input)
  .map(|(next_input, _res)| {
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
    assert_eq!(parametro("char id"),   Ok(("", ("char", "id"))));
    assert_eq!(parametro("entero id"), Ok(("", ("entero", "id"))));
  }

  #[test]
  fn test_parametros_vacios() {
    assert_eq!(parametros_vacios("Persona id"), Ok(("Persona id", vec![("", "")])));
    assert_eq!(parametros_vacios("entero id"),  Ok(("entero id", vec![("", "")])));
  }

  #[test]
  fn test_funcion() {
    // assert_eq!(funcion("void funcion func (entero var): { estatuto; regresa expresion ; }"), Ok(("", ("void", "func", vec![("entero", ("var", vec![]))]))));
    assert_eq!(funcion("void funcion func () { regresa expresion ; }"), Ok(("", "funcion")));
    assert_eq!(funcion("void funcion func (entero var) { num = 10; regresa expresion ; }"), Ok(("", "funcion")));
  }
}
