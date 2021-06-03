//! Módulo que se encarga del inicio de la decisión.

use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::{tuple, delimited},
};

use crate::scanners::constantes::*;
use crate::scanners::ws::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::globales::*;
use crate::semantica::tabla_variables::*;

/// Función auxiliar para buscar una variable dentro de las tablas de variables.  
/// Regresa la variable la información que se buscó.  
///
/// # Parametros
///
/// * `id_valor` - ID de variable que se quiere buscar
///
/// # Ejemplo
///
/// ```ignore
/// buscar_variable("nombre");
/// ```
pub fn buscar_variable(id_valor: &str) -> TipoVar {
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();

  let tabla_variables = VARIABLES.lock().unwrap();
  let tabla_funciones = FUNCIONES.lock().unwrap();
  let tabla_clases = CLASES.lock().unwrap();

  // Busca la variable dentro de la lista de variables
  match tabla_variables.buscar_variable(id_valor.to_owned()) {
    Ok((_, var)) => return var,
    Err(_) => ()
  };

  // Busca la variable dentro de la lista de clases o funciones, dependiendo del contexto
  if contexto_clase.clone() != "".to_owned() {
    if contexto_funcion.clone() != "".to_owned() {
      match tabla_clases.buscar_variable_metodo(contexto_clase.clone(), contexto_funcion.clone(), id_valor.to_owned()) {
        Ok((_, _, _, var)) => return var,
        Err(err) => {
          // println!("{:?}", err);
        }
      };
    } else {
      match tabla_clases.buscar_atributo(contexto_clase.clone(), id_valor.to_owned()) {
        Ok((_, _, var)) => return var,
        Err(err) => {
          // println!("{:?}", err);
        }
      };
    }
  } else {
    match tabla_funciones.buscar_variable(contexto_funcion.clone(), id_valor.to_owned()) {
      Ok((_, _, var)) => return var,
      Err(err) => {
        // println!("{:?}", err);
      }
    };
  }

  match tabla_funciones.buscar_variable(ID_PROGRAMA.lock().unwrap().clone(), id_valor.to_owned()) {
    Ok((_, _, var)) => return var,
    Err(err) => {
      println!("{:?}", err);
    }
  };

  drop(contexto_funcion);
  drop(contexto_clase);

  drop(tabla_variables);
  drop(tabla_funciones);
  drop(tabla_clases);

  // Regresa una variable inválida
  TipoVar {
    nombre: "".to_owned(),
    tipo: "".to_owned(),
    dimensiones: vec![],
    direccion: -10
  }
}

/// Función auxiliar checar un corchete izquierdo al principio de una dimensión, esto para evitar una ambigüedad de tipos al llamar el _scanner_ de _nom_.  
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match corchete("[") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn corchete(input: &str) -> IResult<&str, &str> {
  tag("[")(input)
}

/// Función auxiliar para leer dimensiones vacías en las declaraciones/indexación de variables.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match ws_vec("id") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn ws_vec(input: &str) -> IResult<&str, Vec<&str>> {
  Ok((input, vec![]))
}

/// Función auxiliar para agregar el fondo falso de las dimensiones dentro de la pila de operadores.  
/// También agrega una dimensión en la pila de dimensiones.  
///
/// # Parametros
///
/// * `variable` - Variable no atómica
/// * `dim` - Dimensión de la variable
///
/// # Ejemplo
///
/// ```ignore
/// pushear_dimension(TipoVar {
///   nombre: "arreglo".to_owned(),
///   tipo: "entero".to_owned(),
///   dimensiones: vec![4, 3],
///   direccion: 10,
/// }, 2);
/// ```
fn pushear_dimension(variable: TipoVar, dim: i64) {
  PILA_DIMENSIONES.lock().unwrap().push((variable, dim));
  PILA_OPERADORS.lock().unwrap().push("(".to_owned());
}

