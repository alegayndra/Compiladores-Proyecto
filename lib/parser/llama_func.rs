use nom::{
  bytes::complete::tag,
  IResult,
  multi::many0,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::func_params::*;

fn attr_objeto(input: &str) -> IResult<&str, Vec<&str>> {
  many0(tuple((ws, tag("."), ws, id)))(input)
  .map(|(next_input, res)| {
    let mut lista_attr = Vec::new();
    for r in res {
      let (_, _, _, attr) = r;
      lista_attr.push(attr);
    }
    (next_input, lista_attr)
  })
}

pub fn llama_func(input: &str) -> IResult<&str, (&str, Vec<&str>, (&str, Vec<&str>))> {
  tuple((
    id, attr_objeto, func_params, ws, tag(";")
  ))
  (input)
  .map(|(next_input, res)| {
    let (id, atributos, lista_params, _, _) = res;
    (next_input, (id, atributos, lista_params))
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
    fn test_llama_func() {
      assert_eq!(llama_func("id();"), Ok(("", ("id", vec![], ("expresiones", vec![])))));
      assert_eq!(llama_func("id.metodo();"), Ok(("", ("id", vec!["metodo"], ("expresiones", vec![])))));
      assert_eq!(llama_func("id(expresion);"), Ok(("", ("id", vec![], ("expresiones", vec!["expresion"])))));
      assert_eq!(llama_func("id.metodo.metodo2(expresion);"), Ok(("", ("id", vec!["metodo", "metodo2"], ("expresiones", vec!["expresion"])))));
    }

    // #[test]
    // fn test_parametros_vacios() {
    //   assert_eq!(parametros_vacios("Persona id"), Ok(("Persona id", vec![("", ("", vec![]))])));
    //   assert_eq!(parametros_vacios("entero id"), Ok(("entero id", vec![("", ("", vec![]))])));
    //   assert_eq!(parametros_vacios("entero id[id]"), Ok(("entero id[id]", vec![("", ("", vec![]))])));
    // }

    // #[test]
    // fn test_funcion() {
    //   assert_eq!(funcion("void funcion func (entero var): { estatuto; regresa expresion ; }"), Ok(("", ("void", "func", vec![("entero", ("var", vec![]))]))));
    // }
}
