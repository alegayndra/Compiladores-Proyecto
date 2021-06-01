use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::{tuple, delimited},
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::globales::*;
use crate::semantica::tabla_variables::*;

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

pub fn corchete(input: &str) -> IResult<&str, &str> {
  tag("[")(input)
}

pub fn ws_vec(input: &str) -> IResult<&str, Vec<&str>> {
  Ok((input, vec![]))
}

pub fn con_dim(id_valor: &str) -> impl FnMut(&str)  -> IResult<&str, &str> + '_ {
  move |input| {
    let next: &str = input;
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
                return Ok((next_i, "con_dim"));
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
                        return Ok((next_i, "con_dim"));
                      },
                      Err(err) => return Err(err)
                    }
                  },
                  Err(_) => {
                    PILA_VALORES.lock().unwrap().push(variable.clone());
                    return Ok((next_input, "con_dim"));
                  }
                };
              },
              _ => ()
            };
            Ok((next_i, "con_dim"))
          },
          Err(err) => return Err(err)
        }
      },
      Err(_) => {
        PILA_VALORES.lock().unwrap().push(variable.clone());
        Ok((next, "con_dim"))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dimension() {
    assert_eq!(dimension("[termino]"),     Ok(("", vec!["exp"])));
    assert_eq!(dimension("[num_float]"),   Ok(("", vec!["exp"])));
    assert_eq!(dimension("[  id  ]"),      Ok(("", vec!["exp"])));
  }

  #[test]
  fn test_dos_dimensiones() {
    assert_eq!(dos_dimensiones("[id][id]"),         Ok(("", vec!["exp", "exp"])));
    assert_eq!(dos_dimensiones("[ id ][ id ]"),     Ok(("", vec!["exp", "exp"])));
    assert_eq!(dos_dimensiones("[  id  ][  id  ]"), Ok(("", vec!["exp", "exp"])));
  }

  #[test]
  fn test_ws_vec() {
    assert_eq!(ws_vec("aaaa"), Ok(("aaaa", vec![])));
    assert_eq!(ws_vec("bbbb"), Ok(("bbbb", vec![])));
    assert_eq!(ws_vec("cccc"), Ok(("cccc", vec![])));
    assert_eq!(ws_vec("    "), Ok(("    ", vec![])));
  }

  #[test]
  fn test_con_dim() {
    assert_eq!(con_dim("[id]"),     Ok(("", vec!["exp"])));
    assert_eq!(con_dim("[id][id]"), Ok(("", vec!["exp", "exp"])));
    assert_eq!(con_dim("aaaa"),     Ok(("aaaa", vec![])));
  }
}