/// Función auxiliar para eliminar el fondo falso de las dimensiones dentro de la pila de operadores.  
/// También elimina la última dimensión en la pila de dimensiones.  
///
/// # Ejemplo
///
/// ```ignore
/// popear_dimension();
/// ```
fn popear_dimension() {
  PILA_DIMENSIONES.lock().unwrap().pop();
  PILA_OPERADORS.lock().unwrap().pop();
}

/// Función auxiliar para agregar un cuadruplo de verificar el valor dado para el indexamiento de un arreglo sea válido.  
/// Regresa la última variable dentro de la pila de valores.  
///
/// # Parametros
///
/// * `variable` - Variable no atómica
/// * `dim` - Dimensión de la variable - 1
///
/// # Ejemplo
///
/// ```ignore
/// generar_cuadruplo_verificar(TipoVar {
///   nombre: "arreglo".to_owned(),
///   tipo: "entero".to_owned(),
///   dimensiones: vec![4, 3],
///   direccion: 10,
/// }, 1);
/// ```
fn generar_cuadruplo_verificar(variable: TipoVar, dim: usize) -> TipoVar {
  let mut pila_valores = PILA_VALORES.lock().unwrap();
  println!("dim ver: \n{:?}\n", pila_valores);
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  let valor = pila_valores.pop().unwrap();
  drop(pila_valores);
  match cuadruplos.agregar_cuadruplo_verificar(valor.direccion, variable.dimensiones[dim]) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    }
  };
  valor
}

/// Función auxiliar para agregar un cuadruplo de verificar el valor dado para el indexamiento de un arreglo sea válido.  
///
/// # Parametros
///
/// * `variable` - Variable no atómica base
/// * `valor` - Valor con el que se quiere indexar la variable no atómica
/// * `asignacion` - Marca si se está haciendo una asignación
/// * `dimension` - Marca la dimensión del arreglo
///
/// # Ejemplo
///
/// ```ignore
/// generar_cuadruplo_acceder(TipoVar {
///   nombre: "arreglo".to_owned(),
///   tipo: "entero".to_owned(),
///   dimensiones: vec![4, 3],
///   direccion: 10,
/// }, TipoVar {
///   nombre: "temporal1".to_owned(),
///   tipo: "entero".to_owned(),
///   dimensiones: vec![],
///   direccion: 10,
/// }, true, 1);
/// ```
fn generar_cuadruplo_acceder(variable: TipoVar, valor: TipoVar, asignacion: bool, dimension: i64) {
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  let mut constantes = CONSTANTES.lock().unwrap();
  // Consigue la dirección base de la variable atómica como variable constante
  let dir = constantes.agregar_constante(variable.direccion.to_string(), variable.tipo.clone());
  drop(constantes);
  // Suma el valor y la dirección base del arreglo
  match cuadruplos.agregar_cuadruplo_suma_arreglo("+", valor.clone(), dir.clone()) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    }
  };
  
  if dimension == 2 {
    let mut pila_valores = PILA_VALORES.lock().unwrap();
    println!("dim acceder: \n{:?}\n", pila_valores);
    let val = pila_valores.pop().unwrap();
    let offset = pila_valores.pop().unwrap();
    drop(pila_valores);
    match cuadruplos.agregar_cuadruplo("+", offset, val) {
      Ok(_) => (),
      Err(err) => {
        println!("{:?}", err);
      }
    };
    
    let mut pila_valores = PILA_VALORES.lock().unwrap();
    println!("dim acceder 2: \n{:?}\n", pila_valores);
    let val = pila_valores.pop().unwrap();
    drop(pila_valores);
    match cuadruplos.agregar_cuadruplo_suma_arreglo("+", dir.clone(), val.clone()) {
      Ok(_) => (),
      Err(err) => {
        println!("{:?}", err);
      }
    };
  }
  if !asignacion {
    let mut pila_valores = PILA_VALORES.lock().unwrap();
    println!("dim asignacion: \n{:?}\n", pila_valores);
    let apuntador = pila_valores.pop().unwrap();
    drop(pila_valores);
    match cuadruplos.agregar_cuadruplo_acceder(apuntador) {
      Ok(_) => (),
      Err(err) => {
        println!("{:?}", err);
      }
    };
  }
}

