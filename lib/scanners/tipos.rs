use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult
};

pub fn tipo(input: &str) -> IResult<&str, &str> {
  alt((tag("entero"), tag("flotante"), tag("char")))(input)
}

pub fn tipo_compuesto(input: &str) -> IResult<&str, &str> {
  alt((tipo, tag("id")))(input)
}

pub fn tipo_retorno(input: &str) -> IResult<&str, &str> {
  alt((tipo, tag("void")))(input)
}