//! Módulo que se encarga de las funciones

use nom::{
  bytes::complete::tag,
  IResult,
  sequence::{tuple, preceded},
  combinator::opt
};

use crate::scanners::ws::*;
use crate::scanners::tipos::*;
use crate::scanners::id::*;
use crate::parser::bloque::*;
use crate::semantica::globales::*;

/// No terminal para la declaración de un parametro.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```
/// TIPO id
/// ```
///
/// # Ejemplo
///
/// ```
/// match parametro("entero b") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn parametro(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((tipo, ws, id))(input)
  .map(|(next_input, res)| {
    let (tipo, _, id) = res;
    (next_input, (tipo, id))
  })
}

/// Funcion axuliar para agregar los parametros a la  función dentro de la tabla de funciones.
///
/// # Parametros
///
/// * `tipo_param` - Tipo del parámetro
/// * `id_param` - ID del parámetro
///
/// # Ejemplo
///
/// ```
/// agregar_funcion("suma", "void");
/// ```
fn agregar_param(tipo_param: &str, id_param: &str) {
  // Consigue dirección de memoria para el parametro 
  let dir = match conseguir_direccion(tipo_param, "variable", 0, vec![]) {
    Ok(num) => num,
    Err(err) => { println!("{:?}", err); return; }
  };

  // Agrega la variable a la tabla actual de variables
  match VARIABLES.lock().unwrap().agregar_variable(id_param.to_owned(), tipo_param.to_owned(), vec![], dir) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
      return;
    },
  }

  // Agrega la variable a su respectiva función/clase
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();

  if contexto_clase.clone() != "".to_owned() {
    match CLASES.lock().unwrap().agregar_parametro_metodo(contexto_clase.to_string(), contexto_funcion.to_string(), id_param.to_owned(), tipo_param.to_owned(), dir) {
      Ok(_res) => {},
      Err(err) => {
        println!("{:?}", err);
        ()
      },
    };
  } else {
    match FUNCIONES.lock().unwrap().agregar_parametro(contexto_funcion.to_string(), id_param.to_owned(), tipo_param.to_owned(), dir) {
      Ok(_res) => {},
      Err(err) => {
        println!("{:?}", err);
        ()
      },
    }
  }
}

/// Funcion axuliar para agregar una función a la tabla de funciones.
///
/// # Parametros
///
/// * `id_f` - ID de la función
/// * `tipo_func` - Tipo de la función
///
/// # Ejemplo
///
/// ```
/// agregar_funcion("suma", "void");
/// ```
fn agregar_funcion(id_f: &str, tipo_func: &str) {
  let mut funciones = FUNCIONES.lock().unwrap();

  // Marca que estamos en el contexto local
  { *CONTEXTO_FUNCION.lock().unwrap() = ID_PROGRAMA.lock().unwrap().to_string(); }

  // Separa una dirección de memoria para el return de la función en el caso que la función no sea void
  let cuad: i64 = CUADRUPLOS.lock().unwrap().lista.len() as i64;
  let dir = match tipo_func {
    "void" => -8, 
    _ => match conseguir_direccion(tipo_func, "variable", 0, vec![]) {
      Ok(num) => {
        // Checa que no estemos agregando la función para las variables globales
        let id_programa = ID_PROGRAMA.lock().unwrap().to_string();
        if id_programa != "".to_owned() && id_programa != id_f {
          match funciones.tabla.get_mut(&id_programa) {  
            Some(funcion) => {
              funcion.modificar_era(tipo_func.to_owned(), 0);
            },
            None => {
              println!("Funcion global no existente")
            }
          };
        }
        num
      },
      Err(err) => { println!("{:?}", err); return; }
    }
  };

  // Guarda la dirección de la variable global para la función donde se guardara el valor en el return
  unsafe { DIRECCION_CONTEXTO_FUNCION = dir; }

  // Agrega la función a la tabla de variables globales o su respectiva clase
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();

  if contexto_clase.clone() != "".to_owned() {
    match CLASES.lock().unwrap().agregar_metodo(contexto_clase.to_string(), id_f.to_owned(), tipo_func.to_owned(), dir, cuad) {
      Ok(_res) => {},
      Err(err) => {
        println!("{:?}", err);
        ()
      }
    };
  } else {
    match funciones.agregar_funcion(id_f.to_owned(), tipo_func.to_owned(), dir, cuad) {
      Ok(_res) => {},
      Err(err) => {
        println!("{:?}", err);
        ()
      },
    };
  }

  // Marca el contexto actual como la función que se acaba de crear
  *CONTEXTO_FUNCION.lock().unwrap() = id_f.to_owned();
}

