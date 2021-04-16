use nom::{
  bytes::complete::{take_while1, take_while},
  IResult,
};

pub fn ws(input: &str) -> IResult<&str, &str> {
  take_while(|c: char| c == ' ' || c == '\n' || c == '\t')(input)
}

pub fn necessary_ws(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c == ' ' || c == '\n' || c == '\t')(input)
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_ws() {
      assert_eq!(ws(""), Ok(("", "")));
      assert_eq!(ws("  "), Ok(("", "  ")));
      assert_eq!(ws("\n"), Ok(("", "\n")));
      assert_eq!(ws("\n   \t"), Ok(("", "\n   \t")));
      assert_eq!(ws("a"), Ok(("a", "")));
  }

  #[test]
  fn test_necessary_ws() {
      assert_eq!(necessary_ws("  "), Ok(("", "  ")));
      assert_eq!(necessary_ws(" "), Ok(("", " ")));
      assert_eq!(necessary_ws("\n"), Ok(("", "\n")));
      assert_eq!(necessary_ws("\n   \t"), Ok(("", "\n   \t")));
      assert_eq!(necessary_ws(" a"), Ok(("a", " ")));
  }
}
