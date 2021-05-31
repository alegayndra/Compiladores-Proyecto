use nom::{
  IResult,
  sequence::delimited
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::expresion::*;
use crate::semantica::globales::*;

fn checar_lista_operadores() {
  let mut lista_operadores = PILA_OPERADORS.lock().unwrap();
  match lista_operadores.pop() {
    Some(op) => {
      match op_logica(&op) {
        Ok(_) => {
          let mut pila_val = PILA_VALORES.lock().unwrap();

          let der = match pila_val.pop() {
            Some(val) => val,
            _ => return
          };
          let izq = match pila_val.pop() {
            Some(val) => val,
            _ => {
              println!("Stack de valores vacío en EXP_LOGICA");
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
      };
    },
    _ => {}
  };

  drop(lista_operadores);
}

pub fn exp_logica(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  next = match expresion(next) {
    Ok((next_input, _)) => {
      checar_lista_operadores();
      next_input
    },
    Err(err) => return Err(err)
  };

  loop {
    next = match delimited(ws, op_logica, ws)(next) {
      Ok((next_input, operador)) => {
        PILA_OPERADORS.lock().unwrap().push(operador.to_owned());
        next_input
      },
      _ => return Ok((next, "exp_logica"))
    };

    next = match expresion(next) {
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
  fn test_exp_logica() {
    assert_eq!(exp_logica("id"),                                  Ok(("", "exp_logica")));
    assert_eq!(exp_logica("10"),                                  Ok(("", "exp_logica")));
    assert_eq!(exp_logica("id & num_entero"),                     Ok(("", "exp_logica")));
    assert_eq!(exp_logica("id | num_entero"),                     Ok(("", "exp_logica")));
    assert_eq!(exp_logica("id | id > 2 * ( - num_entero + id )"), Ok(("", "exp_logica")));
  }
}
