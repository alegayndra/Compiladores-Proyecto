use nom::{
  IResult,
  sequence::tuple,
  bytes::complete::tag
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::declaraciones::declaraciones::*;

pub fn programa(input: &str) -> IResult<&str, (&str, Vec<&str>, &str)> {
  tuple((tag("programa"), necessary_ws, id, ws, tag(";"), ws, declaraciones, ws, tag("principal()"), ws, tag("bloque")))
  (input)
  .map(|(next_input, res)| {
    let (_, _, id, _, _, _, declaraciones, _, _, _, bloque) = res;
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
  }
}
