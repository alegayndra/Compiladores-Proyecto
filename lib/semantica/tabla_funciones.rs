use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::semantica::tabla_variables::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TipoFunc {
  pub nombre: String,
  pub tipo: String,
  pub parametros: HashMap<String, TipoVar>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TablaFunciones {
  pub tabla: HashMap<String, TipoFunc>
}

impl TipoFunc {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.nombre.hash(state);
  }
}

impl TablaFunciones {
  pub fn agregar_funcion(&mut self, nombre_func: String, tipo_func: String) -> &str {
    match self.tabla.contains_key(&nombre_func) {
      true => "Nombre de funcion ocupado",
      false =>  {
        self.tabla.insert(nombre_func.clone(), TipoFunc { 
          nombre: nombre_func.clone(),
          tipo: tipo_func.clone(),
          parametros: HashMap::new() 
        });
        "Funcion agregada"
      }
    }
  }

  pub fn buscar_funcion(&mut self, nombre_func: String) -> &str {
    match self.tabla.contains_key(&nombre_func) {
      true => "Funcion existente",
      false =>  "Funcion no existe"
    }
  }

  pub fn agregar_variable(&mut self, nombre_func: String, nombre_var: String, tipo_var: String) -> &str {
    match self.tabla.get_mut(&nombre_func) {
      Some(funcion) => match funcion.parametros.contains_key(&nombre_var) {
        true => "Nombre de variable ocupado",
        false => {
          funcion.parametros.insert(nombre_var.clone(), TipoVar {
            nombre: nombre_var.clone(),
            tipo: tipo_var.clone()
          });
          "Variable agregada"
        }
      }
      None => "Funcion no existente"
    }
  }

  pub fn buscar_variable(&mut self, nombre_func: String, nombre_var: String) -> &str {
    match self.tabla.get(&nombre_func) {
      Some(funcion) => match funcion.parametros.contains_key(&nombre_var) {
        true => "Variable existente",
        false => "Variable no existente"
      },
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
