use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::scanners::texto::*;
use crate::parser::dimensiones::*;

fn valor_cte(input: &str) -> IResult<&str, (&str,&str)> {
  alt((tag("num_entero"), tag("num_float"), texto))(input)
  .map(|(next_input, res)| {
    (next_input, (res, "constante"))
  })
}


fn func_params(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((
    ws, tag("("), ws, tag("expresion"), many0(tuple((ws, tag(","), ws, tag("expresion")))), ws, tag(")")
  ))(input)
  //Llama al no terminal expresion
  .map(|(next_input, res)| {
    let (_, _, _, exp, expresiones, _, _) = res;
    let mut lista_expresiones = Vec::new();
    lista_expresiones.push(exp);
    for i in expresiones {
      let (_, _, _, expresion) = i;
      lista_expresiones.push(expresion)
    }
    (next_input, ("expresiones", lista_expresiones))
  })
}
fn dim_normalizado(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  con_dim(input)
  //Llama al no terminal expresion
  .map(|(next_input, res)| {
    (next_input, ("dimensiones", res))
  })
}

fn valor_id(input: &str) -> IResult<&str, (&str,&str)> {
  tuple((
    id, 
    many0(tuple((ws, tag("."), ws, id))), 
    alt((func_params, dim_normalizado))
  ))
  (input)
  .map(|(next_input, res)| {
    let (id, atributos, dimOfunc) = res;
    (next_input,(id,"variable"))
  })
}

pub fn valor(input: &str) -> IResult<&str, (&str,&str)> {
  alt((valor_cte, valor_id ))(input)
}
