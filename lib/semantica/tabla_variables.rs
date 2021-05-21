use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TipoVar {
  pub nombre: String,
  pub direccion: i64,
  pub tipo: String,
  pub dimensiones: Vec<String>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TablaVariables {
  pub tabla: HashMap<String, TipoVar>
}

impl TablaVariables {
  pub fn agregar_variable(&mut self, nombre_var: String, tipo_var: String, dims: Vec<String>) -> Result<(&str, String), (&str, String)> {
    match self.tabla.contains_key(&nombre_var) {
      true => Err(("Nombre de variable ocupado", nombre_var.clone())),
      false => {
        self.tabla.insert(nombre_var.clone(), TipoVar {
          nombre: nombre_var.clone(),
          tipo: tipo_var.clone(),
          dimensiones: dims,
          direccion: 0
        });
        Ok(("Variable agregada", nombre_var.clone()))
      }
    }
  }

  pub fn buscar_variable(&self, nombre_var: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.contains_key(&nombre_var) {
      true => Ok(("Variable existente", nombre_var.clone())),
      false => Err(("Variable no existente", nombre_var.clone()))
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
  fn test_tabla_variables() {
    let mut tabla : TablaVariables = TablaVariables { tabla: HashMap::new() };
    let dims = vec![];
    assert_eq!(
      tabla.agregar_variable("variable".to_owned(), "entero".to_owned(), dims.clone()), 
      Ok(("Variable agregada", "variable".to_owned()))
    );
    assert_eq!(
      tabla.agregar_variable("variable".to_owned(), "entero".to_owned(), dims.clone()), 
      Err(("Nombre de variable ocupado", "variable".to_owned()))
    );
    assert_eq!(
      tabla.buscar_variable("variable".to_owned()),
      Ok(("Variable existente", "variable".to_owned()))
    );
    assert_eq!(
      tabla.buscar_variable("a".to_owned()),
      Err(("Variable no existente", "a".to_owned()))
    );
  }
}

