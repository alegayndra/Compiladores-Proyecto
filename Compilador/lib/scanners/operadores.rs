use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult
};

pub fn op_sumsub(input: &str) -> IResult<&str, &str> {
  alt((tag("+"), tag("-")))(input)
}

pub fn op_multdiv(input: &str) -> IResult<&str, &str> {
  alt((tag("/"), tag("*")))(input)
}

pub fn op_relacional(input: &str) -> IResult<&str, &str> {
  alt((tag("<="), tag("=="), tag(">="), tag("!="), tag("<"), tag(">")))(input)
}

pub fn op_logica(input: &str) -> IResult<&str, &str> {
  alt((tag("&"), tag("|")))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_op_sumsub() {
      assert_eq!(op_sumsub("+"), Ok(("", "+")));
      assert_eq!(op_sumsub("-"), Ok(("", "-")));
  }

  #[test]
  fn test_op_multdiv() {
      assert_eq!(op_multdiv("/"), Ok(("", "/")));
      assert_eq!(op_multdiv("*"), Ok(("", "*")));
  }

  #[test]
  fn test_op_relacional() {
      assert_eq!(op_relacional("<="), Ok(("", "<=")));
      assert_eq!(op_relacional(">="), Ok(("", ">=")));
      assert_eq!(op_relacional("=="), Ok(("", "==")));
      assert_eq!(op_relacional("!="), Ok(("", "!=")));
      assert_eq!(op_relacional(">"), Ok(("", ">")));
      assert_eq!(op_relacional("<"), Ok(("", "<")));
  }

  #[test]
  fn test_op_logica() {
      assert_eq!(op_logica("&"), Ok(("", "&")));
      assert_eq!(op_logica("|"), Ok(("", "|")));
  }
}
