use nom::{
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;

pub fn programa(input: &str) -> IResult<&str, (&str, &str, &str)> {
  tuple((tag("programa"), necessary_ws, id, ws, tag(";"), ws, tag("declaraciones"), ws, tag("principal()"), ws, tag("bloque")))
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
    assert_eq!(programa("programa idPrograma; declaraciones principal() bloque"), Ok(("", ("", "declaraciones", "bloque"))));
  }
}
