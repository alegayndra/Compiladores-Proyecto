//! Módulo que se encarga de las dimensiones.

use nom::{
  IResult,
  sequence::{tuple, preceded},
  bytes::complete::tag
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::declaraciones::declaraciones::*;
use crate::parser::bloque::*;
use crate::semantica::globales::*;

/// Inicio del parser del programa.  
/// Regresa un IResult, un Result nativo modificado de la librería de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input`- Input a parsear
///
/// # Gramática
///
/// ```
/// programa id DECLARACIONES principal() BLOQUE
/// ```
///
/// # Ejemplo
///
/// ```
/// match programa("programa idPrograma principal(){}") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn programa(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;
  let id_programa: &str;
  
  // Marca el inicio del programa y consigue el id del programa
  next = match preceded(tuple((ws, tag("programa"), necessary_ws)), id)(next) {
    Ok((next_input, id)) => {
      id_programa = id;
      next_input
    },
    Err(err) => return Err(err)
  };

  // Crear tabla de variables globales
  {
    match FUNCIONES.lock().unwrap().agregar_funcion(id_programa.to_owned(), "void".to_owned(), -5, 0) {
      Ok(_) => (),
      Err(err) => {
        println!("{:?}", err);
      },
    };
  }

  // Agregar el GOTO al main
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  let mut saltos = PILA_SALTOS.lock().unwrap();
  match cuadruplos.agregar_cuadruplo_goto() {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    },
  };
  saltos.push((cuadruplos.lista.len() - 1) as i64);
  drop(cuadruplos);
  drop(saltos);

  // Actualizar contexto global y guardar id del programa
  {
    *CONTEXTO_FUNCION.lock().unwrap() = id_programa.to_owned();
    *ID_PROGRAMA.lock().unwrap() = id_programa.to_owned();
  }

  // Lee las declaraciones de clases, funciones y variables globales
  next = match tuple((ws, tag(";"), ws, declaraciones, ws, tag("principal()"), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  // Marcar que el contexto actual es el global
  {
    *CONTEXTO_FUNCION.lock().unwrap() = ID_PROGRAMA.lock().unwrap().to_string();
  }

  // Actualicar el GOTO al main
  {
    match PILA_SALTOS.lock().unwrap().pop() {
      Some(valor) => {
        match CUADRUPLOS.lock().unwrap().modificar_cuadruplo_goto(valor as usize) {
          Ok(_) => (),
          Err(err) => {
            println!("{:?}", err);
          }
        };
      },
      _ => {
        println!("Pila de saltos vacía en PRINCIPAL");
      }
    }
  }

  // Lee el bloque de la función principal
  match tuple((bloque, ws))(next) {
    Ok((_, _)) => Ok(("", "programa")),
    Err(err) => Err(err),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_programa() {
    assert_eq!(programa("
      programa programaCompleto;
      entero id, arr[5];
      void funcion func3 (entero var) {
        id = 10;
        regresa 10;
      }
      entero x, d, var;
      principal() {
        x = 10;
        d = 10 + 10;
        lee(var);
        escribe(var);
        mientras ( d > 10 ) {
          escribe(d);
          d = d - 1;
        }

        desde arr[10] = 10 hasta 20 {
          escribe(id);
        }

        %% comentario %%
        si (x > 2) {
          escribe(\"wiiii\");
        }
        si (x > 2) {
          escribe(10);
        } sino {
          escribe(2);
        }
      }"
    ), Ok(("", "programa")));
  }
}
