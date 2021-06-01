use nom::{
  bytes::complete::tag,
  IResult,
  sequence::{tuple, delimited, preceded},
};
  
use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::exp::*;
use crate::parser::reglas_expresion::valor::*;
use crate::semantica::globales::*;
use crate::semantica::tabla_variables::*;

fn generar_cuadruplo_asignacion(variable: TipoVar) {
  let mut pila_valores = PILA_VALORES.lock().unwrap();
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();

  match pila_valores.pop() {
    Some(valor) => {
      match cuadruplos.agregar_cuadruplo_asignacion(variable, valor) {
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

  let variable = buscar_variable(id_valor);
  next = match corchete(next) {
    Ok((next_input, _)) => {
      match variable.dimensiones.len() {
        0 => {
          println!("Variable no tiene dimensiones");
        },
        _ => {
          PILA_DIMENSIONES.lock().unwrap().push((variable.clone(), 1));
          PILA_OPERADORS.lock().unwrap().push("(".to_owned());
        }
      };

      match tuple((delimited(ws, exp, ws), tag("]")))(next_input) {
        Ok((next_i, _)) => {
          {
            PILA_DIMENSIONES.lock().unwrap().pop();
            PILA_OPERADORS.lock().unwrap().pop();
          }
          let mut pila_valores = PILA_VALORES.lock().unwrap();
          let valor = pila_valores.pop().unwrap();
          let mut cuadruplos = CUADRUPLOS.lock().unwrap();
          let mut constantes = CONSTANTES.lock().unwrap();
          cuadruplos.agregar_cuadruplo_verificar(valor.direccion, variable.dimensiones[0]);
          drop(pila_valores);
          match variable.dimensiones.len() {
            1 => {
              let dir = constantes.agregar_constante(variable.direccion.to_string(), variable.tipo.clone());
              cuadruplos.agregar_cuadruplo("+",valor.clone(), dir.clone());
              {
                let apuntador = PILA_VALORES.lock().unwrap().pop().unwrap();
                cuadruplos.agregar_cuadruplo_acceder(apuntador);
              }
              next_i
            },
            2 => {
              let dim_constante = constantes.agregar_constante(variable.dimensiones[1].to_string(), variable.tipo.clone());
              cuadruplos.agregar_cuadruplo("*", valor.clone(), dim_constante.clone());
              match corchete(next_i) {
                Ok((next_input, _)) => {
                  {
                    PILA_DIMENSIONES.lock().unwrap().push((variable.clone(), 2));
                    PILA_OPERADORS.lock().unwrap().push("(".to_owned());
                  }
                  drop(cuadruplos);
                  drop(constantes);
                  match tuple((delimited(ws, exp, ws), tag("]")))(next_input) {
                    Ok((next_i, _)) => {
                      println!("exp 2");
                      {
                        PILA_DIMENSIONES.lock().unwrap().pop();
                        PILA_OPERADORS.lock().unwrap().pop();
                      }
                      let mut pila_valores = PILA_VALORES.lock().unwrap();
                      let mut cuadruplos = CUADRUPLOS.lock().unwrap();
                      let mut constantes = CONSTANTES.lock().unwrap();
                      let valor = pila_valores.pop().unwrap();
                      cuadruplos.agregar_cuadruplo_verificar(valor.direccion, variable.dimensiones[1]);
                      drop(pila_valores);
                      let dir = constantes.agregar_constante(variable.direccion.to_string(), variable.tipo.clone());
                      cuadruplos.agregar_cuadruplo("+", valor.clone(), dir.clone());
                      {
                        let apuntador = PILA_VALORES.lock().unwrap().pop().unwrap();
                        cuadruplos.agregar_cuadruplo_acceder(apuntador);
                      }
                      next_i
                    },
                    Err(err) => {
                      println!("Error mero dentro");
                      return Err(err);
                    }
                  }
                },
                Err(_) => next_i
              }
            },
            _ => next_i
          }
        },
        Err(err) => {
          println!("Error no sé");
          return Err(err);
        }
      }
    },
    Err(_) => {
      PILA_VALORES.lock().unwrap().push(variable);
      next
    }
  };

  next = match tuple((ws, tag("="), ws, exp))(next) {
    Ok((next_input, _)) => {
      let var;
      {
        var = PILA_VALORES.lock().unwrap().pop().unwrap();
      }
      generar_cuadruplo_asignacion(var);
      next_input
    },
    Err(err) => {
      println!("Error =");
      return Err(err);
    }
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
  