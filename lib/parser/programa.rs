use nom::{
  IResult,
  sequence::tuple,
  bytes::complete::tag
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::declaraciones::declaraciones::*;
use crate::parser::bloque::*;

pub fn programa(input: &str) -> IResult<&str, &str> {
  let mut next: &str;
  
  next = match tuple((ws, tag("programa"), necessary_ws))(input) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err), 
  };

  let id_programa: &str;

  match id(next) {
    Ok((next_input, id)) => {
      next = next_input;
      id_programa = id;
    },
    Err(err) => return Err(err),
  };

  next = match tuple((ws, tag(";"), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err),
  };

  let decl: Vec<&str>;

  match (declaraciones)(next) {
    Ok((next_input, de)) => {
      next = next_input;
      decl = de;
    },
    Err(err) => return Err(err),
  };

  next = match tuple((ws, tag("principal()"), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err),
  };

  let blo: &str;

  match bloque(next) {
    Ok((next_input, b)) => {
      next = next_input;
      blo = b;
    },
    Err(err) => return Err(err),
  };

  match ws(next) {
    Ok((next_input, _)) => Ok((next_input, "programa")),
    Err(err) => return Err(err),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

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
      clase Estudiante <Persona> {
        char nombre[10], apellido[10];
      };
      principal() {}"
    ), Ok(("", "programa")));
    assert_eq!(programa("
      programa idPrograma;
      void funcion func (entero var): {
        estatuto;
        regresa expresion;
      }
      principal() {}"
    ), Ok(("", "programa")));
    assert_eq!(programa("
      programa idPrograma;
      void funcion func (entero var): {
        estatuto;
        regresa expresion;
      }
      entero num;
      clase Estudiante <Persona> {
        char nombre[10], apellido[10];
      };
      principal() {}"
    ), Ok(("", "programa")));

    assert_eq!(programa("
      programa idPrograma;
      void funcion func (entero var): {
        estatuto;
        regresa expresion;
      }
      entero num;
      clase Estudiante <Persona> {
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
