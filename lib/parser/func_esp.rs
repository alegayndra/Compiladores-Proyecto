use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1, take_while},
  combinator::value,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::lexer::*;
use crate::lexer::operadores::*;

pub fn leer_parser(input: &str) -> IResult<&str, (&str, &str, &str, Vec<(&str, &str)>, &str)> {
  tuple((tag("lee"), necessary_ws, tag("("), ws, tag("id"),
      many0(tuple((
          ws, tag(","), 
          ws, tag("id")
      ))),
      ws, tag(")")
  ))
  (input)
  .map(|(next_input, res)| {
      let (lee, _, lp, _, id, ids, _, rp) = res;
      let mut lista_ids = Vec::new();
      for sid in ids {
          let (_, coma, _, sid2) = sid;
          lista_ids.push((
              coma,
              sid2
          ));
      }
      (
          next_input,
          (
              lee,
              lp,
              id,
              lista_ids,
              rp
          )
      )
  })
}