/// Función auxiliar para agregar un cuadruplo del offset de la segunda dimensión de una variable no atómica de dos dimensiones.  
///
/// # Parametros
///
/// * `variable` - Variable no atómica base
/// * `valor` - Valor con el que se quiere indexar la variable no atómica
///
/// # Ejemplo
///
/// ```ignore
/// generar_cuadruplo_offset(TipoVar {
///   nombre: "arreglo".to_owned(),
///   tipo: "entero".to_owned(),
///   dimensiones: vec![4, 3],
///   direccion: 10,
/// }, TipoVar {
///   nombre: "temporal1".to_owned(),
///   tipo: "entero".to_owned(),
///   dimensiones: vec![],
///   direccion: 10,
/// });
/// ```
fn generar_cuadruplo_offset(variable: TipoVar, valor: TipoVar) {
  let mut constantes = CONSTANTES.lock().unwrap();
  let mut cuadruplos = CUADRUPLOS.lock().unwrap();
  // Consigue la dirección base de la variable atómica como variable constante
  let dim_constante = constantes.agregar_constante(variable.dimensiones[0].to_string(), variable.tipo.clone());

  // Genera cuadruplo de offset
  match cuadruplos.agregar_cuadruplo("*", valor.clone(), dim_constante.clone()) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    }
  };
}

/// No terminal dimensiones. Sirve para indexar las dimensiones de un arreglos.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
/// * `asignacion` - Marca si es una asignacion
///
/// # Gramática
///
/// ```ignore
/// [ EXP ] [ EXP ]
/// ```
///
/// La segunda dimensión es opcional
///
/// # Ejemplo
///
/// ```ignore
/// match con_dim_decl("[10]") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn con_dim(id_valor: &str, asignacion: bool) -> impl FnMut(&str)  -> IResult<&str, &str> + '_ {
  move |input| {
    let mut next: &str = input;

    // Checa que la variable exista
    let variable = buscar_variable(id_valor);

    // Primera dimensión
    if variable.dimensiones.len() > 0 {
      next = match corchete(next) {
        Ok((next_input, _)) => {
          match variable.dimensiones.len() {
            0 => {
              // Marca error cuando se intenta indexar una variable atómica
              println!("Variable no tiene dimensiones");
              return Err(nom::Err::Error(nom::error::Error {
                input: next,
                code: nom::error::ErrorKind::Tag
              }));
            },
            _ => {}
          };
          // Genera cuadruplos de indexación de variable no atómica
          pushear_dimension(variable.clone(), 1);
          match tuple((delimited(ws, exp, ws), tag("]")))(next_input) {
            Ok((next_i, _)) => {
              popear_dimension();
              let valor = generar_cuadruplo_verificar(variable.clone(), 0);
              // Checa que haya otra dimensión para saber si generar el cuadruplo de offset o de acceso
              match variable.dimensiones.len() {
                1 => { generar_cuadruplo_acceder(variable.clone(), valor.clone(), asignacion, 1); },
                2 => { generar_cuadruplo_offset(variable.clone(), valor.clone()); },
                _ => ()
              };
              next_i
            },
            Err(err) => return Err(err)
          }
        },
        Err(_) => {
          if variable.dimensiones.len() >= 1 {
              println!("Falta dimensión");
              return Err(nom::Err::Error(nom::error::Error {
              input: next,
              code: nom::error::ErrorKind::Tag
            }));
          } else {
            next
          }
        }
      };
      if variable.dimensiones.len() > 1 {
        match corchete(next) {
          Ok((next_input, _)) => {
            pushear_dimension(variable.clone(), 2);
            match tuple((delimited(ws, exp, ws), tag("]")))(next_input) {
              Ok((next_i, _)) => {
                let valor = generar_cuadruplo_verificar(variable.clone(), 1);
                generar_cuadruplo_acceder(variable.clone(), valor.clone(), asignacion, 2);
                popear_dimension();
                return Ok((next_i, "con_dim"));
              },
              Err(err) => return Err(err)
            }
          },
          Err(err) => return Err(err)
        };
      }
    } else {
      {
        let mut pila_val = PILA_VALORES.lock().unwrap();
        pila_val.push(variable.clone());
        println!("dim al final: \n{:?}\n", pila_val);
      }
    }
    Ok((next, "con_dim"))
  }
}

