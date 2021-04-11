use nom::{
  bytes::complete::take_while1,
  IResult,
};

pub fn id(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric())(input)
}