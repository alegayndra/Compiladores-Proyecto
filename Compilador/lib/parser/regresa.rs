//! Módulo que se encarga de los _returns_ de funciones.

use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::globales::*;

/// No terminal de regresa.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// regresa EXP ;
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match regresa("regresa 10;") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn regresa(input: &str) -> IResult<&str, &str> {
  tuple((tag("regresa"), ws, exp, ws, tag(";")))(input)
  .map(|(next_input, _)| {
    unsafe {
      // Marca que hay un return existente
      RETURN_EXISTENTE = true;

      // Genera cuadruplo de return
      match PILA_VALORES.lock().unwrap().pop() {
        Some(valor) => {
          match CUADRUPLOS.lock().unwrap().agregar_cuadruplo_return(valor.clone(), DIRECCION_CONTEXTO_FUNCION){
            Ok(_) => (),
            Err(err) => {
              println!("{:?}", err);
            }
          };
        },
        None => ()
      }
    }
    (next_input, "regresa")
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_regresa() {
    assert_eq!(regresa("regresa  a;"),  Ok(("", "regresa")));
    assert_eq!(regresa("regresa 0;"),   Ok(("", "regresa")));
  }
}
