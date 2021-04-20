use nom::{
  IResult,
  sequence::tuple,
  bytes::complete::tag
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::declaraciones::declaraciones::*;
use crate::parser::bloque::*;

pub fn programa(input: &str) -> IResult<&str, (&str, Vec<&str>, &str)> {
  tuple((ws, tag("programa"), necessary_ws, id, ws, tag(";"), ws, declaraciones, ws, tag("principal()"), ws, bloque, ws))
  (input)
  .map(|(next_input, res)| {
    let (_, _, _, id, _, _, _, declaraciones, _, _, _, bloque, _) = res;
    (next_input, (id, declaraciones, bloque))
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
  fn test_programa() {
    // assert_eq!(programa("programa idPrograma; clase principal() bloque"), Ok(("", ("idPrograma", vec!["clase"], "bloque"))));
    // assert_eq!(programa("programa idPrograma; principal() bloque"), Ok(("", ("idPrograma", vec![], "bloque"))));
    // assert_eq!(programa("programa idPrograma; clase, variables principal() bloque"), Ok(("", ("idPrograma", vec!["clase", "variables"], "bloque"))));
    assert_eq!(programa("
      programa idPrograma;
      principal() {}"
    ), Ok(("", ("idPrograma", vec![], "bloque"))));
    assert_eq!(programa("
      programa idPrograma;
      principal() {
        %% comentario %%
      }"
    ), Ok(("", ("idPrograma", vec![], "bloque"))));
    assert_eq!(programa("
      programa idPrograma;
      entero num;
      principal() {}"
    ), Ok(("", ("idPrograma", vec!["variables"], "bloque"))));
    assert_eq!(programa("
      programa idPrograma;
      clase Estudiante <Persona> {
        char nombre[10], apellido[10];
      };
      principal() {}"
    ), Ok(("", ("idPrograma", vec!["clase"], "bloque"))));
    assert_eq!(programa("
      programa idPrograma;
      void funcion func (entero var): {
        estatuto;
        regresa expresion;
      }
      principal() {}"
    ), Ok(("", ("idPrograma", vec!["funcion"], "bloque"))));
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
    ), Ok(("", ("idPrograma", vec!["funcion", "variables", "clase"], "bloque"))));
  }
}
