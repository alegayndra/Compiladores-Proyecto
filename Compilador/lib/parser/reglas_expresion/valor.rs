use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::{tuple, preceded},
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::scanners::constantes::*;
use crate::parser::llama_func::*;
use crate::parser::dimensiones::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::globales::*;

fn agregar_constante_a_tabla(valor: &str, tipo: &str) {
  let mut pila_valores = PILA_VALORES.lock().unwrap();
  pila_valores.push(CONSTANTES.lock().unwrap().agregar_constante(valor.to_owned(), tipo.to_owned()));
  drop(pila_valores);
}

fn valor_cte(input: &str) -> IResult<&str, &str> {
  alt((num_flotante, num_entero, caracter))(input)
  .map(|(next_input, res)| {
    agregar_constante_a_tabla(res.0, res.1);
    (next_input, "valor_cte")
  })
}

fn agregar_funcion_a_pila_valores(id_valor: &str) {
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();

  let tabla_funciones = FUNCIONES.lock().unwrap();
  let tabla_clases = CLASES.lock().unwrap();
  let funcion;

  if contexto_clase.clone() != "".to_owned() {
    funcion = match tabla_clases.buscar_metodo(contexto_clase.clone(), id_valor.to_owned()) {
      Ok((_, _, func)) => func,
      Err(err) => {
        println!("{:?}", err);
        return;
      }
    };
  } else {
    funcion = match tabla_funciones.buscar_funcion(id_valor.to_owned()) {
      Ok((_, func)) => func,
      Err(err) => {
        println!("{:?}", err);
        return;
      }
    };
  }

  drop(contexto_clase);

  drop(tabla_funciones);
  drop(tabla_clases);

  match CUADRUPLOS.lock().unwrap().agregar_cuadruplo_asignacion_valor_funcion(funcion.direccion, funcion.tipo.clone()) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    }
  };
}

fn parentesis(input: &str) -> IResult<&str, &str> {
  tag("(")(input)
}

fn valor_id(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;
  let id_valor: &str;
  let mut _id_attr: &str;

  next = match id(next) {
    Ok((next_input, id_v)) => {
      id_valor = id_v;
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match preceded(tuple((ws, tag("."), ws)), id)(next) {
    Ok((next_input, id_obj)) => {
      _id_attr = id_obj;
      next_input
    },
    Err(_) => next
  };

  match parentesis(next) {
    Ok((next_input, _)) => {
      next = next_input;
      {
        PILA_OPERADORS.lock().unwrap().push("(".to_owned());
      }
      let params = generar_cuadruplo_era(id_valor);
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
        Err(_) => {
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

      match tuple((ws, tag(")")))(next) {
        Ok((next_input, _)) => {
          {
            PILA_OPERADORS.lock().unwrap().pop();
          }
          generar_cuadruplo_gosub(id_valor);
          agregar_funcion_a_pila_valores(id_valor);
          Ok((next_input, "valor_id"))
        },
        Err(err) => Err(err)
      }
    },
    Err(_) => {
      con_dim(id_valor)(next)
      .map(|(next_input, _)| {
        (next_input, "valor_id")
      })
    }
  }
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

  #[test]
  fn test_valor_cte() {
    assert_eq!(valor_cte("\"s\""),  Ok(("", "valor_cte")));
    assert_eq!(valor_cte("10"),     Ok(("", "valor_cte")));
    assert_eq!(valor_cte("10.1"),   Ok(("", "valor_cte")));
  }

  #[test]
  fn test_valor_id() {
    assert_eq!(valor_id("id"),                                  Ok(("", "valor_id")));
    assert_eq!(valor_id("SoyUnString.arreglo[id]"),             Ok(("", "valor_id")));
    assert_eq!(valor_id("Nombre.metodo()"),                     Ok(("", "valor_id")));
    // assert_eq!(valor_id("Nombre.metodo(expresion)"),            Ok(("", "valor_id")));
    // assert_eq!(valor_id("Nombre.metodo(expresion)"),            Ok(("", "valor_id")));
  }
}