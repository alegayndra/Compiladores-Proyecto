use nom::{
    bytes::complete::tag,
    IResult,
    sequence::tuple,
  };
  
  use crate::scanners::ws::*;
  use crate::scanners::id::*;
  
pub fn asginacion(input: &str) -> IResult<&str, (&str,&str)> {
    tuple((id, ws, tag("="), ws, id, ws, tag(";")))(input)
    .map(|(next_input, res)| {
        let (id, _, _, _, exp, _, _) = res;
        (next_input, (id, exp))
    })
  }
  