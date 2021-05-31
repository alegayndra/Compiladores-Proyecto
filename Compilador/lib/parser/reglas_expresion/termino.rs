use nom::{
  IResult,
  sequence::delimited,
  combinator::opt
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::factor::*;
use crate::semantica::globales::*;

fn checar_lista_operadores() {
  let mut lista_operadores = PILA_OPERADORS.lock().unwrap();
  match lista_operadores.pop() {
    Some(op) => {
      match op_multdiv(&op) {
        Ok(_) => {
          let mut pila_val = PILA_VALORES.lock().unwrap();
          let der = match pila_val.pop() {
            Some(val) => val,
            _ => {
              // println!("Stack de valores vacío en TERMINO");
              return;
            }
          };
          let izq = match pila_val.pop() {
            Some(val) => val,
            _ => {
              // println!("Stack de valores vacío en TERMINO");
              return;
            }
          };

          drop(pila_val);

          match CUADRUPLOS.lock().unwrap().agregar_cuadruplo(&op, izq, der) {
            Ok(_res) => { /*println!("{:?}", _res);*/ () },
            Err(_err) => { /*println!("{:?}", _err);*/ () },
          };
        },
        Err(_) => { lista_operadores.push(op); () }
      }
      ()
    },
    _ => {
      // println!("Stack de operadores vacío en TERMINO");
      ()
    }
  }

  drop(lista_operadores);
}

pub fn termino(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  next = match factor(next) {
    Ok((next_input, _)) => {
      checar_lista_operadores();
      next_input
    },
    Err(err) => return Err(err)
  };

  loop {
    next = match opt(delimited(ws, op_multdiv, ws))(next) {
      Ok((next_input, Some(operador))) => {
        PILA_OPERADORS.lock().unwrap().push(operador.to_owned());
        next_input
      },
      _ => {
        return Ok((next, "termino"));
      }
    };

    next = match factor(next) {
      Ok((next_input, _)) => {
        checar_lista_operadores();
        next_input
      },
      Err(err) => return Err(err)
    };
  };
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_termino() {
    assert_eq!(termino("factor"),                            Ok(("", "termino")));
    assert_eq!(termino("factor * factor * factor / factor"), Ok(("", "termino")));
    assert_eq!(termino("num_entero"),                        Ok(("", "termino")));
    assert_eq!(termino("id"),                                Ok(("", "termino")));
    assert_eq!(termino("id * num_entero * id2 / id3"),       Ok(("", "termino")));
  }
}
