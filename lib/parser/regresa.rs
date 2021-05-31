use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::globales::*;

pub fn regresa(input: &str) -> IResult<&str, &str> {
  tuple((tag("regresa"), ws, exp, ws, tag(";")))(input)
  .map(|(next_input, _)| {
    unsafe {
      RETURN_EXISTENTE = true;
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
