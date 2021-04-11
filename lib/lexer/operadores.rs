use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1, take_while},
  combinator::value,
  multi::many0,
  IResult,
  sequence::tuple,
};

pub fn sumsub_parser(input: &str) -> IResult<&str, &str> {
  alt((tag("+"), tag("-")))(input)
}

pub fn multdiv_parser(input: &str) -> IResult<&str, &str> {
  alt((tag("/"), tag("*")))(input)
}

pub fn op_relacional_parser(input: &str) -> IResult<&str, &str> {
  alt((tag("<="), tag("=="), tag(">="), tag("!="), tag("<"), tag(">")))(input)
}

pub fn op_logica_parser(input: &str) -> IResult<&str, &str> {
  alt((tag("&"), tag("|")))(input)
}