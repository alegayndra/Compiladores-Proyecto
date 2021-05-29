use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
  combinator::opt,
};

use crate::scanners::ws::*;
use crate::scanners::tipos::*;
use crate::scanners::id::*;
use crate::semantica::globales::*;

fn variable_compuesta(input: &str) -> IResult<&str, &str> {
  let mut next : &str;
  let tipo_var : &str;

  next = match id(input) {
    Ok((next_input, res)) => {
      tipo_var = res;
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match ws(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  next = match id_sin_dim(next) {
    Ok((next_input, (var, dims))) => {
      agregar_variable_a_tabla(var, tipo_var, dims);
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

    next = match id_sin_dim(next) {
      Ok((next_input, (var, dims))) => {
        agregar_variable_a_tabla(var, tipo_var, dims);
        next_input
      },
      Err(err) => return Err(err)
    };
  };

  match ws(next) {
    Ok((next_input, _)) => Ok((next_input, "variable compuesta")),
    Err(err) => Err(err)
  }
}

fn agregar_variable_a_tabla(var: &str, tipo_var: &str, dims: Vec<&str>) {
  let mut dims_string : Vec<String> = vec![];
  for dim in dims {
    dims_string.push(dim.to_owned());
  }

  let dir = match conseguir_direccion(tipo_var, "variable", 0) {
    Ok(num) => num,
    Err(err) => { println!("{:?}", err); return; }
  };

  match VARIABLES.lock().unwrap().agregar_variable(var.to_owned(), tipo_var.to_owned(), dims_string.clone(), dir) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
      ()
    },
  }

  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();

  if contexto_clase.clone() != "".to_owned() {
    if contexto_funcion.clone() != "".to_owned() {
      match CLASES.lock().unwrap().agregar_atributo(contexto_clase.to_string(), var.to_owned(), tipo_var.to_owned(), dims_string.clone(), 20000) {
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
      match CLASES.lock().unwrap().agregar_variable_metodo(contexto_clase.to_string(), contexto_funcion.to_string(), var.to_owned(), tipo_var.to_owned(), dims_string.clone(), 25000, 0) {
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
  } else {
    match FUNCIONES.lock().unwrap().agregar_variable(contexto_funcion.to_string(), var.to_owned(), tipo_var.to_owned(), dims_string.clone(), dir, 0) {
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

fn variable_normal(input: &str) -> IResult<&str, &str> {
  let mut next : &str;
  let tipo_var : &str;

  next = match tipo(input) {
    Ok((next_input, res)) => {
      tipo_var = res;
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match ws(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  next = match id_con_dim_decl(next) {
    Ok((next_input, (var, dims))) => {
      agregar_variable_a_tabla(var, tipo_var, dims);
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

    next = match id_con_dim_decl(next) {
      Ok((next_input, (var, dims))) => {
        agregar_variable_a_tabla(var, tipo_var, dims);
        next_input
      },
      Err(err) => return Err(err)
    };
  };

  match ws(next) {
    Ok((next_input, _)) => Ok((next_input, "variable normal")),
    Err(err) => Err(err)
  }
}

pub fn variables(input: &str) -> IResult<&str, &str> {
  tuple((ws, alt((variable_normal, variable_compuesta)), tag(";"), ws))
  (input)
  .map(|(next_input, _res)| {
    (next_input, "variables")
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
  fn test_variable_compuesta() {
    assert_eq!(variable_compuesta("id id;"), Ok((";", "variable compuesta")));
    assert_eq!(variable_compuesta("id id, id;"), Ok((";", "variable compuesta")));
  }

  #[test]
  fn test_variable_normal() {
    assert_eq!(variable_normal("entero id;"), Ok((";", "variable normal")));
    assert_eq!(variable_normal("flotante id, id2;"), Ok((";", "variable normal")));
    assert_eq!(variable_normal("char id, id2;"), Ok((";", "variable normal")));
  }

  #[test]
  fn test_variables() {
    assert_eq!(variables("Persona id;"),        Ok(("", "variables")));
    assert_eq!(variables("Persona id, id;"),    Ok(("", "variables")));
    assert_eq!(variables("entero id;"),         Ok(("", "variables")));
    assert_eq!(variables("entero id[74];"),     Ok(("", "variables")));
    assert_eq!(variables("entero id[22][0];"), Ok(("", "variables")));
    assert_eq!(variables("entero id, id;"),     Ok(("", "variables")));
    assert_eq!(variables("entero id[1], id;"), Ok(("", "variables")));
  }
}
