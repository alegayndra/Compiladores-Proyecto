use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
  combinator::opt
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::func_params::*;

fn attr_objeto(input: &str) -> IResult<&str, &str> {
  opt(tuple((ws, tag("."), ws, id)))(input)
  .map(|(next_input, res)| {
    match res {
      Some(val) => (next_input, val.3),
      None => (next_input, "")
    }
  })
}

pub fn llama_func(input: &str) -> IResult<&str, &str> {
  tuple((id, attr_objeto, func_params, ws, tag(";")))(input)
  .map(|(next_input, __res)| {
    (next_input, "llama_func")
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
    assert_eq!(llama_func("id();"),           Ok(("", "llama_func")));
    assert_eq!(llama_func("id.metodo();"),    Ok(("", "llama_func")));
    assert_eq!(llama_func("id(expresion);"),  Ok(("", "llama_func")));
  }
}
