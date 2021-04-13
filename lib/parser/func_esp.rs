use nom::{
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::scanners::texto::*;

pub fn leer_parser(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("lee"), necessary_ws, tag("("), ws, lista_ids, ws, tag(")")))
  (input)
  .map(|(next_input, res)| {
    let (lee, _, lp, _, lista_ids, _, rp) = res;
    (next_input, lista_ids)
  })
}

pub fn leer_parser(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("lee"), necessary_ws, tag("("), ws, lista_ids, ws, tag(")")))
  (input)
  .map(|(next_input, res)| {
    let (lee, _, lp, _, lista_ids, _, rp) = res;
    (next_input, lista_ids)
  })
}