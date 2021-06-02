//! Módulo que se encarga de las funciones especiales (_leer_ y _escribir_).

use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::tuple
};

use crate::scanners::ws::*;
use crate::scanners::constantes::*;
use crate::parser::reglas_expresion::exp::*;
use crate::parser::reglas_expresion::valor::*;
use crate::semantica::globales::*;

/// Función auxiliar para generar cuadruplo de escritura.  
///
/// # Ejemplo
///
/// ```ignore
/// generar_cuadruplo_lectura();
/// ```
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

/// Función auxiliar para generar cuadruplo de escritura.  
///
/// # Ejemplo
///
/// ```ignore
/// generar_cuadruplo_escritura();
/// ```
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

/// No terminal leer. Sirve para pedir valores al usuario.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// lee ( id ) ;
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match escribir("leer(num);") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn leer(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;

  // Lee inicio de función
  next = match tuple((tag("lee"), ws, tag("("), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  // Lee valor a pedir
  next = match valor(next) {
    Ok((next_input, _)) => {
      generar_cuadruplo_lectura();
      next_input
    },
    Err(err) => return Err(err)
  };

  // Itera sobre los siguientes valores
  loop {
    // Checa que haya una coma, indicando otro valor
    next = match tuple((ws, tag(","), ws))(next) {
      Ok((next_input, _)) => next_input,
      _ => break
    };

    // Lee valor a pedir
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

/// Función auxiliar para agregar un texto a la tablas de constantes.  
///
/// # Parametros
///
/// * `valor` - Texto que se quiere agregar a la tabla de constantes
///
/// # Ejemplo
///
/// ```ignore
/// generar_cuadruplo_offset("texto");
/// ```
fn agregar_texto_a_tabla(valor: &str) {
  let mut pila_valores = PILA_VALORES.lock().unwrap();
  pila_valores.push(CONSTANTES.lock().unwrap().agregar_constante(valor.to_owned(), "texto".to_owned()));
  drop(pila_valores);
  generar_cuadruplo_escritura();
}

/// No terminal escribir. Sirve para imprimir valores a consola.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// escribe ( texto | EXP ) ;
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match escribir("escribe(10);") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn escribir(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;

  // Lee el inicio de función
  next = match tuple((tag("escribe"), ws, tag("("), ws))(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  // Lee constantes de texto
  next = match texto(next) {
    Ok((next_i, texto_const)) => {
      agregar_texto_a_tabla(texto_const);
      next_i
    },
    Err(_) => {
      // En caso de no encontrar texto, lee una expresión
      match exp(next) {
        Ok((next_input, _)) => {
          generar_cuadruplo_escritura();
          next_input
        },
        Err(err) => return Err(err)
      }
    }
  };

  // Itera sobre los siguientes valores
  loop {
    // Checa que haya una coma, indicando otro valor
    next = match tuple((ws, tag(","), ws))(next) {
      Ok((next_input, _)) => next_input,
      _ => break
    };

    // Lee constantes de texto
    next = match texto(next) {
      Ok((next_i, texto_const)) => {
        agregar_texto_a_tabla(texto_const);
        next_i
      },
      Err(_) => {
        // En caso de no encontrar texto, lee una expresión
        match exp(next) {
          Ok((next_input, _)) => {
            generar_cuadruplo_escritura();
            next_input
          },
          Err(err) => return Err(err)
        }
      }
    };
  };

  // Lee el final de la función
  match tuple((ws, tag(")"), tag(";")))(next) {
    Ok((next_input, _)) => Ok((next_input, "escribir")),
    Err(err) => Err(err)
  }
}

/// No terminal func_esp. Sirve para leer ambas funciones especiales (_leer_ y _escribir_).  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// ESCRIBIR | LEER
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match funcion_esp("escribe(10);") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
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
