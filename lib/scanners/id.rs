use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::tipos::*;

pub fn id(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric())(input)
}

pub fn list_ids(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("id"), many0(tuple((ws, tag(","), ws, tag("id"))))))(input)
  .map(|(next_input, res)| {
    let (id, ids) = res;
    let mut lista_ids = Vec::new();
    lista_ids.push(id);
    for sid in ids {
      let (_, _, _, sid2) = sid;
      lista_ids.push(sid2);
    }
    (next_input, lista_ids)
  })
}