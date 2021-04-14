// use nom::{
//   branch::alt,
//   bytes::complete::tag,
//   IResult,
//   sequence::tuple,
// };

// use crate::scanners::ws::*;
// use crate::scanners::id::*;
// use crate::parser::funcion::*;
// use crate::parser::variables::*;

// fn herencia(input: &str) -> IResult<&str, &str> {
//   tuple((tag("<"), ws, id, ws, tag(">")))(input)
//   .map(|(next_input, res)| {
//     let (_, _, id, _, _,) = res;
//     (next_input, id)
//   })
// }

// fn posible_herencia(input: &str) -> IResult<&str, &str> {
//   alt((herencia, ws))(input)
// }

// fn atributos(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>)> {
//   variables(input)
//   .map(|(next_input, res)| {
//     (next_input, ("null", "variables", vec![res]))
//   })
// }

// fn metodos(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>)> {
//   funcion(input)
//   .map(|(next_input, res)| {
//     let (tipo, id, params) = res;
//     let mut lista_params = Vec::new();
//     for par in params {
//       let (tipo_param, param) = par;
//       lista_params.push((tipo_param, vec![param]))
//     } 
//     (next_input, (tipo, id, lista_params))
//   })
// }

// fn variable_funcion(input: &str) -> IResult<&str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>)> {
//   alt((atributos, metodos))(input)
// }

// pub fn clase(input: &str) -> IResult<&str, (&str, &str, (&str, &str, Vec<(&str, (&str, Vec<&str>))>))> {
//   tuple((
//     tag("clase"), necessary_ws, id, posible_herencia, tag("{"), variable_funcion, tag("}"), ws, tag(";") 
//   ))
//   (input)
//   .map(|(next_input, res)| {
//     let (_, _, id, padre, _, declaraciones, _, _, _) = res;
//     (next_input, (id, padre, declaraciones))
//   })
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   // use nom::{
//   //     error::{ErrorKind, VerboseError, VerboseErrorKind},
//   //     Err,
//   // };

//   #[test]
//   fn test_parametro() {
//     assert_eq!(parametro("Persona id"), Ok(("", ("Persona", ("id", vec![])))));
//     assert_eq!(parametro("entero id"), Ok(("", ("entero", ("id", vec![])))));
//     assert_eq!(parametro("entero id[id]"), Ok(("", ("entero", ("id", vec!["id"])))));
//   }

//   #[test]
//   fn test_parametros_vacios() {
//     assert_eq!(parametros_vacios("Persona id"), Ok(("Persona id", vec![("", ("", vec![]))])));
//     assert_eq!(parametros_vacios("entero id"), Ok(("entero id", vec![("", ("", vec![]))])));
//     assert_eq!(parametros_vacios("entero id[id]"), Ok(("entero id[id]", vec![("", ("", vec![]))])));
//   }

//   #[test]
//   fn test_funcion() {
//     assert_eq!(funcion("void funcion func (entero var): { estatuto; regresa expresion ; }"), Ok(("", ("void", "func", vec![("entero", ("var", vec![]))]))));
//   }
// }
