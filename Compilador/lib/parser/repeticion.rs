//! Módulo que se encarga del analisis de las repeticiones.
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

/// Función auxiliar que agrega el cuadruplo siguiente a la pila de saltos.
///
/// # Ejemplo
///
/// ```
/// agregar_cuadruplo_a_pila_saltos();
/// ```
fn agregar_cuadruplo_a_pila_saltos() {
  PILA_SALTOS.lock().unwrap().push((CUADRUPLOS.lock().unwrap().lista.len()) as i64);
}

/// Función auxiliar que genera el gotof del ciclo _mientras_.
///
/// # Ejemplo
///
/// ```
/// generar_gotof_mientras();
/// ```
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

/// Función auxiliar que genera el gotof del ciclo _desde_.  
/// Regresa la dirección de memoria de la variable del _desde_.
///
/// # Ejemplo
///
/// ```
/// generar_gotof_desde();
/// ```
fn generar_gotof_desde() -> i64 {
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();

  let dir = match cuadruplos.agregar_cuadruplo_gotof_desde() {
    Ok((_, dir_temp)) => dir_temp,
    Err(err) => {
      println!("{:?}", err);
      -7
    }
  };

  let mut saltos = PILA_SALTOS.lock().unwrap();
  saltos.push((cuadruplos.lista.len() - 1) as i64);
  drop(cuadruplos);
  drop(saltos);
  dir
}

/// Función auxiliar que genera el goto de regreso a la condicional de al repetición y actualiza el goto para salirse del ciclo.
///
/// # Ejemplo
///
/// ```
/// generar_gotos_final();
/// ```
fn generar_gotos_final() {
  // Consigue los dos últimos valores dentro de la pila de saltos
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

  // Agrega el cuadruplo del goto para regresarse a la condicional
  match cuadruplos.agregar_cuadruplo_goto() {
    Ok(_res) => (),
    Err(err) => {
      println!("{:?}", err);
    },
  };

  let tamanio_cuadruplos = cuadruplos.lista.len() - 1;
  cuadruplos.lista[tamanio_cuadruplos].3 = return_dec;

  // Modifica el gotof para saltarse el código dentro del bloque del ciclo
  match cuadruplos.modificar_cuadruplo_goto(final_dec as usize) {
    Ok(_res) => (),
    Err(err) => {
      println!("{:?}", err);
    },
  };
}

/// No terminal de mientras.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```
/// mientras (EXP_LOGICA) BLOQUE;
/// ```
///
/// # Ejemplo
///
/// ```
/// match desde("desde id = valor hasta valor {}") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn mientras(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;

  next = match tag("mientras")(next) {
    Ok((next_input, _)) => {
      agregar_cuadruplo_a_pila_saltos();
      next_input
    },
    Err(err) => return Err(err)
  };

  // Condicional
  next = match tuple((ws, tag("("), ws, exp_logica, ws, tag(")")))(next) {
    Ok((next_input, _)) => {
      generar_gotof_mientras();
      next_input
    },
    Err(err) => return Err(err)
  };

  // Bloque de código del ciclo
  match tuple((ws, bloque))(next) {
    Ok((next_input, _)) => {
      generar_gotos_final();
      Ok((next_input, "mientras"))
    },
    Err(err) => Err(err)
  }
}

/// No terminal de repetición.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// # Gramática
///
/// ```
/// desde id DIMENSIONES = EXP hasta EXP BLOQUE;
/// ```
///
/// ```
/// match desde("desde id = valor hasta valor {}") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn desde(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;
  let variable: i64;

  // Asignación inicial del _desde_
  next = match preceded(tuple((tag("desde"), necessary_ws)), asignacion_interna)(next) {
    Ok((next_input, _)) => {
      agregar_cuadruplo_a_pila_saltos();
      next_input
    },
    Err(err) => return Err(err)
  };

  // Valor final del ciclo
  next = match tuple((necessary_ws, tag("hasta"), necessary_ws, exp))(next) {
    Ok((next_input, _)) => {
      variable = generar_gotof_desde();
      next_input
    },
    Err(err) => return Err(err)
  };

  // Bloque de código del ciclo
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

/// Función auxiliar para agrupar las diferentes repeticiones.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match repeticion("mientras(1){}") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
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
    let wii = "wii";
    let id = "id";
    let parte = "parte";
    let mut tabla_variables = VARIABLES.lock().unwrap();
    tabla_variables.agregar_variable(wii.to_owned(), "entero".to_owned(), vec![], 200);
    tabla_variables.agregar_variable(id.to_owned(), "entero".to_owned(), vec![5], 300);
    tabla_variables.agregar_variable(parte.to_owned(), "entero".to_owned(), vec![5, 10], 400);
    drop(tabla_variables);
    assert_eq!(desde("desde wiii = 10 hasta 20 {}"),         Ok(("", "desde")));
    assert_eq!(desde("desde id[id] = 10 hasta 20 {}"),     Ok(("", "desde")));
    assert_eq!(desde("desde parte[id][id] = 10 hasta 20 {}"), Ok(("", "desde")));
    // assert_eq!(desde("desde id.id[id] = 10 hasta 20 {}"),  Ok(("", "desde")));
    // assert_eq!(desde("desde id.id = 15 hasta 25 {}"),      Ok(("", "desde")));
  }

  #[test]
  fn test_repeticion() {
    assert_eq!(repeticion("mientras(expresion) {}"),    Ok(("", "repeticion")));
    assert_eq!(repeticion("desde wii = 10 hasta 20 {}"), Ok(("", "repeticion")));
  }
}
