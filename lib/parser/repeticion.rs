use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  sequence::tuple,
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::expresion::*;
use crate::parser::reglas_expresion::exp::*;
use crate::parser::bloque::*;
use crate::parser::dimensiones::*;
use crate::semantica::globales::*;

fn agregar_cuadruplo_a_pila_saltos() {
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  let mut saltos = PILA_SALTOS.lock().unwrap();
  saltos.push((cuadruplos.lista.len()) as i64);
  drop(cuadruplos);
  drop(saltos);
}

fn generar_gotof() {
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  let mut lista_valores = PILA_VALORES.lock().unwrap();

  let mut saltos = PILA_SALTOS.lock().unwrap();
  match lista_valores.pop() {
    Some(var) => {
      match cuadruplos.agregar_cuadruplo_gotof(var) {
        Ok(_) => (),
        Err(err) => { println!("{:?}", err); () }
      };
    },
    _ => ()
  }
  drop(lista_valores);
  saltos.push((cuadruplos.lista.len() - 1) as i64);
  drop(cuadruplos);
  drop(saltos);
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

  cuadruplos.agregar_cuadruplo_goto();
  let tamanio_cuadruplos = cuadruplos.lista.len() - 1;
  cuadruplos.lista[tamanio_cuadruplos].3 = return_dec;

  cuadruplos.modificar_cuadruplo_goto(final_dec as usize);
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

  next = match tuple((ws, tag("("), ws, expresion, ws, tag(")")))(next) {
    Ok((next_input, _)) => {
      generar_gotof();
      next_input
    },
    Err(err) => return Err(err)
  };

  match tuple((necessary_ws, bloque))(next) {
    Ok((next_input, _)) => {
      generar_gotos_final();
      Ok((next_input, "mientras"))
    },
    Err(err) => Err(err)
  }
}

pub fn desde_id(input: &str) -> IResult<&str, &str> {
  tuple((id, many0(tuple((ws, tag("."), ws, id))), con_dim))(input)
  .map(|(next_input, _res)| {
    (next_input, "desde_id")
  })
}

pub fn desde(input: &str) -> IResult<&str, &str> {
  tuple((tag("desde"), necessary_ws, desde_id, ws, tag("="), ws, exp, necessary_ws, tag("hasta"), necessary_ws, exp, necessary_ws, bloque))(input)
  .map(|(next_input, _res)| {
    (next_input, "desde")
  })
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
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_mientras() {
    assert_eq!(mientras("mientras(expresion)"),    Ok(("", "mientras")));
    assert_eq!(mientras("mientras ( expresion )"), Ok(("", "mientras")));
  }

  #[test]
  fn test_desde() {
    assert_eq!(desde("desde id = 10 hasta 20"),         Ok(("", "desde")));
    // assert_eq!(desde("desde id = num_entero hasta num_entero"), Ok(("", "desde")));
    assert_eq!(desde("desde id[id] = 10 hasta 20"),     Ok(("", "desde")));
    assert_eq!(desde("desde id[id][id] = 10 hasta 20"), Ok(("", "desde")));
    assert_eq!(desde("desde id.id[id] = 10 hasta 20"),  Ok(("", "desde")));
    assert_eq!(desde("desde id.id = 15 hasta 25"),      Ok(("", "desde")));
  }

  #[test]
  fn test_repeticion() {
    // assert_eq!(repeticion("mientras(expresion) bloque"), Ok(("", ("mientras", "bloque"))));
    // assert_eq!(repeticion("desde id = num_entero hasta num_entero bloque"), Ok(("", ("desde", "bloque"))));

    assert_eq!(repeticion("mientras(expresion) {}"),    Ok(("", "repeticion")));
    assert_eq!(repeticion("desde id = 10 hasta 20 {}"), Ok(("", "repeticion")));
  }
}
