//! Módulo que se encarga de acceder a valores constantes o de variables.

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

/// Función auxiliar que agrega constante a tabla de constantes.  
///
/// # Parametros
///
/// * `valor` - Input a parsear
/// * `tipo` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// agregar_constante_a_tabla("10", "entero");
fn agregar_constante_a_tabla(valor: &str, tipo: &str) {
  let mut pila_valores = PILA_VALORES.lock().unwrap();
  pila_valores.push(CONSTANTES.lock().unwrap().agregar_constante(valor.to_owned(), tipo.to_owned()));
  drop(pila_valores);
}

/// Función auxiliar para _parsear_ los diferentes valores constantes.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match valor_cte("10") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn valor_cte(input: &str) -> IResult<&str, &str> {
  alt((num_flotante, num_entero, caracter))(input)
  .map(|(next_input, res)| {
    agregar_constante_a_tabla(res.0, res.1);
    (next_input, "valor_cte")
  })
}

/// Función auxiliar para generar cuadruplo de asignación del valor de una función a un temporal para guardar el valor del return.  
///
/// # Parametros
///
/// * `id_func` - ID de la función
///
/// # Ejemplo
///
/// ```
/// agregar_cuadruplo_asignacion_return_funcion("variable");
fn agregar_cuadruplo_asignacion_return_funcion(id_func: &str) {
  let funcion;

  // Busca función
  {
    let contexto_clase = CONTEXTO_CLASE.lock().unwrap();
    if contexto_clase.clone() != "".to_owned() {
      funcion = match CLASES.lock().unwrap().buscar_metodo(contexto_clase.clone(), id_func.to_owned()) {
        Ok((_, _, func)) => func,
        Err(err) => {
          println!("{:?}", err);
          return;
        }
      };
    } else {
      funcion = match FUNCIONES.lock().unwrap().buscar_funcion(id_func.to_owned()) {
        Ok((_, func)) => func,
        Err(err) => {
          println!("{:?}", err);
          return;
        }
      };
    }
  }

  // Genera cuadruplo de asignación del valor del return de la función a un temporal
  match CUADRUPLOS.lock().unwrap().agregar_cuadruplo_asignacion_valor_funcion(funcion.direccion, funcion.tipo.clone()) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    }
  };
}

/// Función auxiliar checar un parentesis izquierdo al principio de una dimensión, esto para evitar una ambigüedad de tipos al llamar el _scanner_ de _nom_.  
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match parentesis("(") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn parentesis(input: &str) -> IResult<&str, &str> {
  tag("(")(input)
}

/// Función auxiliar para _parsear_ el acceso a una variables o llamada a función del lado derecho de una asignación.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match valor_id("func(10, i * 2)") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn valor_id(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;
  let id_valor: &str;
  let mut _id_attr: &str;

  // Lee id
  next = match id(next) {
    Ok((next_input, id_v)) => {
      id_valor = id_v;
      next_input
    },
    Err(err) => return Err(err)
  };

  // Lee posible acceso a método/atributo de un objeto
  next = match preceded(tuple((ws, tag("."), ws)), id)(next) {
    Ok((next_input, id_obj)) => {
      _id_attr = id_obj;
      next_input
    },
    Err(_) => next
  };

  // Busca un parentesis
  match parentesis(next) {
    Ok((next_input, _)) => {
      // Determina que estamos en una llamada a función
      next = next_input;
      // Agrega fondo falso a la pila de operadores
      { PILA_OPERADORS.lock().unwrap().push("(".to_owned()); }
      // Genera cuadruplo de era
      let params = generar_cuadruplo_era(id_valor);
      let mut pos: usize = 0;
      let mut continuar = true;

      // Lee el primer argumento
      next = match preceded(ws, exp)(next) {
        Ok((next_input, _)) => {
          // Checa que la función reciba parametros
          if pos >= params.len() {
            println!("Se excedió la cantidad de parametros dentro de la llamada a función");
          } else {
            // Genera cuadruplo param
            generar_cuadruplo_param(params[pos].clone());
            pos += 1;
          }
          next_input
        },
        Err(_) => {
          continuar = false;
          next
        }
      };

      // Checa si debe seguir leyendo argumentos
      if continuar {
        loop {
          // Checa que haya una coma, indicando que hay otro argumento
          next = match tuple((ws, tag(",")))(next) {
            Ok((next_input, _)) => next_input,
            Err(_err) => break
          };
      
           // Lee argumento
          next = match preceded(ws, exp)(next) {
            Ok((next_input, _)) => {
              if pos >= params.len() {
                println!("Se excedió la cantidad de parametros dentro de la llamada a función");
              } else {
                generar_cuadruplo_param(params[pos].clone());
                pos += 1;
              }
              next_input
            },
            Err(err) => return Err(err)
          };
        }
      }

      // Lee fin de llamada a función
      match tuple((ws, tag(")")))(next) {
        Ok((next_input, _)) => {
          // Elimina fondo falso, genera cuadruplo de gosub y guarda el valor del return en una variable temporal
          { PILA_OPERADORS.lock().unwrap().pop(); }
          generar_cuadruplo_gosub(id_valor);
          agregar_cuadruplo_asignacion_return_funcion(id_valor);
          Ok((next_input, "valor_id"))
        },
        Err(err) => Err(err)
      }
    },
    Err(_) => {
      // Determina que estamos accediendo a un elemento
      con_dim(id_valor, false)(next)
      .map(|(next_input, _)| {
        (next_input, "valor_id")
      })
    }
  }
}

/// No terminal de valor. Sirve para _parsear_ valores constantes o de variables.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```
/// num_entero | num_flotante | caracter | VALOR_ID
/// ```
///
/// # Ejemplo
///
/// ```
/// match valor("arr[10 + i]") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
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
    let id = "abr";
    let mut tabla_variables = VARIABLES.lock().unwrap();
    tabla_variables.agregar_variable(id.to_owned(), "entero".to_owned(), vec![5], 200);
    drop(tabla_variables);
    assert_eq!(valor_id("abr"),                                  Ok(("", "valor_id")));
    assert_eq!(valor_id("abr[10]"),                                  Ok(("", "valor_id")));
  }
}