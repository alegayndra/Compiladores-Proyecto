use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};
  
use crate::scanners::ws::*;
use crate::scanners::id::*;

pub fn asginacion(input: &str) -> IResult<&str, ((&str,Vec<&str>), &str)> {
  tuple((id_parser, ws, tag("="), ws, tag("exp"), ws, tag(";")))(input)
  .map(|(next_input, res)| {
    let (id_parser, _, _, _, exp, _, _) = res;
    (next_input, (id_parser, exp))
  })
}
  