use nom::{
  bytes::complete::{tag, take_while},
  IResult,
  sequence::tuple,
};

pub fn comentario(input: &str) -> IResult<&str, &str> {
  tuple((tag("%%"), take_while(|c: char| c != '%'), tag("%%")))
  (input)
  .map(|(next_input, res)| {
    let (_, com, _,) = res;
    (next_input, com)
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
  fn test_comentario() {
    assert_eq!(comentario("%%%%"), Ok(("", "")));
    assert_eq!(comentario("%%  %%"), Ok(("", "  ")));
    assert_eq!(comentario("%% aaa %%"), Ok(("", " aaa ")));
    // assert_eq!(leer("lee()"), Ok(("", vec![])));
  }
}
