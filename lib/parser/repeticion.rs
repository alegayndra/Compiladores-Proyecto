use nom::{
  branch::alt,
  bytes::complete::tag,
  multi::many0,
  IResult,
  combinator::opt,
  sequence::{tuple, delimited},
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::exp_logica::*;
use crate::parser::reglas_expresion::exp::*;
use crate::parser::bloque::*;
use crate::parser::dimensiones::*;
use crate::semantica::tabla_variables::*;
use crate::semantica::globales::*;

fn agregar_cuadruplo_a_pila_saltos() {
  let cuadruplos = CUADRUPLOS.lock().unwrap();
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

  match cuadruplos.agregar_cuadruplo_goto() {
    Ok(res) => { println!("{:?}", res); () },
    Err(err) => { println!("{:?}", err); () }
  };
  let tamanio_cuadruplos = cuadruplos.lista.len() - 1;
  cuadruplos.lista[tamanio_cuadruplos].3 = return_dec;

  match cuadruplos.modificar_cuadruplo_goto(final_dec as usize) {
    Ok(res) => { println!("{:?}", res); () },
    Err(err) => { println!("{:?}", err); () }
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

fn buscar_variable(id_valor: &str) -> TipoVar {
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();

  let tabla_variables = VARIABLES.lock().unwrap();
  let tabla_funciones = FUNCIONES.lock().unwrap();
  let tabla_clases = CLASES.lock().unwrap();

  let variable_invalida = TipoVar {
    nombre: "".to_owned(),
    tipo: "".to_owned(),
    dimensiones: vec!["-1".to_owned()],
    direccion: -3
  };

  match tabla_variables.buscar_variable(id_valor.to_owned()) {
    Ok((res, variable)) => { println!("{:?}", res); return variable;},
    Err(err) => ()
  };

  if contexto_clase.clone() != "".to_owned() {
    if contexto_funcion.clone() != "".to_owned() {
      match tabla_clases.buscar_variable_metodo(contexto_clase.clone(), contexto_funcion.clone(), id_valor.to_owned()) {
        Ok((res, res2, res3, variable)) => { println!("{:?} {:?} {:?}", res, res2, res3); return variable; },
        Err(err) => { println!("{:?}", err); (); }
      };
    } else {
      match tabla_clases.buscar_atributo(contexto_clase.clone(), id_valor.to_owned()) {
        Ok((res, res2, variable)) => { println!("{:?} {:?}", res, res2); return variable; },
        Err(err) => { println!("{:?}", err); () }
      };
    }
  } else {
    match tabla_funciones.buscar_variable(contexto_funcion.clone(), id_valor.to_owned()) {
      Ok((res, res2, variable)) => { println!("{:?} {:?}", res, res2); return variable; },
      Err(err) => { println!("{:?}", err); () }
    };
  }

  drop(contexto_funcion);
  drop(contexto_clase);

  drop(tabla_variables);
  drop(tabla_funciones);
  drop(tabla_clases);

  variable_invalida
}

pub fn desde_id(input: &str) -> IResult<&str, TipoVar> {
  // tuple((id, many0(tuple((ws, tag("."), ws, id))), con_dim))(input)
  match tuple((id, opt(tuple((ws, tag("."), ws, id))), con_dim))(input) {
    Ok((next_input, (id, _, _))) => {
      println!("id for loop {:?}", id);
      let var = buscar_variable(id);
      Ok((next_input, var))
    },
    Err(err) => Err(err)
  }
}

pub fn desde(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;
  let variable;
  next = match delimited(tuple((tag("desde"), necessary_ws)), desde_id, tuple((ws, tag("="), ws, exp)))(next) {
    Ok((next_input, var)) => {
      variable = var;
      agregar_cuadruplo_a_pila_saltos();
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match tuple((necessary_ws, tag("hasta"), necessary_ws, exp))(next) {
    Ok((next_input, _)) => {
      generar_gotof();
      next_input
    },
    Err(err) => return Err(err)
  };

  match tuple((necessary_ws, bloque))(next) {
    Ok((next_input, _)) => {
      let mut cuadruplos = CUADRUPLOS.lock().unwrap();
      match cuadruplos.agregar_cuadruplo_for(variable) {
        Ok(res) => {  println!("{:?}", res); () },
        Err(err) => {  println!("{:?}", err); () }
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
