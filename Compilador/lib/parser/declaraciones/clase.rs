use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::declaraciones::funcion::*;
use crate::parser::declaraciones::variables::*;
use crate::semantica::globales::*;

// fn herencia(input: &str) -> IResult<&str, &str> {
//   tuple((tag("<"), ws, id, ws, tag(">")))(input)
//   .map(|(next_input, res)| {
//     let (_, _, id, _, _,) = res;
//     (next_input, id)
//   })
// }

fn atributos(input: &str) -> IResult<&str, &str> {
  variables(input)
  .map(|(next_input, _res)| {
    (next_input, "atributos")
  })
}

fn metodos(input: &str) -> IResult<&str, &str> {
  funcion(input)
  .map(|(next_input, _res)| {
    (next_input, "metodos")
  })
}

fn variable_funcion(input: &str) -> IResult<&str,  &str> {
  alt((atributos, metodos))(input)
}

fn lista_variable_funcion(input: &str) -> IResult<&str, &str> {
  many0(tuple((variable_funcion, ws)))(input)
  .map(|(next_input, _)| {
    (next_input, "lista_variable_funcion")
  })
}

pub fn clase(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;
  let id_clase : &str;
  let id_padre : &str = "";
  
  next = match tuple((ws, tag("clase"), necessary_ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  next = match id(next) {
    Ok((next_input, id_c)) => {
      id_clase = id_c;
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match ws(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  // next = match herencia(input) {
  //   Ok((next_input, id_p)) => {
  //     id_padre = id_p;
  //     next_input
  //   },
  //   _ => {
  //     id_padre = "";
  //     next
  //   }
  // };

  // if id_clase == id_padre {
  //   println!("{:?}", ("ID de clase y de padre iguales.", id_padre));
  // } else {
    match CLASES.lock().unwrap().agregar_clase(id_clase.to_owned(), id_padre.to_owned()) {
      Ok(res) => {
        println!("{:?}", res);
        let mut contexto_clase = CONTEXTO_CLASE.lock().unwrap();
        *contexto_clase = id_clase.to_owned();
      },
      Err(err) => {
        println!("{:?}", err);
      },
    };
  // }

  match tuple((ws, tag("{"), ws, lista_variable_funcion, ws, tag("}"), ws, tag(";"), ws))(next) {
    Ok((next_input, _)) => {
      let mut contexto_clase = CONTEXTO_CLASE.lock().unwrap();
      *contexto_clase = "".to_owned();
      Ok((next_input, "clase"))
    },
    Err(err) => Err(err)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn test_herencia() {
  //   assert_eq!(herencia("<Persona>"),   Ok(("", "Persona")));
  //   assert_eq!(herencia("< Persona >"), Ok(("", "Persona")));
  // }

  #[test]
  fn test_atributos() {
    assert_eq!(atributos("Persona id, id;"),   Ok(("", "atributos")));
    assert_eq!(atributos("entero id[10][7];"), Ok(("", "atributos")));
  }

  #[test]
  fn test_metodos() {
    assert_eq!(metodos("void funcion func (entero var) {  regresa expresion ; }"), Ok(("", "metodos")));
  }

  #[test]
  fn test_variable_funcion() {
    assert_eq!(variable_funcion("Persona id, id;"),                                    Ok(("", "atributos")));
    assert_eq!(variable_funcion("void funcion func (entero var){regresa expresion;}"), Ok(("", "metodos")));
  }

  #[test]
  fn test_lista_variable_funcion() {
    assert_eq!(lista_variable_funcion("Persona id, id;"),                                    Ok(("", "lista_variable_funcion")));
    assert_eq!(lista_variable_funcion("void funcion func (entero var){regresa expresion;}"), Ok(("", "lista_variable_funcion")));
  }

  #[test]
  fn test_clase() {
    assert_eq!(clase("clase Estudiante {};"),           Ok(("", "clase")));
    assert_eq!(clase("clase Estudiante {};"), Ok(("", "clase")));
    assert_eq!(clase(
      "clase Estudiante {
        char nombre[10], apellido[10];
      };"
    ), Ok(("", "clase")));
  }
}
