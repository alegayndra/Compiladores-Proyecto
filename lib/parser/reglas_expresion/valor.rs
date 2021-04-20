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
use crate::scanners::constantes::*;
use crate::parser::dimensiones::*;
use crate::parser::func_params::*;

fn valor_cte(input: &str) -> IResult<&str, (&str, &str)> {
  alt((num_entero, tag("num_float"), texto))(input)
  .map(|(next_input, res)| {
    (next_input, (res, "constante"))
  })
}

fn dim_normalizado(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  con_dim(input)
  .map(|(next_input, res)| {
    (next_input, ("dimensiones", res))
  })
}

fn valor_id(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((
    id,
    many0(tuple((ws, tag("."), ws, id))), ws, 
    alt((func_params, dim_normalizado))
  ))(input)
  .map(|(next_input, res)| {
    let (id, _atributos, _, _dim_func) = res;
    (next_input,(id, "variable"))
  })
}

pub fn valor(input: &str) -> IResult<&str, (&str, &str)> {
  alt((valor_cte, valor_id))(input)
  .map(|(next_input, res)| {
    (next_input, res)
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
  fn test_valor_cte() {
    assert_eq!(valor_cte("\"soyUnaVariable\""), Ok(("", ("\"soyUnaVariable\"", "constante"))));
    assert_eq!(valor_cte("10"), Ok(("", ("10", "constante"))));
    assert_eq!(valor_cte("num_float"), Ok(("", ("num_float", "constante"))));
  }

  //Hace las mismas pruebas de lib > parser > dim - "con_dim()", solo regresa valor distinto
  #[test]
  fn test_dim_normalizado() {
    assert_eq!(dim_normalizado("[id][id]"), Ok(("", ("dimensiones", vec!["expresion", "expresion"]))));
    assert_eq!(dim_normalizado("[id][id]"), Ok(("", ("dimensiones", vec!["expresion", "expresion"]))));
  }

  #[test]
  fn test_valor_id() {
    assert_eq!(valor_id("SoyUnString.arreglo[id]"),             Ok(("", ("SoyUnString", "variable"))));
    assert_eq!(valor_id("Objeto.metodo.arreglo[id][id]"),       Ok(("", ("Objeto", "variable"))));
    assert_eq!(valor_id("Nombre  .metodo. arreglo[  id][id ]"), Ok(("", ("Nombre", "variable"))));
    assert_eq!(valor_id("Nombre.metodo(expresion)"),            Ok(("", ("Nombre", "variable"))));
    assert_eq!(valor_id("Nombre.metodo(expresion)"),            Ok(("", ("Nombre", "variable"))));
    assert_eq!(valor_id("Nombre.metodo ()"),                    Ok(("", ("Nombre", "variable"))));
  }
}