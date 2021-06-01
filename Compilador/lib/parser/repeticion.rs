use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::{tuple, preceded},
};

use crate::scanners::ws::*;
use crate::parser::reglas_expresion::exp_logica::*;
use crate::parser::reglas_expresion::exp::*;
use crate::parser::bloque::*;
use crate::parser::asignacion::*;
use crate::semantica::globales::*;

fn agregar_cuadruplo_a_pila_saltos() {
  PILA_SALTOS.lock().unwrap().push((CUADRUPLOS.lock().unwrap().lista.len()) as i64);
}

fn generar_gotof_mientras() {
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  let mut lista_valores = PILA_VALORES.lock().unwrap();
  let mut saltos = PILA_SALTOS.lock().unwrap();

  match lista_valores.pop() {
    Some(var) => {
      match cuadruplos.agregar_cuadruplo_gotof(var) {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      };
    },
    _ => ()
  };

  drop(lista_valores);
  saltos.push((cuadruplos.lista.len() - 1) as i64);
  drop(cuadruplos);
  drop(saltos);
}

fn generar_gotof_desde() -> i64{
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  // let mut lista_valores = PILA_VALORES.lock().unwrap();

  // match lista_valores.pop() {
  //   Some(var) => {
  //     drop(lista_valores);
  //     match cuadruplos.agregar_cuadruplo("<=", variable.clone(), var.clone()) {
  //       Ok(_) => (),
  //       Err(err) => {
  //         println!("{:?}", err);
  //       }
  //     };      
  //   },
  //   _ => ()
  // };

  let dir = match cuadruplos.agregar_cuadruplo_gotof_desde() {
    Ok((_, dir_temp)) => dir_temp,
    Err(err) => {
      println!("{:?}", err);
      -7
    }
  };

  // match lista_valores.pop() {
  //   Some(var) => {
  //     match cuadruplos.agregar_cuadruplo_gotof(var) {
  //       Ok(_) => (),
  //       Err(err) => {
  //         println!("{:?}", err);
  //       }
  //     };
  //   },
  //   _ => ()
  // };

  // drop(lista_valores);
  let mut saltos = PILA_SALTOS.lock().unwrap();
  saltos.push((cuadruplos.lista.len() - 1) as i64);
  drop(cuadruplos);
  drop(saltos);
  dir
}

fn generar_gotos_final() {
  let mut saltos = PILA_SALTOS.lock().unwrap();
  let final_dec = match saltos.pop() {
    Some(val) => val,
    None => return
  };

  let return_dec = match saltos.pop() {
    Some(val) => val,
    None => return
  };

  let mut cuadruplos = CUADRUPLOS.lock().unwrap();

  match cuadruplos.agregar_cuadruplo_goto() {
    Ok(_res) => (),
    Err(err) => {
      println!("{:?}", err);
    },
  };

  let tamanio_cuadruplos = cuadruplos.lista.len() - 1;
  cuadruplos.lista[tamanio_cuadruplos].3 = return_dec;

  match cuadruplos.modificar_cuadruplo_goto(final_dec as usize) {
    Ok(_res) => (),
    Err(err) => {
      println!("{:?}", err);
    },
  };
}

pub fn mientras(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;
  next = match tag("mientras")(next) {
    Ok((next_input, _)) => {
      agregar_cuadruplo_a_pila_saltos();
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match tuple((ws, tag("("), ws, exp_logica, ws, tag(")")))(next) {
    Ok((next_input, _)) => {
      generar_gotof_mientras();
      next_input
    },
    Err(err) => return Err(err)
  };

  match tuple((ws, bloque))(next) {
    Ok((next_input, _)) => {
      generar_gotos_final();
      Ok((next_input, "mientras"))
    },
    Err(err) => Err(err)
  }
}

pub fn desde(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;
  let variable: i64;
  next = match preceded(tuple((tag("desde"), necessary_ws)), asignacion_interna)(next) {
    Ok((next_input, _)) => {
      agregar_cuadruplo_a_pila_saltos();
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match tuple((necessary_ws, tag("hasta"), necessary_ws, exp))(next) {
    Ok((next_input, _)) => {
      variable = generar_gotof_desde();
      next_input
    },
    Err(err) => return Err(err)
  };

  match tuple((necessary_ws, bloque))(next) {
    Ok((next_input, _)) => {
      let mut cuadruplos = CUADRUPLOS.lock().unwrap();
      match cuadruplos.agregar_cuadruplo_for(variable) {
        Ok(_res) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      };
      drop(cuadruplos);
      generar_gotos_final();
      Ok((next_input, "desde"))
    },
    Err(err) => Err(err)
  }
}

pub fn repeticion(input: &str) -> IResult<&str, &str> {
  alt((mientras, desde))(input)
  .map(|(next_input, _res)| {
    (next_input, "repeticion")
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mientras() {
    assert_eq!(mientras("mientras(expresion) {}"),    Ok(("", "mientras")));
    assert_eq!(mientras("mientras ( expresion ) {}"), Ok(("", "mientras")));
  }

  #[test]
  fn test_desde() {
    assert_eq!(desde("desde id = 10 hasta 20 {}"),         Ok(("", "desde")));
    assert_eq!(desde("desde id[id] = 10 hasta 20 {}"),     Ok(("", "desde")));
    assert_eq!(desde("desde id[id][id] = 10 hasta 20 {}"), Ok(("", "desde")));
    assert_eq!(desde("desde id.id[id] = 10 hasta 20 {}"),  Ok(("", "desde")));
    assert_eq!(desde("desde id.id = 15 hasta 25 {}"),      Ok(("", "desde")));
  }

  #[test]
  fn test_repeticion() {
    assert_eq!(repeticion("mientras(expresion) {}"),    Ok(("", "repeticion")));
    assert_eq!(repeticion("desde id = 10 hasta 20 {}"), Ok(("", "repeticion")));
  }
}
