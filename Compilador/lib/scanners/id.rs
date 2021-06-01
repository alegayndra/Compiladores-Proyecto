use nom::{
  bytes::complete::take_while1,
  IResult,
  sequence::tuple,
};

use crate::parser::dimensiones::*;
use crate::parser::dimensiones_decl::*;

pub fn id(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '-')(input)
}

pub fn id_sin_dim(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((id, ws_vec))(input)
}

pub fn id_con_dim_decl(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
  tuple((id, con_dim_decl))(input)
}

// pub fn id_con_dim(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
//   tuple((id, con_dim))(input)
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_id() {
    assert_eq!(id("id"),        Ok(("", "id")));
    assert_eq!(id("id["),       Ok(("[", "id")));
    assert_eq!(id("aaa123"),    Ok(("", "aaa123")));
    assert_eq!(id("1aa123"),    Ok(("", "1aa123")));
    assert_eq!(id("1aa_123"),   Ok(("", "1aa_123")));
    assert_eq!(id("1aa_123  "), Ok(("  ", "1aa_123")));
    assert_eq!(id("1aa_ 123"),  Ok((" 123", "1aa_")));
  }

  #[test]
  fn test_id_sin_dim() {
    assert_eq!(id_sin_dim("id"),     Ok(("", ("id", vec![]))));
    assert_eq!(id_sin_dim("aaa123"), Ok(("", ("aaa123", vec![]))));
    assert_eq!(id_sin_dim("1aa123"), Ok(("", ("1aa123", vec![]))));
  }

  #[test]
  fn test_id_con_dim() {
    assert_eq!(id_con_dim("id"),     Ok(("", ("id", vec![]))));
    assert_eq!(id_con_dim("id[id]"), Ok(("", ("id", vec!["exp"]))));
  }
}
