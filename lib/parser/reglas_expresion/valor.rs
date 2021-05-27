use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::{tuple, preceded},
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::scanners::constantes::*;
use crate::parser::dimensiones::*;
use crate::parser::func_params::*;
use crate::semantica::globales::*;
use crate::semantica::cubo_semantico::*;

fn agregar_constante_a_tabla(valor: &str, tipo: &str) {
  let mut pila_valores = PILA_VALORES.lock().unwrap();
  pila_valores.push(CONSTANTES.lock().unwrap().agregar_constante(valor.to_owned(), tipo.to_owned()));
  unsafe {
    match conseguir_num_tipo(tipo) {
      0 => ERA_CONSTANTES.0 += 1,
      1 => ERA_CONSTANTES.1 += 1,
      2 => ERA_CONSTANTES.2 += 1,
      5 => ERA_CONSTANTES.3 += 1,
      _ => (),
    }
  }
  drop(pila_valores);
}

fn valor_cte(input: &str) -> IResult<&str, &str> {
  alt((num_flotante, num_entero, caracter))(input)
  .map(|(next_input, res)| {
    agregar_constante_a_tabla(res.0, res.1);
    (next_input, "valor_cte")
  })
}

fn buscar_variable(id_valor: &str) {
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();

  let tabla_variables = VARIABLES.lock().unwrap();
  let tabla_funciones = FUNCIONES.lock().unwrap();
  let tabla_clases = CLASES.lock().unwrap();

  match tabla_variables.buscar_variable(id_valor.to_owned()) {
    Ok(_) => { return; },
    Err(_) => ()
  };

  if contexto_clase.clone() != "".to_owned() {
    if contexto_funcion.clone() != "".to_owned() {
      match tabla_clases.buscar_variable_metodo(contexto_clase.clone(), contexto_funcion.clone(), id_valor.to_owned()) {
        Ok(res) => { println!("{:?}", res); () },
        Err(err) => { println!("{:?}", err); () }
      };
    } else {
      match tabla_clases.buscar_atributo(contexto_clase.clone(), id_valor.to_owned()) {
        Ok(res) => { println!("{:?}", res); () },
        Err(err) => { println!("{:?}", err); () }
      };
    }
  } else {
    match tabla_funciones.buscar_variable(contexto_funcion.clone(), id_valor.to_owned()) {
      Ok(res) => { println!("{:?}", res); () },
      Err(err) => { println!("{:?}", err); () }
    };
  }

  drop(contexto_funcion);
  drop(contexto_clase);

  drop(tabla_variables);
  drop(tabla_funciones);
  drop(tabla_clases);
}

fn agregar_variable_a_pila(id_valor: &str, dims: Vec<String>) {
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();

  let tabla_variables = VARIABLES.lock().unwrap();
  let tabla_funciones = FUNCIONES.lock().unwrap();
  let tabla_clases = CLASES.lock().unwrap();
  let mut pila_valores = PILA_VALORES.lock().unwrap();

  match tabla_variables.buscar_variable(id_valor.to_owned()) {
    Ok((_, var)) => {
      if dims.clone() == var.dimensiones.clone() { pila_valores.push(var); }
      ()
    },
    Err(_) => ()
  };

  if contexto_clase.clone() != "".to_owned() {
    if contexto_funcion.clone() != "".to_owned() {
      match tabla_clases.buscar_variable_metodo(contexto_clase.clone(), contexto_funcion.clone(), id_valor.to_owned()) {
        Ok((_, _, _, var)) => {
          if dims.clone() == var.dimensiones.clone() { pila_valores.push(var); }
          ()
        },
        Err(err) => { println!("{:?}", err); () }
      };
    } else {
      match tabla_clases.buscar_atributo(contexto_clase.clone(), id_valor.to_owned()) {
        Ok((_, _, var)) => {
          if dims.clone() == var.dimensiones.clone() { pila_valores.push(var); }
          ()
        },
        Err(err) => { println!("{:?}", err); () }
      };
    }
  } else {
    match tabla_funciones.buscar_variable(contexto_funcion.clone(), id_valor.to_owned()) {
      Ok((_, _, var)) => {
        if dims.clone() == var.dimensiones.clone() { pila_valores.push(var); }
        ()
      },
      Err(err) => { println!("{:?}", err); () }
    };
  }

  drop(pila_valores);
  
  drop(contexto_funcion);
  drop(contexto_clase);

  drop(tabla_variables);
  drop(tabla_funciones);
  drop(tabla_clases);
}

fn valor_id(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;
  let mut id_valor: &str;
  let mut vec_dims: Vec<String> = vec![];

  next = match id(next) {
    Ok((next_input, id_v)) => {
      id_valor = id_v;
      buscar_variable(id_valor);
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match preceded(tuple((ws, tag("."), ws)), id)(next) {
    Ok((next_input, id_obj)) => {
      id_valor = id_obj;
      next_input
    },
    Err(_) => next
  };

  match func_params(next) {
    Ok((next_input, _res)) => {
      // checar todo el rollo de parametros
      return Ok((next_input, "valor_id"));
    },
    Err(_) => ()
  };

  next = match con_dim(next) {
    Ok((next_input, dims)) => {
      for dim in dims { vec_dims.push(dim.to_owned()); }
      next_input
    },
    Err(_) => next
  };

  agregar_variable_a_pila(id_valor, vec_dims);

  Ok((next, "valor_id"))
}

pub fn valor(input: &str) -> IResult<&str, &str> {
  alt((valor_cte, valor_id))(input)
  .map(|(next_input, _)| {
    (next_input, "valor")
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
  fn test_valor_cte() {
    assert_eq!(valor_cte("\"s\""),  Ok(("", "valor_cte")));
    assert_eq!(valor_cte("10"),     Ok(("", "valor_cte")));
    assert_eq!(valor_cte("10.1"),   Ok(("", "valor_cte")));
  }

  #[test]
  fn test_valor_id() {
    assert_eq!(valor_id("SoyUnString.arreglo[id]"),             Ok(("", "valor_id")));
    assert_eq!(valor_id("Nombre.metodo(expresion)"),            Ok(("", "valor_id")));
    assert_eq!(valor_id("Nombre.metodo(expresion)"),            Ok(("", "valor_id")));
    assert_eq!(valor_id("Nombre.metodo()"),                     Ok(("", "valor_id")));
    // assert_eq!(valor_id("Objeto.metodo.arreglo[id][id]"),       Ok(("", "valor_id")));
    // assert_eq!(valor_id("Nombre.metodo.arreglo[  id][id ]"),    Ok(("", "valor_id")));
  }
}