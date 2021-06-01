use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::{tuple, preceded, delimited},
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::scanners::constantes::*;
use crate::parser::dimensiones::*;
use crate::parser::llama_func::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::globales::*;
use crate::semantica::tabla_variables::*;

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

pub fn buscar_variable(id_valor: &str) -> TipoVar {
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();

  let tabla_variables = VARIABLES.lock().unwrap();
  let tabla_funciones = FUNCIONES.lock().unwrap();
  let tabla_clases = CLASES.lock().unwrap();

  match tabla_variables.buscar_variable(id_valor.to_owned()) {
    Ok((_, var)) => return var,
    Err(_) => ()
  };

  if contexto_clase.clone() != "".to_owned() {
    if contexto_funcion.clone() != "".to_owned() {
      match tabla_clases.buscar_variable_metodo(contexto_clase.clone(), contexto_funcion.clone(), id_valor.to_owned()) {
        Ok((_, _, _, var)) => return var,
        Err(err) => {
          println!("{:?}", err);
        }
      };
    } else {
      match tabla_clases.buscar_atributo(contexto_clase.clone(), id_valor.to_owned()) {
        Ok((_, _, var)) => return var,
        Err(err) => {
          println!("{:?}", err);
        }
      };
    }
  } else {
    match tabla_funciones.buscar_variable(contexto_funcion.clone(), id_valor.to_owned()) {
      Ok((_, _, var)) => return var,
      Err(err) => {
        println!("{:?}", err);
      }
    };
  }

  drop(contexto_funcion);
  drop(contexto_clase);

  drop(tabla_variables);
  drop(tabla_funciones);
  drop(tabla_clases);

  TipoVar {
    nombre: "".to_owned(),
    tipo: "".to_owned(),
    dimensiones: vec![],
    direccion: -10
  }
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

pub fn corchete(input: &str) -> IResult<&str, &str> {
  tag("[")(input)
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
      let variable = buscar_variable(id_valor);
      match corchete(next) {
        Ok((next_input, _)) => {

          match variable.dimensiones.len() {
            0 => {
              println!("Variable no tiene dimensiones");
            },
            _ => {
              PILA_DIMENSIONES.lock().unwrap().push((variable.clone(), 1));
              PILA_OPERADORS.lock().unwrap().push("(".to_owned());
            }
          }

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
                  return Ok((next_i, "valor_id"));
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
                          return Ok((next_i, "valor_id"));
                        },
                        Err(err) => return Err(err)
                      }
                    },
                    Err(_) => {
                      PILA_VALORES.lock().unwrap().push(variable.clone());
                      return Ok((next_input, "valor_id"));
                    }
                  };
                },
                _ => ()
              };
              Ok((next_i, "valor_id"))
            },
            Err(err) => return Err(err)
          }
        },
        Err(_) => {
          PILA_VALORES.lock().unwrap().push(variable.clone());
          Ok((next, "valor_id"))
        }
      }
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