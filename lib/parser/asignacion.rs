use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::many0,
    IResult,
    sequence::tuple,
  };
  
  use crate::scanners::ws::*;
  use crate::scanners::id::*;
  use crate::scanners::texto::*;
  
  fn asginacion(input: &str) -> IResult<&str, (&str,&str)> {
    tuple((id, ws, tag("="), ws, id, ws, tag(";")))(input)
    .map(|(next_input, res)| {
        let (id, _, _, _, exp, _, _) = res;
        (next_input, (id, exp))
    })
  }
  