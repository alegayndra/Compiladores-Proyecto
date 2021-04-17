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
use crate::parser::asginacion::*;
use crate::parser::funcion_esp::*;

fn estatuto_func_esp(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
  
}

pub fn estatuto(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
  alt((asginacion, funcion_esp))(input)
}
  