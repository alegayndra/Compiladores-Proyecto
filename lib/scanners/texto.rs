use nom::{
  bytes::complete::{tag, take_while},
  IResult,
  sequence::tuple,
};

pub fn texto(input: &str) -> IResult<&str, &str> {
  tuple((tag("\""), take_while(|c: char| c.is_alphanumeric()), tag("\"")))
  (input)
  .map(|(next_input, res)| {
    let (_, texto, _) = res;
    (next_input, texto)
  })
}