use std::collections::HashMap;

use crate::semantica::tabla_variables::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TipoFunc {
  pub nombre: String,
  pub tipo: String,
  pub parametros: TablaVariables,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TablaFunciones {
  pub tabla: HashMap<String, TipoFunc>
}

impl TablaFunciones {
  pub fn agregar_funcion(&mut self, nombre_func: String, tipo_func: String) -> &str {
    match self.tabla.contains_key(&nombre_func) {
      true => "Nombre de funcion ocupado",
      false =>  {
        self.tabla.insert(nombre_func.clone(), TipoFunc { 
          nombre: nombre_func.clone(),
          tipo: tipo_func.clone(),
          parametros: TablaVariables { tabla: HashMap::new() } 
        });
        "Funcion agregada"
      }
    }
  }

  pub fn buscar_funcion(&self, nombre_func: String) -> &str {
    match self.tabla.contains_key(&nombre_func) {
      true => "Funcion existente",
      false =>  "Funcion no existe"
    }
  }

  pub fn agregar_variable(&mut self, nombre_func: String, nombre_var: String, tipo_var: String) -> &str {
    match self.tabla.get_mut(&nombre_func) {
      Some(funcion) => funcion.parametros.agregar_variable(nombre_var, tipo_var),
      None => "Funcion no existente"
    }
  }

  pub fn buscar_variable(&mut self, nombre_func: String, nombre_var: String) -> &str {
    match self.tabla.get(&nombre_func) {
      Some(funcion) => funcion.parametros.buscar_variable(nombre_var),
      None => "Funcion no existente"
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
  fn test_agregar_funcion() {
    let mut tabla : TablaFunciones = TablaFunciones { tabla: HashMap::new() };
    assert_eq!(tabla.agregar_funcion("func".to_string(), "entero".to_string()), "Funcion agregada");
    assert_eq!(tabla.agregar_funcion("func".to_string(), "entero".to_string()), "Nombre de funcion ocupado");
    assert_eq!(tabla.buscar_funcion("func".to_string()), "Funcion existente");
    assert_eq!(tabla.agregar_variable("func".to_string(), "variable".to_string(), "entero".to_string()), "Variable agregada");
    assert_eq!(tabla.agregar_variable("func".to_string(), "variable".to_string(), "entero".to_string()), "Nombre de variable ocupado");
    assert_eq!(tabla.buscar_variable("func".to_string(), "variable".to_string()), "Variable existente");
    assert_eq!(tabla.buscar_variable("func".to_string(), "a".to_string()), "Variable no existente");
    assert_eq!(tabla.buscar_variable("a".to_string(), "a".to_string()), "Funcion no existente");
  }
}
