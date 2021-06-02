//! Módulo que se encarga de las expresiones lógicas.

use nom::{
  IResult,
  sequence::delimited
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::expresion::*;
use crate::parser::reglas_expresion::*;
use crate::semantica::globales::*;

/// Función auxiliar que checa si debe generar un cuadruplo de operación.
///
/// # Ejemplo
///
/// ```
/// checar_lista_operadores();
/// ```
fn checar_lista_operadores() {
  let mut lista_operadores = PILA_OPERADORS.lock().unwrap();
  // Checa que el operador sea lógico
  match lista_operadores.pop() {
    Some(op) => {
      match op_logica(&op) {
        Ok(_) => {
          generar_cuadruplo_operacion(&op);
        },
        Err(_) => { lista_operadores.push(op); }
      };
    },
    _ => {}
  };

  drop(lista_operadores);
}

/// No terminal de exp_logica. Sirve para las operaciones de lógica con los operadores '&' y '|'  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```
/// EXPRESION & EXPRESION
/// EXPRESION | EXPRESION
/// ```
///
/// # Ejemplo
///
/// ```
/// match exp_logica("10 & 0") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn exp_logica(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  // Lee una expresión
  next = match expresion(next) {
    Ok((next_input, _)) => {
      checar_lista_operadores();
      next_input
    },
    Err(err) => return Err(err)
  };

  // Itera sobre posibles expresiones extra
  loop {
    // Checa que haya un operador, indicando otra expresion
    next = match delimited(ws, op_logica, ws)(next) {
      Ok((next_input, operador)) => {
        // Agrega operador a la pila de operadores
        PILA_OPERADORS.lock().unwrap().push(operador.to_owned());
        next_input
      },
      _ => return Ok((next, "exp_logica"))
    };

    // Lee una expresión
    next = match expresion(next) {
      Ok((next_input, _)) => {
        checar_lista_operadores();
        next_input
      },
      Err(err) => return Err(err)
    };
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_exp_logica() {
    assert_eq!(exp_logica("id"),                                  Ok(("", "exp_logica")));
    assert_eq!(exp_logica("10"),                                  Ok(("", "exp_logica")));
    assert_eq!(exp_logica("id & num_entero"),                     Ok(("", "exp_logica")));
    assert_eq!(exp_logica("id | num_entero"),                     Ok(("", "exp_logica")));
    assert_eq!(exp_logica("id | id > 2 * ( - num_entero + id )"), Ok(("", "exp_logica")));
  }
}
