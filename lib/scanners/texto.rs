use nom::{
  bytes::complete::{tag, take_while},
  IResult,
  sequence::{delimited},
};

pub fn texto(input: &str) -> IResult<&str, &str> {
  match delimited(tag("\""), take_while(|c: char| c.is_alphanumeric()), tag("\""))(input) {
    Ok(res) => Ok(res),
    Err(err) => Err(err)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_texto() {
    assert_eq!(texto("\"a\""),      Ok(("", "a")));
  }
}
