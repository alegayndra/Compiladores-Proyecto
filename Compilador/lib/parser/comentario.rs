use nom::{
  bytes::complete::{tag, take_while},
  IResult,
  sequence::tuple,
};

pub fn comentario(input: &str) -> IResult<&str, &str> {
  tuple((tag("%%"), take_while(|c: char| c != '%'), tag("%%")))
  (input)
  .map(|(next_input, _)| {
    (next_input, "comentario")
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_comentario() {
    assert_eq!(comentario("%%%%"),      Ok(("", "comentario")));
    assert_eq!(comentario("%%  %%"),    Ok(("", "comentario")));
    assert_eq!(comentario("%% aaa %%"), Ok(("", "comentario")));
    assert_eq!(comentario("%%
      aaa
    %%"), Ok(("", "comentario")));
  }
}
