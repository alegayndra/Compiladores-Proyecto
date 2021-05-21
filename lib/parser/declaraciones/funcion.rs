use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
  combinator::opt
};

use crate::scanners::ws::*;
use crate::scanners::tipos::*;
use crate::scanners::id::*;
use crate::parser::bloque::*;
use crate::semantica::globales::*;

fn parametro(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((tipo, ws, id))(input)
  .map(|(next_input, res)| {
    let (tipo, _, id) = res;
    (next_input, (tipo, id))
  })
}

fn parametros_vacios(input: &str) -> IResult<&str, &str> {
  Ok((input, "parametros_vacios"))
}

fn agregar_param(tipo_param: &str, id_param: &str) {
  let dims_string : Vec<String> = vec![];
  match VARIABLES.lock().unwrap().agregar_variable(id_param.to_owned(), tipo_param.to_owned(), dims_string.clone()) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
      ()
    },
  }

  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();

  if contexto_clase.clone() != "".to_owned() {
    match CLASES.lock().unwrap().agregar_parametro_metodo(contexto_clase.to_string(), contexto_funcion.to_string(), id_param.to_owned(), tipo_param.to_owned(), dims_string.clone()) {
      Ok(res) => {
        println!("{:?}", res);
        ()
      },
      Err(err) => {
        println!("{:?}", err);
        ()
      },
    };
  } else {
    match FUNCIONES.lock().unwrap().agregar_parametro(contexto_funcion.to_string(), id_param.to_owned(), tipo_param.to_owned(), dims_string.clone()) {
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
}

fn parametros_varios(input: &str) -> IResult<&str, &str> {
  let mut next : &str;

  next = match parametro(input) {
    Ok((next_input, (tipo_param, id_param))) => {
      agregar_param(tipo_param, id_param);
      next_input
    },
    Err(err) => return Err(err)
  };

  loop {
    next = match opt(tuple((ws, tag(","), ws)))(next) {
      Ok((next_input, Some(_))) => next_input,
      _ => {
        break;
      }
    };

    next = match parametro(next) {
      Ok((next_input, (tipo_param, id_param))) => {
        agregar_param(tipo_param, id_param);
        next_input
      },
      Err(err) => return Err(err)
    };
  };

  Ok((next, "parametros_varios"))
}

fn lista_parametros(input: &str) -> IResult<&str, &str> {
  alt((parametros_varios, parametros_vacios))(input)
}

pub fn funcion(input: &str) -> IResult<&str, &str> {
  let mut next : &str;
  let tipo_func : &str;

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

  next = match tuple((ws, tag("funcion"), necessary_ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  next = match id(next) {
    Ok((next_input, id_f)) => {
      let contexto_clase = CONTEXTO_CLASE.lock().unwrap();

      if contexto_clase.clone() != "".to_owned() {
        match CLASES.lock().unwrap().agregar_metodo(contexto_clase.to_string(), id_f.to_owned(), tipo_func.to_owned()) {
          Ok(res) => {
            println!("{:?}", res);
            ()
          },
          Err(err) => {
            println!("{:?}", err);
            ()
          }
        };
      } else {
        match FUNCIONES.lock().unwrap().agregar_funcion(id_f.to_owned(), tipo_func.to_owned()) {
          Ok(res) => {
            println!("{:?}", res);
            ()
          },
          Err(err) => {
            println!("{:?}", err);
            ()
          },
        };
      }

      *CONTEXTO_FUNCION.lock().unwrap() = id_f.to_owned();

      next_input
    },
    Err(err) => return Err(err)
  };

  match tuple((
    ws,
    tag("("), ws, lista_parametros, ws, tag(")"), ws,
    bloque_funcion, ws
  ))(next) {
    Ok((next_input, _)) => {
      *CONTEXTO_FUNCION.lock().unwrap() = "".to_owned();
      Ok((next_input, "funcion"))
    },
    Err(err) => Err(err)
  }
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
