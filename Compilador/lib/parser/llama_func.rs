//! Módulo que se encarga de las llamadas a función.

use nom::{
  bytes::complete::tag,
  IResult,
  sequence::{tuple, preceded},
};

use crate::scanners::ws::*;
use crate::scanners::id::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::tabla_variables::*;
use crate::semantica::globales::*;

/// Función auxiliar para generar cuadruplo de era.  
/// Regresa la lista de parametros de una función.  
/// 
/// # Parametros
///
/// * `id_func` - Función a la cual se le quiere crear su era
///
/// # Ejemplo
///
/// ```ignore
/// generar_cuadruplo_era("func");
/// ```
pub fn generar_cuadruplo_era(id_func: &str) -> Vec<TipoVar> {
  // Busca la función, genera el cuadruplo y regresa su parámetros.
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap().to_string();
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  if contexto_clase != "".to_owned() {
    match CLASES.lock().unwrap().buscar_metodo(contexto_clase, id_func.to_owned()) {
      Ok((_, _, func)) => {
        match cuadruplos.agregar_cuadruplo_era(func.num_cuadruplo) {
          Ok(_) => (),
          Err(err) => {
            println!("{:?}", err);
          }
        };
        return func.parametros.clone();
      },
      Err(err) => {
        println!("{:?}", err);
        return vec![];
      }
    };
  } else {
    match FUNCIONES.lock().unwrap().buscar_funcion(id_func.to_owned()) {
      Ok((_, func)) => {
        match cuadruplos.agregar_cuadruplo_era(func.num_cuadruplo) {
          Ok(_) => (),
          Err(err) => {
            println!("{:?}", err);
          }
        };
        return func.parametros.clone();
      },
      Err(err) => {
        println!("{:?}", err);
        return vec![];
      }
    };
  }
}

/// Función auxiliar para generar cuadruplo de gosub.  
///
/// # Parametros
///
/// * `id_func` - Función a la cual se quiere ir
///
/// # Ejemplo
///
/// ```ignore
/// generar_cuadruplo_gosub("func");
/// ```
pub fn generar_cuadruplo_gosub(id_func: &str) {
  // Busca la función y genera el cuadruplo
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap().to_string();
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  if contexto_clase != "".to_owned() {
    match CLASES.lock().unwrap().buscar_metodo(contexto_clase, id_func.to_owned()) {
      Ok((_, _, func)) => {
        match cuadruplos.agregar_cuadruplo_gosub(func.num_cuadruplo) {
          Ok(_) => (),
          Err(err) => {
            println!("{:?}", err);
          }
        };
      },
      Err(err) => {
        println!("{:?}", err);
      }
    };
  } else {
    match FUNCIONES.lock().unwrap().buscar_funcion(id_func.to_owned()) {
      Ok((_, func)) => {
        match cuadruplos.agregar_cuadruplo_gosub(func.num_cuadruplo) {
          Ok(_) => (),
          Err(err) => {
            println!("{:?}", err);
          }
        };
      },
      Err(err) => {
        println!("{:?}", err);
      }
    };
  }
}

/// Función auxiliar para generar cuadruplo de param.  
///
/// # Parametros
///
/// * `param` - Parametro al cual se le quiere asignar un valor
///
/// # Ejemplo
///
/// ```ignore
/// generar_cuadruplo_param(vec![Tipo {
///   nombre: "arreglo".to_owned(),
///   tipo: "entero".to_owned(),
///   dimensiones: vec![],
///   direccion: 10,
/// }], 4);
/// ```
pub fn generar_cuadruplo_param(param: TipoVar) {
  // Saca el último valor de la pila de valores
  let variable = match PILA_VALORES.lock().unwrap().pop() {
    Some(var) => var,
    None => return
  };

  if param.tipo != variable.tipo {
    return;
  }

  // Genera cuaruplo
  match CUADRUPLOS.lock().unwrap().agregar_cuadruplo_param(variable.clone(), param.clone()) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    }
  };
}

/// No terminal de llama_func.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// uno ( EXP ) ;
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match desde("desde id = valor hasta valor {}") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn llama_func(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;
  let id_func: &str;
  // let _id_attr: &str;

  // Consigue id de función
  next = match id(next) {
    Ok((next_input, id_f)) => {
      id_func = id_f;
      next_input
    },
    Err(err) => return Err(err)
  };

  // next = match preceded(tuple((ws, tag("."), ws)), id)(next) {
  //   Ok((next_input, id_a)) => {
  //     _id_attr = id_a;
  //     next_input
  //   },
  //   Err(_) => next
  // };

  // Lee inicio de parametros
  next = match tag("(")(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  // Agrega fondo falso a la pila de operadores
  { PILA_OPERADORS.lock().unwrap().push("(".to_owned()); }

  // Genera cuadruplo de era
  let params = generar_cuadruplo_era(id_func);
  let mut pos: usize = 0;
  let mut continuar = true;

  // Lee el primer argumento
  next = match preceded(ws, exp)(next) {
    Ok((next_input, _)) => {
      // Checa que la función reciba parametros
      if pos >= params.len() {
        println!("Se excedió la cantidad de parametros dentro de la llamada a función");
      } else {
        // Genera cuadruplo param
        generar_cuadruplo_param(params[pos].clone());
        pos += 1;
      }
      next_input
    },
    Err(_) => {
      continuar = false;
      next
    }
  };

  // Checa si debe seguir leyendo argumentos
  if continuar {
    // Itera sobre los parametros
    loop {
      // Checa que haya una coma, indicando que hay otro argumento
      next = match tuple((ws, tag(",")))(next) {
        Ok((next_input, _)) => next_input,
        Err(_err) => break
      };
  
      // Lee argumento
      next = match preceded(ws, exp)(next) {
        Ok((next_input, _)) => {
          if pos >= params.len() {
            println!("Se excedió la cantidad de parametros dentro de la llamada a función");
          } else {
            generar_cuadruplo_param(params[pos].clone());
            pos += 1;
          }
          next_input
        },
        Err(err) => return Err(err)
      };
    }
  }

  // Lee fin de llamada a función
  match tuple((ws, tag(")"), ws, tag(";")))(next) {
    Ok((next_input, _)) => {
      // Elimina fondo falso y genera cuadruplo de gosub 
      { PILA_OPERADORS.lock().unwrap().pop(); }
      generar_cuadruplo_gosub(id_func);
      Ok((next_input, "llama_func"))
    },
    Err(err) => Err(err)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_llama_func() {
    assert_eq!(llama_func("id();"), Ok(("", "llama_func")));
  }
}
