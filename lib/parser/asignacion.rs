use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};
  
use crate::scanners::ws::*;
use crate::scanners::id::*;

fn vec_asignacion(input: &str) -> IResult<&str, (Vec<(&str,Vec<&str>)>)> {
  id_parser(input)
  .map(|(next_input, res)| {
    let (id_parseado) = res;
    let mut lista = Vec::new();
    lista.push(id_parseado);
    (next_input, (id_parser, exp))
  })
}

pub fn asginacion(input: &str) -> IResult<&str, ((&str,Vec<&str>), &str)> {
  tuple((vec_asignacion, ws, tag("="), ws, tag("exp"), ws, tag(";")))(input)
  .map(|(next_input, res)| {
    let (id_parser, _, _, _, exp, _, _) = res;
    (next_input, (id_parser, exp))
  })
}
  