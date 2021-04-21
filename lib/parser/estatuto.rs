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


pub fn estatuto(input: &str) -> IResult<&str, &str> {
  alt((asignacion, funcion_esp, llama_func, repeticion, decision, comentario))(input)
}
  