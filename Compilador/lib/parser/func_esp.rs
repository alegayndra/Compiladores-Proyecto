use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple,
  combinator::opt
};

use crate::scanners::ws::*;
use crate::scanners::texto::*;
use crate::parser::reglas_expresion::expresion::*;
use crate::parser::reglas_expresion::valor::*;
use crate::semantica::globales::*;

fn generar_cuadruplo_lectura() {
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  let var = PILA_VALORES.lock().unwrap().pop().unwrap();

  match cuadruplos.agregar_cuadruplo_lectura(var) {
    Ok(_res) => (),
    Err(err) => {
      println!("{:?}", err);
    },
  };
  drop(cuadruplos);
}

fn generar_cuadruplo_escritura() {
  match PILA_VALORES.lock().unwrap().pop() {
    Some(valor) => {
      match CUADRUPLOS.lock().unwrap().agregar_cuadruplo_escritura(valor) {
        Ok(_res) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      };
    },
    _ => {
      println!("Stack de valores vacío en ESCRIBIR");
    }
  };
}

pub fn leer(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  next = match tuple((tag("lee"), ws, tag("("), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  next = match valor(next) {
    Ok((next_input, _)) => {
      generar_cuadruplo_lectura();
      next_input
    },
    Err(err) => return Err(err)
  };

  loop {
    next = match opt(tuple((ws, tag(","), ws)))(next) {
      Ok((next_input, Some(_))) => next_input,
      _ => break
    };

    next = match valor(next) {
      Ok((next_input, _)) => {
        generar_cuadruplo_lectura();
        next_input
      },
      Err(err) => return Err(err)
    };
  };

  match tuple((ws, tag(")"), tag(";")))(next) {
    Ok((next_input, _)) => Ok((next_input, "leer")),
    Err(err) => Err(err)
  }
}

fn agregar_texto_a_tabla(valor: &str) {
  let mut pila_valores = PILA_VALORES.lock().unwrap();
  pila_valores.push(CONSTANTES.lock().unwrap().agregar_constante(valor.to_owned(), "texto".to_owned()));
  drop(pila_valores);
  generar_cuadruplo_escritura();
}

pub fn escribir(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  next = match tuple((tag("escribe"), ws, tag("("), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  next = match texto(next) {
    Ok((next_i, texto_const)) => {
      agregar_texto_a_tabla(texto_const);
      next_i
    },
    Err(_) => {
      match expresion(next) {
        Ok((next_input, _)) => {
          generar_cuadruplo_escritura();
          next_input
        },
        Err(err) => return Err(err)
      }
    }
  };

  loop {
    next = match opt(tuple((ws, tag(","), ws)))(next) {
      Ok((next_input, Some(_))) => next_input,
      _ => break
    };

    next = match texto(next) {
      Ok((next_i, texto_const)) => {
        agregar_texto_a_tabla(texto_const);
        next_i
      },
      Err(_) => {
        match expresion(next) {
          Ok((next_input, _)) => {
            generar_cuadruplo_escritura();
            next_input
          },
          Err(err) => return Err(err)
        }
      }
    };
  };

  match tuple((ws, tag(")"), tag(";")))(next) {
    Ok((next_input, _)) => Ok((next_input, "escribir")),
    Err(err) => Err(err)
  }
}

pub fn funcion_esp(input: &str) -> IResult<&str, &str> {
  alt((leer, escribir))(input)
  .map(|(next_input, _)| {
    (next_input, "funcion_esp")
  })
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_leer() {
    assert_eq!(leer("lee(id);"),        Ok(("", "leer")));
    assert_eq!(leer("lee ( id );"),     Ok(("", "leer")));
    assert_eq!(leer("lee ( id, id );"), Ok(("", "leer")));
  }

  #[test]
  fn test_escribir() {
    assert_eq!(escribir("escribe(id);"),                        Ok(("", "escribir")));
    assert_eq!(escribir("escribe(\"abr\");"),                   Ok(("", "escribir")));
    assert_eq!(escribir("escribe ( id );"),                     Ok(("", "escribir")));
    assert_eq!(escribir("escribe(\"abr\", id, id, \"abr\");"),  Ok(("", "escribir")));
  }
}
