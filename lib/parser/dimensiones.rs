use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::tipos::*;
use crate::scanners::id::*;

fn dimension(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((tag("["), ws, tag("id"), ws, tag("]")))
  (input)
  .map(|(next_input, res)| {
    let (_, _, dimension, _, _,) = res;
    let mut lista_dimensiones = Vec::new();
    lista_dimensiones.push(dimension);
    (next_input, lista_dimensiones)
  })
}

fn dos_dimensiones(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((dimension, dimension))
  (input)
  .map(|(next_input, res)| {
    let (dimension_1, dimension_2) = res;
    let mut lista_dimensiones = Vec::new();
    lista_dimensiones.push(dimension_1[0]);
    lista_dimensiones.push(dimension_2[0]);
    (next_input, lista_dimensiones)
  })
}

pub fn ws_vec(input: &str) -> IResult<&str, Vec<&str>> {
  ws(input)
  .map(|(next_input, res)| {
    let mut vector = Vec::new();
    vector.push("");
    (next_input, vector)
  })
}

pub fn con_dim(input: &str) -> IResult<&str, Vec<&str>> {
  alt((dos_dimensiones, dimension, ws_vec))
  (input)
}