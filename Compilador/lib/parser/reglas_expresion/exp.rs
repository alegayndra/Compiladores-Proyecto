//! Módulo que se encarga de las expresiones de suma/resta.

use nom::{
  IResult,
  sequence::delimited,
  combinator::opt
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::termino::*;
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
  // Checa que el operador sea suma/resta
  match lista_operadores.pop() {
    Some(op) => {
      match op_sumsub(&op) {
        Ok(_) => {
          generar_cuadruplo_operacion(&op);
        },
        Err(_) => { lista_operadores.push(op); }
      }
    },
    _ => ()
  }
  drop(lista_operadores);
}

/// No terminal de exp. Sirve para las operaciones de lógica con los operadores '+' y '-'  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// TERMINO + TERMINO
/// TERMINO - TERMINO
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match exp_logica("10 & 0") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn exp(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  // Lee una expresión
  next = match termino(next) {
    Ok((next_input, _)) => {
      checar_lista_operadores();
      next_input
    },
    Err(err) => return Err(err)
  };

  // Itera sobre posibles expresiones extra
  loop {
    // Checa que haya un operador, indicando otra expresion
    next = match opt(delimited(ws, op_sumsub, ws))(next) {
      Ok((next_input, Some(operador))) => {
        // Agrega operador a la pila de operadores
        PILA_OPERADORS.lock().unwrap().push(operador.to_owned());
        next_input
      },
      _ => {
        return Ok((next, "exp"));
      }
    };

    // Lee una expresión
    next = match termino(next) {
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
  fn test_exp() {
    assert_eq!(exp("abr  "), Ok(("  ", "exp")));
    assert_eq!(exp("num_entero"), Ok(("", "exp")));
    assert_eq!(exp("id"), Ok(("", "exp")));
    assert_eq!(exp("id  "), Ok(("  ", "exp")));
    assert_eq!(exp("10  "), Ok(("  ", "exp")));
    assert_eq!(exp("id * num_entero"), Ok(("", "exp")));
    assert_eq!(exp("id + num_entero"), Ok(("", "exp")));
    assert_eq!(exp("id + num_entero * id2 - num_entero - termino"), Ok(("", "exp")));
  }
}