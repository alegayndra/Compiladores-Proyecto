//! Módulo que se encarga de las diferentes expresiones.
//! Se encarga de las diferentes operaciones (aritméticas, relacionales y lógicas), al igual que el acceder a un valor constante o de una variable.

pub mod exp;
pub mod expresion;
pub mod exp_logica;
pub mod factor;
pub mod termino;
pub mod valor;

use crate::semantica::globales::*;

/// Función auxiliar quegenera un cuadruplo de operación.
///
/// # Ejemplo
///
/// ```ignore
/// checar_lista_operadores();
/// ```
fn generar_cuadruplo_operacion(op: &str) {
  let mut pila_val = PILA_VALORES.lock().unwrap();

  // Saca los últimos dos valores de la pila de valores
  let der = match pila_val.pop() {
    Some(val) => val,
    _ => return
  };
  let izq = match pila_val.pop() {
    Some(val) => val,
    _ => {
      println!("Stack de valores vacío en EXP_LOGICA");
      return;
    }
  };

  drop(pila_val);

  // Genera cuadruplo
  match CUADRUPLOS.lock().unwrap().agregar_cuadruplo(&op, izq, der) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    },
  };
}
