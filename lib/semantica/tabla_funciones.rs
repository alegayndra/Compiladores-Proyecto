use std::collections::HashMap;

use crate::semantica::tabla_variables::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TipoFunc {
  pub nombre: String,
  pub tipo: String,
  pub parametros_hash: TablaVariables,
  pub parametros_vec: Vec<TipoVar>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TablaFunciones {
  pub tabla: HashMap<String, TipoFunc>
}

impl TablaFunciones {
  pub fn agregar_funcion(&mut self, nombre_func: String, tipo_func: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.contains_key(&nombre_func) {
      true => Err(("Nombre de funcion ocupado", nombre_func.clone())),
      false =>  {
        self.tabla.insert(nombre_func.clone(), TipoFunc { 
          nombre: nombre_func.clone(),
          tipo: tipo_func.clone(),
          parametros_hash: TablaVariables { tabla: HashMap::new() },
          parametros_vec: vec![]
        });
        Ok(("Funcion agregada", nombre_func.clone()))
      }
    }
  }

  pub fn buscar_funcion(&self, nombre_func: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.contains_key(&nombre_func) {
      true => Ok(("Funcion existente", nombre_func.clone())),
      false => Err(("Funcion no existente", nombre_func.clone()))
    }
  }

  pub fn agregar_variable(&mut self, nombre_func: String, nombre_var: String, tipo_var: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.get_mut(&nombre_func) {
      // Some(funcion) => match funcion.parametros_hash.buscar_variable(nombre_var.clone()) {
      //   Ok(_) => {
      //     funcion.parametros_vec.push(TipoVar {
      //       nombre: nombre_var.clone(),
      //       tipo: tipo_var.clone()
      //     });
      //     funcion.parametros_hash.agregar_variable(nombre_var.clone(), tipo_var.clone())
      //   },
      //   Err(err) => Err(err)
      // },
      Some(funcion) => funcion.parametros_hash.agregar_variable(nombre_var.clone(), tipo_var.clone()),
      None => Err(("Funcion no existente", nombre_func.clone()))
    }
  }

  pub fn buscar_variable(&mut self, nombre_func: String, nombre_var: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.get(&nombre_func) {
      Some(funcion) => funcion.parametros_hash.buscar_variable(nombre_var),
      None => Err(("Funcion no existente", nombre_func.clone()))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_tabla_funciones() {
    let mut tabla : TablaFunciones = TablaFunciones { tabla: HashMap::new() };
    assert_eq!(
      tabla.agregar_funcion("func".to_owned(), "entero".to_owned()), 
      Ok(("Funcion agregada", "func".to_owned()))
    );
    assert_eq!(
      tabla.agregar_funcion("func".to_owned(), "entero".to_owned()),
      Err(("Nombre de funcion ocupado", "func".to_owned()))
    );

    assert_eq!(
      tabla.buscar_funcion("func".to_owned()),
      Ok(("Funcion existente", "func".to_owned()))
    );
    assert_eq!(
      tabla.buscar_funcion("a".to_owned()),
      Err(("Funcion no existente", "a".to_owned()))
    );

    assert_eq!(
      tabla.agregar_variable("func".to_owned(), "variable".to_owned(), "entero".to_owned()), 
      Ok(("Variable agregada", "variable".to_owned()))
    );
    assert_eq!(
      tabla.agregar_variable("func".to_owned(), "variable".to_owned(), "entero".to_owned()),
      Err(("Nombre de variable ocupado", "variable".to_owned()))
    );
    assert_eq!(
      tabla.agregar_variable("a".to_owned(), "variable".to_owned(), "entero".to_owned()),
      Err(("Funcion no existente", "a".to_owned()))
    );

    assert_eq!(
      tabla.buscar_variable("func".to_owned(), "variable".to_owned()), 
      Ok(("Variable existente", "variable".to_owned()))
    );
    assert_eq!(
      tabla.buscar_variable("func".to_owned(), "a".to_owned()), 
      Err(("Variable no existente", "a".to_owned()))
    );
    assert_eq!(
      tabla.buscar_variable("a".to_owned(), "a".to_owned()), 
      Err(("Funcion no existente", "a".to_owned()))
    );
  }
}
