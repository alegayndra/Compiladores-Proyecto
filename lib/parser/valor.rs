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

fn expresiones_vacias(input: &str) -> IResult<&str, Vec<&str>> {
  ws(input)
  .map(|(next_input, res)| {
    (next_input, vec![])
  })
}

fn lista_expresiones(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((
    tag("expresion"), many0(tuple((ws, tag(","), ws, tag("expresion"))))
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

fn func_params(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((tag("("), ws, alt((lista_expresiones, expresiones_vacias)), ws, tag(")")))(input)
  //Llama al no terminal expresion
  .map(|(next_input, res)| {
    let (_, _, expresiones, _, _) = res;
    (next_input, ("expresiones", expresiones))
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
    many0(tuple((ws, tag("."), ws, id))), ws, 
    alt((func_params, dim_normalizado))
  ))
  (input)
  .map(|(next_input, res)| {
    let (id, atributos, _, dimOfunc) = res;
    (next_input,(id,"variable"))
  })
}

pub fn valor(input: &str) -> IResult<&str, (&str,&str)> {
  alt((valor_cte, valor_id ))(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_valor_cte() {
    assert_eq!(valor_cte("\"soyUnaVariable\""), Ok(("", ("soyUnaVariable", "constante"))));
    assert_eq!(valor_cte("num_entero"), Ok(("", ("num_entero", "constante"))));
    assert_eq!(valor_cte("num_float"), Ok(("", ("num_float", "constante"))));
  }

  #[test]
  fn test_func_params() {
    assert_eq!(func_params("(expresion)"), Ok(("", ("expresiones",vec!["expresion"]))));
    assert_eq!(func_params("(  expresion , expresion,expresion )"), Ok(("", ("expresiones",vec!["expresion","expresion","expresion"]))));
    assert_eq!(func_params("()"), Ok(("", ("expresiones",vec![]))));
  }

  //Hace las mismas pruebas de lib > parser > dim - "con_dim()", solo regresa valor distinto
  #[test]
  fn test_dim_normalizado() {
    assert_eq!(dim_normalizado("[id][id]"), Ok(("", ("dimensiones",vec!["id","id"]))));
  }

  #[test]
  fn test_valor_id() {
    assert_eq!(valor_id("SoyUnString.arreglo[id]"), Ok(("", ("SoyUnString", "variable"))));
    assert_eq!(valor_id("Objeto.metodo.arreglo[id][id]"), Ok(("", ("Objeto", "variable"))));
    assert_eq!(valor_id("Variable  .metodo. arreglo[  id][id ]"), Ok(("", ("Variable", "variable"))));
    assert_eq!(valor_id("Variable.metodo(expresion)"), Ok(("", ("Variable", "variable"))));
    assert_eq!(valor_id("Variable.metodo(expresion)"), Ok(("", ("Variable", "variable"))));
    assert_eq!(valor_id("Variable.metodo ()"), Ok(("", ("Variable", "variable"))));
  }
}