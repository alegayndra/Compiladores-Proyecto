//! Módulo que se encarga de las expresiones de multiplicación/división.

use nom::{
  IResult,
  sequence::delimited,
  combinator::opt
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::factor::*;
use crate::parser::reglas_expresion::*;
use crate::semantica::globales::*;

/// Función auxiliar que checa si debe generar un cuadruplo de operación.
///
/// # Ejemplo
///
/// ```ignore
/// checar_lista_operadores();
/// ```
fn checar_lista_operadores() {
  let mut lista_operadores = PILA_OPERADORS.lock().unwrap();
  // Checa que el operador sea de multiplicación/división
  match lista_operadores.pop() {
    Some(op) => {
      match op_multdiv(&op) {
        Ok(_) => {
          generar_cuadruplo_operacion(&op);
        },
        Err(_) => { lista_operadores.push(op); }
      }
      ()
    },
    _ => ()
  }

  drop(lista_operadores);
}

/// No terminal de termino. Sirve para las operaciones de lógica con los operadores '*' y '/'  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// FACTOR & FACTOR
/// FACTOR | FACTOR
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match exp_logica("10 * 0") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn termino(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  // Lee una expresión
  next = match factor(next) {
    Ok((next_input, _)) => {
      checar_lista_operadores();
      next_input
    },
    Err(err) => return Err(err)
  };

  // Itera sobre posibles expresiones extra
  loop {
    // Checa que haya un operador, indicando otra expresion
    next = match opt(delimited(ws, op_multdiv, ws))(next) {
      Ok((next_input, Some(operador))) => {
        // Agrega operador a la pila de operadores
        PILA_OPERADORS.lock().unwrap().push(operador.to_owned());
        next_input
      },
      _ => return Ok((next, "termino"))
    };

    // Lee una expresión
    next = match factor(next) {
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
  fn test_termino() {
    assert_eq!(termino("factor"),                            Ok(("", "termino")));
    assert_eq!(termino("factor * factor * factor / factor"), Ok(("", "termino")));
    assert_eq!(termino("num_entero"),                        Ok(("", "termino")));
    assert_eq!(termino("id"),                                Ok(("", "termino")));
    assert_eq!(termino("id * num_entero * id2 / id3"),       Ok(("", "termino")));
  }
}
