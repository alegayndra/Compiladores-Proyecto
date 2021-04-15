use nom::{
  bytes::complete::{tag, take_while1},
  IResult,
  sequence::tuple,
};

pub fn num_entero(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c >= '0' || c <= '9')(input)
}

// pub fn num_flotante(input: &str) -> IResult<&str, &str> {
//   tuple((num_entero, tag("."), num_entero))(input)
//   .map(|(next_input, res)| {
//     let (matisa, _, decimal) = res;
//     let numero = format!("{}.{}", matisa, decimal);
//     (next_input, &numero)
//   })
// }

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_num_entero() {
    assert_eq!(num_entero("1"), Ok(("", "1")));
    assert_eq!(num_entero("11"), Ok(("", "11")));
    assert_eq!(num_entero("1123131"), Ok(("", "1123131")));
  }

  // #[test]
  // fn test_num_flotante() {
  //   assert_eq!(num_flotante("1.1"), Ok(("", "1.1")));
  //   assert_eq!(num_flotante("11.23"), Ok(("", "11.23")));
  //   assert_eq!(num_flotante("112.3131"), Ok(("", "112.3131")));
  // }
}
