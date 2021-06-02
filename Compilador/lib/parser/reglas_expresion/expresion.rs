//! Módulo que se encarga de las expresiones relacionales.

use nom::{
  IResult,
  sequence::{tuple, preceded},
  combinator::opt,
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::exp::*;
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
  // Checa que el operador sea relacional
  match lista_operadores.pop() {
    Some(op) => {
      match op_relacional(&op) {
        Ok(_) => {
          generar_cuadruplo_operacion(&op);
        },
        Err(_) => { lista_operadores.push(op); }
      }
    },
    _ => ()
  }
}

/// Función auxiliar para leer la expresión opcional relacional.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match exp_extra("> 0") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn exp_extra(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;

  // Busca un operador relacional
  next = match opt(preceded(ws, op_relacional))(next) {
    Ok((next_input, Some(operador))) => {
        // Agrega operador a la pila de operadores
      PILA_OPERADORS.lock().unwrap().push(operador.to_owned());
      next_input
    }
    Err(err) => return Err(err),
    Ok((next_input, None)) => next_input 
  };

  // Lee una expresión
  match preceded(ws, exp)(next) {
    Ok((next_input, _)) => {
      checar_lista_operadores();
      Ok((next_input, "termino"))
    },
    Err(err) => Err(err)
  }
}

/// Función auxiliar para manejar la expresión opcional relacional.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match exp_opcional("> 0") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn exp_opcional(input: &str) -> IResult<&str, &str> {
  match opt(exp_extra)(input) {
    Ok((next_input, Some(res))) => Ok((next_input, res)), 
    _ => Ok((input, "exp_opcional"))  
  }
}

/// No terminal de exp_logica. Sirve para las operaciones de lógica con los operadores relacionales  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// EXP > EXP
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match expresion("10 > 0") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn expresion(input: &str) -> IResult<&str, &str> {
  tuple((exp, exp_opcional))(input)
  .map(|(next_input, _res)| {
    (next_input, "expresion")
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
  fn test_expresion() {
    assert_eq!(expresion("termino > termino"),                                        Ok(("", "expresion")));
    assert_eq!(expresion("termino"),                                                  Ok(("", "expresion")));
    assert_eq!(expresion("id + num_entero * id2 - num_entero - termino"),             Ok(("", "expresion")));
    assert_eq!(expresion("id + num_entero * id2 - num_entero - termino > id3"),       Ok(("", "expresion")));
    assert_eq!(expresion("( id + num_entero * id2 - num_entero - termino >= id3 )"),  Ok(("", "expresion")));
  }
}
