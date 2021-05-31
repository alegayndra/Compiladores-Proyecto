use nom::{
  bytes::complete::tag,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::exp_logica::*;
use crate::parser::bloque::*;
use crate::semantica::globales::*;

fn generar_goto_sino() {
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  let mut saltos = PILA_SALTOS.lock().unwrap();
  match saltos.pop() {
    Some(valor) => {
      match cuadruplos.modificar_cuadruplo_goto(valor as usize) {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      };
    },
    _ => ()
  };

  saltos.push((cuadruplos.lista.len()) as i64);

  match cuadruplos.agregar_cuadruplo_goto() {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    },
  };
  
  drop(cuadruplos);
  drop(saltos);
}

fn sino(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;

  next = match tuple((ws, tag("sino")))(next) {
    Ok((next_input, _)) => {
      generar_goto_sino();
      next_input
    },
    Err(_) => return Ok((input, "sino"))
  };

  match tuple((ws, bloque))(next) {
    Ok((next_input, _)) => Ok((next_input, "sino")),
    Err(err) => Err(err)
  }
}

fn generar_gotof() {
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  let mut lista_valores = PILA_VALORES.lock().unwrap();

  let mut saltos = PILA_SALTOS.lock().unwrap();
  match lista_valores.pop() {
    Some(var) => {
      match cuadruplos.agregar_cuadruplo_gotof(var) {
        Ok(_res) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      };
    },
    _ => ()
  }
  drop(lista_valores);
  saltos.push((cuadruplos.lista.len() - 1) as i64);
  drop(cuadruplos);
  drop(saltos);
}

fn actualizar_gotof() {
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  let mut saltos = PILA_SALTOS.lock().unwrap();
  match saltos.pop() {
    Some(valor) => {
      match cuadruplos.modificar_cuadruplo_goto(valor as usize) {
        Ok(_res) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      };
    },
    _ => { println!("Pila de saltos vacía en PRINCIPAL"); () }
  }
  
  drop(cuadruplos);
  drop(saltos);
}

pub fn decision(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;

  next = match tuple((tag("si"), ws, tag("("), ws, exp_logica, ws, tag(")")))(next) {
    Ok((next_input, _)) => {
      generar_gotof();
      next_input
    },
    Err(err) => return Err(err)
  };

  match tuple((ws, bloque, sino))(next) {
    Ok((next_input, _)) => {
      actualizar_gotof();
      Ok((next_input, "decision"))
    },
    Err(err) => Err(err)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_decision() {
    assert_eq!(decision("si ( expresion ) {}"),        Ok(("", "decision")));
    assert_eq!(decision("si ( expresion ) {} sino {}"), Ok(("", "decision")));
  }
}
