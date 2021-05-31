use nom::{
  branch::alt,
  IResult,
};

use crate::parser::asignacion::*;
use crate::parser::func_esp::*;
use crate::parser::llama_func::*;
use crate::parser::repeticion::*;
use crate::parser::decision::*;
use crate::parser::comentario::*;
use crate::parser::regresa::*;


pub fn estatuto(input: &str) -> IResult<&str, &str> {
  alt((asignacion, funcion_esp, llama_func, repeticion, decision, comentario, regresa))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_estatuto() {
    assert_eq!(estatuto("id = 10;"),              Ok(("", "asignacion")));
    assert_eq!(estatuto("lee(expresion);"),       Ok(("", "funcion_esp")));
    assert_eq!(estatuto("metodo();"),             Ok(("", "llama_func")));
    assert_eq!(estatuto("mientras(10 > 10) {}"),  Ok(("", "repeticion")));
    assert_eq!(estatuto("si (10 > 10) {}"),       Ok(("", "decision")));
    assert_eq!(estatuto("%% comentario %%"),      Ok(("", "comentario")));
    assert_eq!(estatuto("regresa char ;"),        Ok(("", "regresa")));
  }
}
