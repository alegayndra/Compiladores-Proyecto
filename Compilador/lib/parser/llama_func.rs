use nom::{
  bytes::complete::tag,
  IResult,
  sequence::{tuple, preceded},
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::tabla_variables::*;
use crate::semantica::globales::*;

fn generar_cuadruplo_era(id_func: &str) -> Vec<TipoVar> {
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap().to_string();
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  if contexto_clase != "".to_owned() {
    match CLASES.lock().unwrap().buscar_metodo(contexto_clase, id_func.to_owned()) {
      Ok((_, _, func)) => {
        match cuadruplos.agregar_cuadruplo_era(func.direccion) {
          Ok(_) => (),
          Err(err) => {
            println!("{:?}", err);
          }
        };
        return func.parametros.clone();
      },
      Err(err) => {
        println!("{:?}", err);
        return vec![];
      }
    };
  } else {
    match FUNCIONES.lock().unwrap().buscar_funcion(id_func.to_owned()) {
      Ok((_, func)) => {
        match cuadruplos.agregar_cuadruplo_era(func.direccion) {
          Ok(_) => (),
          Err(err) => {
            println!("{:?}", err);
          }
        };
        return func.parametros.clone();
      },
      Err(err) => {
        println!("{:?}", err);
        return vec![];
      }
    };
  }
}

fn generar_cuadruplo_gosub(id_func: &str) {
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap().to_string();
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  if contexto_clase != "".to_owned() {
    match CLASES.lock().unwrap().buscar_metodo(contexto_clase, id_func.to_owned()) {
      Ok((_, _, func)) => {
        match cuadruplos.agregar_cuadruplo_gosub(func.num_cuadruplo) {
          Ok(_) => (),
          Err(err) => {
            println!("{:?}", err);
          }
        };
      },
      Err(err) => {
        println!("{:?}", err);
      }
    };
  } else {
    match FUNCIONES.lock().unwrap().buscar_funcion(id_func.to_owned()) {
      Ok((_, func)) => {
        match cuadruplos.agregar_cuadruplo_gosub(func.num_cuadruplo) {
          Ok(_) => (),
          Err(err) => {
            println!("{:?}", err);
          }
        };
      },
      Err(err) => {
        println!("{:?}", err);
      }
    };
  }
}

fn generar_cuadruplo_param(params: Vec<TipoVar>, pos: usize) {
  let variable = match PILA_VALORES.lock().unwrap().pop() {
    Some(var) => var,
    None => return
  };

  if params[pos].tipo != variable.tipo {
    return;
  }

  match CUADRUPLOS.lock().unwrap().agregar_cuadruplo_param(variable.clone(), params[pos].clone()) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    }
  };
}

pub fn llama_func(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;
  let id_func: &str;
  let _id_attr: &str;

  next = match id(next) {
    Ok((next_input, id_f)) => {
      id_func = id_f;
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match preceded(tuple((ws, tag("."), ws)), id)(next) {
    Ok((next_input, id_a)) => {
      _id_attr = id_a;
      next_input
    },
    Err(_) => next
  };

  next = match tag("(")(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  let params = generar_cuadruplo_era(id_func);
  let mut pos: usize = 0;
  let mut continuar = true;

  next = match preceded(ws, exp)(next) {
    Ok((next_input, _)) => {
      if pos >= params.len() {
        println!("Se excedió la cantidad de parametros dentro de la llamada a función");
      } else {
        generar_cuadruplo_param(params.clone(), pos);
        pos += 1;
      }
      next_input
    },
    Err(_err) => {
      continuar = false;
      next
    }
  };

  if continuar {
    loop {
      next = match tuple((ws, tag(",")))(next) {
        Ok((next_input, _)) => next_input,
        Err(_err) => break
      };
  
      next = match preceded(ws, exp)(next) {
        Ok((next_input, _)) => {
          if pos >= params.len() {
            println!("Se excedió la cantidad de parametros dentro de la llamada a función");
          } else {
            generar_cuadruplo_param(params.clone(), pos);
            pos += 1;
          }
          next_input
        },
        Err(err) => return Err(err)
      };
    }
  }

  match tuple((ws, tag(")"), ws, tag(";")))(next) {
    Ok((next_input, _)) => {
      generar_cuadruplo_gosub(id_func);
      Ok((next_input, "llama_func"))
    },
    Err(err) => Err(err)
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_llama_func() {
    assert_eq!(llama_func("id();"),           Ok(("", "llama_func")));
    assert_eq!(llama_func("id.metodo();"),    Ok(("", "llama_func")));
    assert_eq!(llama_func("id(expresion);"),  Ok(("", "llama_func")));
  }
}
