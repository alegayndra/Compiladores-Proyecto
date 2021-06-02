//! Módulo que se encarga de las asignaciones.

use nom::{
  bytes::complete::tag,
  IResult,
  sequence::{tuple, preceded},
};
  
use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::exp::*;
use crate::parser::dimensiones::*;
use crate::semantica::globales::*;
use crate::semantica::tabla_variables::*;

/// Función auxiliar para generar el cuadruplo de asignación.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `variable` - Variable a la cual se le quiere asignar el valor
///
/// # Ejemplo
///
/// ```
/// generar_cuadruplo_asignacion(TipoVar {
///   nombre: "numero".to_owned()
///   tipo: "entero".to_owned()
///   dimensiones: vec![]
///   direccion: 200
/// });
/// ```
fn generar_cuadruplo_asignacion(variable: TipoVar) {
  let mut pila_valores = PILA_VALORES.lock().unwrap();
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();

  // Consigue el último valor dentro de la pila de valores
  let valor = match pila_valores.pop() {
    Some(valor) => valor,
    _ => { println!("Stack de valores vacío en EXP_LOGICA"); return; }
  };

  drop(pila_valores);

  // Checa si la variable es un elemento atómico
  // En el caso que sí, genera un cuadruplo normal de asignación
  // En el caso que no, genera un cuadruplo especial para la asignación a un elemento no atómico
  match valor.dimensiones.len() {
    0 => {
      match cuadruplos.agregar_cuadruplo_asignacion(variable, valor) {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      };
    },
    _ => {
      match cuadruplos.agregar_cuadruplo_asignacion_arreglo(variable, valor) {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      };
    }
  }
}

/// Función auxiliar de asignacion sin el ; para permitir el uso en la asignación del _desde_.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match asignacion_interna("num = 10") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn asignacion_interna(input: &str) -> IResult<&str, &str> {
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

  next = match con_dim(id_valor)(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
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
      return Err(err);
    }
  };

  Ok((next, "asignacion_interna"))
}

/// No terminal de asignacion.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```
/// id DIMENSIONES = EXP ;
/// ```
///
/// # Ejemplo
///
/// ```
/// match asignacion("num = 10;") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn asignacion(input: &str) -> IResult<&str, &str> {
  match tuple((asignacion_interna, ws, tag(";")))(input) {
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
  