//! Módulo que se encarga de las expresiones y acceder a valores constantes y de variables.

use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::{delimited, preceded}
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::valor::*;
use crate::parser::reglas_expresion::exp_logica::*;
use crate::semantica::globales::*;

/// Función auxiliar que elimina el fondo vacío de ciclar las expresiones.  
///
/// # Ejemplo
///
/// ```
/// checar_pila_operadores();
/// ```
fn checar_pila_operadores() {
  let mut lista_operadores = PILA_OPERADORS.lock().unwrap();
  match lista_operadores.pop() {
    Some(op) => {
      match op.as_str() {
        "(" => (),
        _ => {
          println!("No se encontró ( al final del stack de operadores en FACTOR");
          lista_operadores.push(op);
        }
      }
    },
    None => ()
  };
  drop(lista_operadores);
}

/// Función auxiliar para ciclar expresiones.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input`- Input a parsear
///
/// # Ejemplo
///
/// ```
/// match retorna_expresion("10") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn retorna_expresion(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  // Lee parentesis
  next = match tag("(")(next) {
    Ok((next_input, _)) => {
      // Agrega fondo vacío a pila de operadores
      PILA_OPERADORS.lock().unwrap().push("(".to_owned());
      next_input
    },
    Err(err) => return Err(err)
  };

  // Lee expresion
  next = match delimited(ws, exp_logica, ws)(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  // Lee fin parentesis
  match tag(")")(next) {
    Ok((next_input, _)) => {
      checar_pila_operadores();
      Ok((next_input, "retorna_expresion"))
    },
    Err(err) => Err(err)
  }
}

fn op_vacio(input: &str) -> IResult<&str, &str> {
  Ok((input, ""))
}

/// Función auxiliar que agrega un cuadruplo de multipicación para hacer valor negativo.  
///
/// # Parametros
///
/// * `op_valor`- Operador
///
/// # Ejemplo
///
/// ```
/// checar_lista_operadores();
/// ```
fn checar_lista_operadores(op_valor: &str) {
  // Checa que el operador sea negativo
  match op_valor {
    "-" => {
      // Saca el valor del tope de la pila de valores
      let mut pila_val = PILA_VALORES.lock().unwrap();
      let valor = match pila_val.pop() {
        Some(val) => val,
        _ => {
          println!("Stack de valores vacío en VALOR_FACTOR");
          return
        }
      };

      drop(pila_val);

      // Genera cuadruplo de resta
      match CUADRUPLOS.lock().unwrap().agregar_cuadruplo("-", valor.clone(), valor.clone()) {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        }
      };
    }
    _ => ()
  };
}

/// Función auxiliar para acceder a valor.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input`- Input a parsear
///
/// # Ejemplo
///
/// ```
/// match valor_factor("10") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn valor_factor(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;
  let op_valor: &str;

  // Lee operador
  next = match alt((op_sumsub, op_vacio))(next) {
    Ok((next_input, op)) => {
      op_valor = op;
      next_input
    },
    Err(err) => return Err(err)
  };

  // Lee valor
  match preceded(ws, valor)(next) {
    Ok((next_input, _)) => {
      checar_lista_operadores(op_valor);
      Ok((next_input, "valor_factor")) 
    },
    Err(err) => Err(err),
  }
}

/// No terminal de factor. Sirve para las ciclar las expresiones o acceder a un valor constante o de variable.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input`- Input a parsear
///
/// # Gramática
///
/// ```
/// ( EXP_LOGICA )
/// VALOR
/// ```
///
/// # Ejemplo
///
/// ```
/// match factor("10") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn factor(input: &str) -> IResult<&str, &str> {
  alt((retorna_expresion, valor_factor))(input)
  .map(|(next_input, _)| {
    (next_input, "factor")
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valor_factor() {
    assert_eq!(valor_factor("10"),                   Ok(("", "valor_factor")));
    assert_eq!(valor_factor("- 10"),                 Ok(("", "valor_factor")));
    assert_eq!(valor_factor("+ \"s\""),              Ok(("", "valor_factor")));
    assert_eq!(valor_factor("+ Nombre.metodo()"),    Ok(("", "valor_factor")));
    assert_eq!(valor_factor("+ Nombre . metodo()"),  Ok(("", "valor_factor")));
  }

  #[test]
  fn test_factor() {
    assert_eq!(factor("- num_entero"),        Ok(("", "factor")));
    assert_eq!(factor("+ \"s\""),             Ok(("", "factor")));
    assert_eq!(factor("+ Nombre . metodo()"), Ok(("", "factor")));
    assert_eq!(factor("( expresion )"),       Ok(("", "factor")));
    assert_eq!(factor("( 10 )"),              Ok(("", "factor")));
    assert_eq!(factor("( 10 * id )"),         Ok(("", "factor")));
    assert_eq!(factor("( 11 & id )"),         Ok(("", "factor")));
    assert_eq!(factor("( 1 | 0 )"),           Ok(("", "factor")));
  }
}
