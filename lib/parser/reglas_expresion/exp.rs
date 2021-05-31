use nom::{
  IResult,
  sequence::delimited,
  combinator::opt
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::termino::*;
use crate::semantica::globales::*;

fn checar_lista_operadores() {
  let mut lista_operadores = PILA_OPERADORS.lock().unwrap();
  match lista_operadores.pop() {
    Some(op) => {
      match op_sumsub(&op) {
        Ok(_) => {
          let mut pila_val = PILA_VALORES.lock().unwrap();
          let der = match pila_val.pop() {
            Some(val) => val,
            _ => return
          };
          let izq = match pila_val.pop() {
            Some(val) => val,
            _ => {
              println!("Stack de valores vacío en EXP");
              return;
            }
          };

          drop(pila_val);

          match CUADRUPLOS.lock().unwrap().agregar_cuadruplo(&op, izq, der) {
            Ok(_) => (),
            Err(err) => {
              println!("{:?}", err);
            },
          };
        },
        Err(_) => { lista_operadores.push(op); }
      }
    },
    _ => ()
  }
  drop(lista_operadores);
}

pub fn exp(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  next = match termino(next) {
    Ok((next_input, _)) => {
      checar_lista_operadores();
      next_input
    },
    Err(err) => return Err(err)
  };

  loop {
    next = match opt(delimited(ws, op_sumsub, ws))(next) {
      Ok((next_input, Some(operador))) => {
        let mut lista_operadores = PILA_OPERADORS.lock().unwrap();
        lista_operadores.push(operador.to_owned());
        drop(lista_operadores);

        next_input
      },
      _ => {
        return Ok((next, "exp"));
      }
    };

    next = match termino(next) {
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

  #[test]
  fn test_exp() {
    assert_eq!(exp("abr  "), Ok(("  ", "exp")));
    assert_eq!(exp("num_entero"), Ok(("", "exp")));
    assert_eq!(exp("id"), Ok(("", "exp")));
    assert_eq!(exp("id  "), Ok(("  ", "exp")));
    assert_eq!(exp("10  "), Ok(("  ", "exp")));
    assert_eq!(exp("id * num_entero"), Ok(("", "exp")));
    assert_eq!(exp("id + num_entero"), Ok(("", "exp")));
    assert_eq!(exp("id + num_entero * id2 - num_entero - termino"), Ok(("", "exp")));
  }
}