/// Funcion axuliar para parsear los diferentes parametros.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```
/// match parametros_varios("entero b, entero a)") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn parametros_varios(input: &str) -> IResult<&str, &str> {
  let mut next : &str;

  // Lee el primer parametro
  next = match parametro(input) {
    Ok((next_input, (tipo_param, id_param))) => {
      agregar_param(tipo_param, id_param);
      next_input
    },
    // Se sale de la función en caso de que no exista
    Err(_) => return Ok((input, "parametros_varios"))
  };

  // Itera sobre los siguientes parametros
  loop {
    // Checa que haya una coma, indicando otro parametro
    next = match opt(tuple((ws, tag(","), ws)))(next) {
      Ok((next_input, Some(_))) => next_input,
      _ => {
        break;
      }
    };

    // Lee un parametro
    next = match parametro(next) {
      Ok((next_input, (tipo_param, id_param))) => {
        agregar_param(tipo_param, id_param);
        next_input
      },
      Err(err) => return Err(err)
    };
  };

  Ok((next, "parametros_varios"))
}

/// No terminal para la declaración de funciones.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```
/// TIPO funcion id (PARAMS) BLOQUE_FUNCION
/// ```
///
/// # Ejemplo
///
/// ```
/// match funcion("void funcion suma(entero a, entero b) { regresa a + b; }") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn funcion(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;
  let tipo_func : &str;

  // Consigue el tipo de retorno de la funcion
  next = match preceded(ws, tipo_retorno)(next) {
    Ok((next_input, tipo_f)) => {
      tipo_func = tipo_f;
      next_input
    },
    Err(err) => return Err(err)
  };

  // Consigue el ID de la funcion
  next = match preceded(tuple((ws, tag("funcion"), necessary_ws)), id)(next) {
    Ok((next_input, id_f)) => {
      agregar_funcion(id_f, tipo_func);
      next_input
    },
    Err(err) => return Err(err)
  };

  // Lee los parametros de la función
  match tuple((
    ws,
    tag("("), ws, parametros_varios, ws, tag(")"), ws,
    bloque_funcion, ws
  ))(next) {
    Ok((next_input, _)) => {
      unsafe {
        // Checa que las funciones no void tengan return
        if tipo_func != "void" && RETURN_EXISTENTE == false {
          println!("Funcion le falta return");
        }
        
        // Checa que las funciones void no tengan return
        if tipo_func == "void" && RETURN_EXISTENTE == true {
          println!("Funcion void no debería tener return");
        }

        // Resetea variables globales
        DIRECCION_CONTEXTO_FUNCION = -10;
        RETURN_EXISTENTE = false;
      }
      match CUADRUPLOS.lock().unwrap().agregar_cuadruplo_endfunc() {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        }
      };
      // Marca que no hay contexto de función actual
      // En caso de que se esté dentro de una clase
      *CONTEXTO_FUNCION.lock().unwrap() = "".to_owned();
      Ok((next_input, "funcion"))
    },
    Err(err) => Err(err)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parametro() {
    assert_eq!(parametro("char id"),   Ok(("", ("char", "id"))));
    assert_eq!(parametro("entero id"), Ok(("", ("entero", "id"))));
  }

  #[test]
  fn test_funcion() {
    assert_eq!(funcion("void funcion func () { regresa expresion ; }"), Ok(("", "funcion")));
    assert_eq!(funcion("void funcion func (entero var) { num = 10; regresa expresion ; }"), Ok(("", "funcion")));
  }
}
