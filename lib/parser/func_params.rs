use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::expresion::*;

fn expresiones_vacias(input: &str) -> IResult<&str, Vec<&str>> {
  ws(input)
  .map(|(next_input, _res)| {
    (next_input, vec![])
  })
}

fn lista_expresiones(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((
    expresion, many0(tuple((ws, tag(","), ws, expresion)))
  ))(input)
   //Llama al no terminal expresion
   .map(|(next_input, res)| {
    let (exp, expresiones) = res;
    let mut lista_expresiones = Vec::new();
    lista_expresiones.push(exp);
    for i in expresiones {
      let (_, _, _, expresion) = i;
      lista_expresiones.push(expresion)
    }
    (next_input, lista_expresiones)
  })
}

pub fn func_params(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((tag("("), ws, alt((lista_expresiones, expresiones_vacias)), ws, tag(")")))(input)
  //Llama al no terminal expresion
  .map(|(next_input, res)| {
    let (_, _, expresiones, _, _) = res;
    (next_input, ("expresiones", expresiones))
  })
}