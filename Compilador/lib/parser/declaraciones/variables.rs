//! Módulo que se encarga del analisis de las variables

use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::{tuple, preceded}
};

use crate::scanners::ws::*;
use crate::scanners::tipos::*;
use crate::scanners::id::*;
use crate::semantica::globales::*;

/// Función axuliar para parsear las variabes de tipos personalizados.  
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
/// ```ignore
/// TIPO_COMPUESTO id;
/// ```
///
/// ```ignore
/// match variable_compuesta("Persona nombre;") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn variable_compuesta(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;
  let tipo_var: &str;

  // Consigue tipo de la variable
  next = match id(next) {
    Ok((next_input, res)) => {
      tipo_var = res;
      next_input
    },
    Err(err) => return Err(err)
  };

  // Consigue el ID
  next = match preceded(ws, id)(next) {
    Ok((next_input, var)) => {
      agregar_variable_a_tabla(var, tipo_var, vec![]);
      next_input
    },
    Err(err) => return Err(err)
  };

  // Itera sobre las diferentes variables declaradas
  loop {
    // Checa que haya una coma, indicando que se está declarando otra variable
    next = match tuple((ws, tag(","), ws))(next) {
      Ok((next_input, _)) => next_input,
      _ => {
        break;
      }
    };

    // Consigue ID y sus dimensiones
    next = match id(next) {
      Ok((next_input, var)) => {
        agregar_variable_a_tabla(var, tipo_var, vec![]);
        next_input
      },
      Err(err) => return Err(err)
    };
  };

  Ok((next, "variable compuesta"))
}

/// Función axuliar agregar una variable a las tablas de variables de la semántica
///
/// # Parametros
///
/// * `var` - ID de la variable
/// * `tipo_var` - Tipo de la variable
/// * `dims` - Dimensiones de la variable
///
/// # Ejemplo
///
/// ```ignore
/// agregar_variable_a_tabla("numero", "entero", vec!["3"]);
/// ```
fn agregar_variable_a_tabla(var: &str, tipo_var: &str, dims: Vec<&str>) {
  let mut dims_nums : Vec<i64> = vec![];
  for dim in dims {
    dims_nums.push(dim.to_owned().parse::<i64>().unwrap());
  }

  let dir = match conseguir_direccion(tipo_var, "variable", 0, dims_nums.clone()) {
    Ok(num) => num,
    Err(err) => {
      println!("{:?}", err);
      return;
    }
  };

  match VARIABLES.lock().unwrap().agregar_variable(var.to_owned(), tipo_var.to_owned(), dims_nums.clone(), dir) {
    Ok(_) => (),
    Err(err) => {
      println!("{:?}", err);
    },
  }

  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();

  if contexto_clase.clone() != "".to_owned() {
    if contexto_funcion.clone() != "".to_owned() {
      match CLASES.lock().unwrap().agregar_atributo(contexto_clase.to_string(), var.to_owned(), tipo_var.to_owned(), dims_nums.clone(), dir) {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      }
    } else {
      match CLASES.lock().unwrap().agregar_variable_metodo(contexto_clase.to_string(), contexto_funcion.to_string(), var.to_owned(), tipo_var.to_owned(), dims_nums.clone(), dir, 0) {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        },
      }
    }
  } else {
    match FUNCIONES.lock().unwrap().agregar_variable(contexto_funcion.to_string(), var.to_owned(), tipo_var.to_owned(), dims_nums.clone(), dir, 0) {
      Ok(_) => (),
      Err(err) => {
        println!("{:?}", err);
      },
    }
  }
}

/// Función axuliar para parsear las variabes de tipos primitivos.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Gramática
///
/// ```ignore
/// TIPO id DIMENSIONES;
/// ```
///
/// # Ejemplo
///
/// ```ignore
/// match variable_normal("entero nombre;") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
fn variable_normal(input: &str) -> IResult<&str, &str> {
  let mut next: &str = input;
  let tipo_var: &str;

  // Consigue tipo de la variable
  next = match tipo(next) {
    Ok((next_input, res)) => {
      tipo_var = res;
      next_input
    },
    Err(err) => return Err(err)
  };

  // Consigue el ID y sus dimensiones
  next = match preceded(ws, id_con_dim_decl)(next) {
    Ok((next_input, (var, dims))) => {
      agregar_variable_a_tabla(var, tipo_var, dims);
      next_input
    },
    Err(err) => return Err(err)
  };

  // Itera sobre las diferentes variables declaradas
  loop {
    // Checa que haya una coma, indicando que se está declarando otra variable
    next = match tuple((ws, tag(","), ws))(next) {
      Ok((next_input, _)) => next_input,
      _ => {
        break;
      }
    };

    // Consigue ID y sus dimensiones
    next = match id_con_dim_decl(next) {
      Ok((next_input, (var, dims))) => {
        agregar_variable_a_tabla(var, tipo_var, dims);
        next_input
      },
      Err(err) => return Err(err)
    };
  };

  Ok((next, "variable normal"))
}

/// No terminal de la declaracion de variables.  
/// Regresa un IResult, un Result nativo modificado de la libreria de Nom que incluye el input restante.
///
/// # Parametros
///
/// * `input` - Input a parsear
///
/// # Ejemplo
///
/// ```ignore
/// match diferentes_declaraciones("entero nombre;") {
///   Ok((next_input, res)) => res, // parseo éxitoso
///   Err(err) => err, // error en parseo
/// };
/// ```
pub fn variables(input: &str) -> IResult<&str, &str> {
  tuple((ws, alt((variable_normal, variable_compuesta)), ws, tag(";"), ws))
  (input)
  .map(|(next_input, _res)| {
    (next_input, "variables")
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_variable_compuesta() {
    assert_eq!(variable_compuesta("id id;"), Ok((";", "variable compuesta")));
    assert_eq!(variable_compuesta("id id, id;"), Ok((";", "variable compuesta")));
  }

  #[test]
  fn test_variable_normal() {
    assert_eq!(variable_normal("entero id;"), Ok((";", "variable normal")));
    assert_eq!(variable_normal("flotante id, id2;"), Ok((";", "variable normal")));
    assert_eq!(variable_normal("char id, id2;"), Ok((";", "variable normal")));
  }

  #[test]
  fn test_variables() {
    assert_eq!(variables("Persona id;"),        Ok(("", "variables")));
    assert_eq!(variables("Persona id, id;"),    Ok(("", "variables")));
    assert_eq!(variables("entero id;"),         Ok(("", "variables")));
    assert_eq!(variables("entero id[74];"),     Ok(("", "variables")));
    assert_eq!(variables("entero id[22][0];"), Ok(("", "variables")));
    assert_eq!(variables("entero id, id;"),     Ok(("", "variables")));
    assert_eq!(variables("entero id[1], id;"), Ok(("", "variables")));
  }
}
