use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};
  
use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::globales::*;

fn generar_cuadruplo_asignacion(id_valor: &str, _dims: Vec<&str>) {
  let variable;
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();

  let tabla_variables = VARIABLES.lock().unwrap();
  let tabla_funciones = FUNCIONES.lock().unwrap();
  let tabla_clases = CLASES.lock().unwrap();
  
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();

  match tabla_variables.buscar_variable(id_valor.to_owned()) {
    Ok((_, var)) => { variable = var; },
    Err(_) => {
      if contexto_clase.clone() != "".to_owned() {
        if contexto_funcion.clone() != "".to_owned() {
          variable = match tabla_clases.buscar_variable_metodo(contexto_clase.clone(), contexto_funcion.clone(), id_valor.to_owned()) {
            Ok((_, _, _, var)) => var,
            Err(err) => { 
              println!("{:?}", err);
              return;
            }
          };
        } else {
          variable = match tabla_clases.buscar_atributo(contexto_clase.clone(), id_valor.to_owned()) {
            Ok((_, _, var)) => var,
            Err(err) => {
              println!("{:?}", err); 
              return;
            }
          };
        }
      } else {
        variable =match tabla_funciones.buscar_variable(contexto_funcion.clone(), id_valor.to_owned()) {
          Ok((_, _, var)) => var,
          Err(err) => {
            println!("{:?}", err);
            return;
          }
        };
      }
    }
  };

  drop(contexto_funcion);
  drop(contexto_clase);

  drop(tabla_variables);
  drop(tabla_funciones);
  drop(tabla_clases);

  let mut pila_valores = PILA_VALORES.lock().unwrap();

  match pila_valores.pop() {
    Some(valor) => {
      match cuadruplos.agregar_cuadruplo_asignacion(valor, variable) {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      };
      return;
    },
    _ => { println!("Stack de valores vacío en EXP_LOGICA"); return; }
  };
}

pub fn asignacion(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;
  let variable;
  let dimensiones;

  next = match id_con_dim(next) {
    Ok((next_input, (id_valor, dims))) => {
      variable = id_valor;
      dimensiones = dims;
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match tuple((ws, tag("="), ws, exp))(next) {
    Ok((next_input, _)) => {
      generar_cuadruplo_asignacion(variable, dimensiones);
      next_input
    },
    Err(err) => return Err(err)
  };

  match tuple((ws, tag(";")))(next) {
    Ok((next_input, _)) => Ok((next_input, "asignacion")),
    Err(err) => Err(err)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_asignacion() {
    assert_eq!(asignacion("id = 10;"), Ok(("", "asignacion")));
  }
}
  