/// Función auxiliar para leer una dimensión en las declaraciones de variables.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match dimension_decl("[10]") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn dimension_decl(input: &str) -> IResult<&str, Vec<&str>> {
  delimited(tuple((tag("["), ws)), num_entero, tuple((ws, tag("]"))))(input)
  .map(|(next_input, res)| {
    (next_input, vec![res.0])
  })
}

/// Función auxiliar para leer dos dimensiones en las declaraciones de variables.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match dos_dimensiones_decl("[4][10]") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn dos_dimensiones_decl(input: &str) -> IResult<&str, Vec<&str>> {
  tuple((dimension_decl, ws, dimension_decl))(input)
  .map(|(next_input, (dimension_1, _, dimension_2))| {
    (next_input, vec![dimension_1[0], dimension_2[0]])
  })
}

/// No terminal dim_decl. Sirve para declarar las dimensiones de un arreglos.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// [ num_entero ] [ num_entero ]
/// ```
///
/// La segunda dimensión es opcional
///
/// # Ejemplo
///
/// ```ignore
/// match con_dim_decl("[10]") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn con_dim_decl(input: &str) -> IResult<&str, Vec<&str>> {
  alt((dos_dimensiones_decl, dimension_decl, ws_vec))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ws_vec() {
    assert_eq!(ws_vec("aaaa"), Ok(("aaaa", vec![])));
    assert_eq!(ws_vec("bbbb"), Ok(("bbbb", vec![])));
    assert_eq!(ws_vec("cccc"), Ok(("cccc", vec![])));
    assert_eq!(ws_vec("    "), Ok(("    ", vec![])));
  }

  #[test]
  fn test_dimension_decl() {
    assert_eq!(dimension_decl("[1 ]"),      Ok(("", vec!["1"])));
    assert_eq!(dimension_decl("[ 78]"),     Ok(("", vec!["78"])));
    assert_eq!(dimension_decl("[  69  ]"),  Ok(("", vec!["69"])));
  }

  #[test]
  fn test_dos_dimensiones_decl() {
    assert_eq!(dos_dimensiones_decl("[420][2]"),          Ok(("", vec!["420", "2"])));
    assert_eq!(dos_dimensiones_decl("[ 69666][ 0 ]"),     Ok(("", vec!["69666", "0"])));
    assert_eq!(dos_dimensiones_decl("[  1  ][   2 ]"),    Ok(("", vec!["1", "2"])));
  }

  #[test]
  fn test_con_dim_decl() {
    assert_eq!(con_dim_decl("[7]"),     Ok(("", vec!["7"])));
    assert_eq!(con_dim_decl("[3][13]"), Ok(("", vec!["3", "13"])));
    assert_eq!(con_dim_decl("aaaa"),    Ok(("aaaa", vec![])));
  }

  #[test]
  fn test_con_dim() {
    let hola = "hola";
    let adios = "adios";
    let wiii = "wiii";
    let mut tabla_variables = VARIABLES.lock().unwrap();
    tabla_variables.agregar_variable(hola.to_owned(), "entero".to_owned(), vec![5], 200);
    tabla_variables.agregar_variable(adios.to_owned(), "entero".to_owned(), vec![5, 10], 400);
    tabla_variables.agregar_variable(wiii.to_owned(), "entero".to_owned(), vec![], 600);
    drop(tabla_variables);
    assert_eq!(con_dim(hola, false)("[id]"),     Ok(("", "con_dim")));
    assert_eq!(con_dim(adios, false)("[id][id]"), Ok(("", "con_dim")));
    assert_eq!(con_dim(wiii, false)("aaaa"),     Ok(("aaaa", "con_dim")));
  }
}
