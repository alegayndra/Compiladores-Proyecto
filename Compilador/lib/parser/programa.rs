use nom::{
  IResult,
  sequence::tuple,
  bytes::complete::tag
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::declaraciones::declaraciones::*;
use crate::parser::bloque::*;
use crate::semantica::globales::*;

pub fn programa(input: &str) -> IResult<&str, &str> {
  let mut next: &str;
  
  next = match tuple((ws, tag("programa"), necessary_ws))(input) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  let id_programa: &str;

  match id(next) {
    Ok((next_input, id)) => {
      next = next_input;
      id_programa = id;
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

  next = match bloque(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err),
  };

  match ws(next) {
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
      programa idPrograma;
      principal() {}"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      principal() {
        %% comentario %%
      }"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      entero num;
      principal() {}"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      clase Estudiante {
        char nombre[10], apellido[10];
      };
      principal() {}"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      void funcion func (entero var) {
        id = 10;
        regresa expresion;
      }
      principal() {}"
    ), Ok(("", "programa")));
    
    assert_eq!(programa("
      programa idPrograma;
      void funcion func (entero var) {
        id = 10;
        regresa expresion;
      }
      entero num;
      clase Estudiante {
        char nombre[10], apellido[10];
      };
      principal() {}"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      void funcion func (entero var) {
        id = 10;
        regresa expresion;
      }
      entero num;
      clase Estudiante {
        char nombre[10], apellido[10];
      };
      principal() {
        x = 10;
        d = 10 + 10;
        lee(var);
        escribe(var);
        id();
        id(param);
        id.metodo();
        mientras ( id > 10 ) {
          escribe(id);
        }

        desde arr[10] = 10 hasta 20 {
          escribe(id);
        }
        %% comentario %%
        si (id > 2) {
          escribe(id);
        }
        si (id > 2) {
          escribe(id);
        } sino {
          escribe(id);
        }
      }"
    ), Ok(("", "programa")));
  }
}
