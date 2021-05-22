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
  pub fn agregar_variable(&mut self, nombre_var: String, tipo_var: String, dims: Vec<String>, dir: i64) -> Result<(&str, TipoVar), (&str, String)> {
    match self.tabla.contains_key(&nombre_var) {
      true => Err(("Nombre de variable ocupado", nombre_var.clone())),
      false => {
        let var = TipoVar {
          nombre: nombre_var.clone(),
          tipo: tipo_var.clone(),
          dimensiones: dims,
          direccion: dir
        };
        self.tabla.insert(nombre_var.clone(), var.clone());
        Ok(("Variable agregada", var))
      }
    }
  }

  pub fn buscar_variable(&self, nombre_var: String) -> Result<(&str, TipoVar), (&str, String)> {
    match self.tabla.get(&nombre_var) {
      Some(var) => Ok(("Variable existente", var.clone())),
      None => Err(("Variable no existente", nombre_var.clone()))
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
      tabla.agregar_variable("variable".to_owned(), "entero".to_owned(), dims.clone(), 1000), 
      Ok(("Variable agregada", TipoVar {
        nombre: "variable".to_owned(),
        tipo: "entero".to_owned(),
        dimensiones: vec![],
        direccion: 1000
      }))
    );
    assert_eq!(
      tabla.agregar_variable("variable".to_owned(), "entero".to_owned(), dims.clone(), 1001), 
      Err(("Nombre de variable ocupado", "variable".to_owned()))
    );
    assert_eq!(
      tabla.buscar_variable("variable".to_owned()),
      Ok(("Variable existente", TipoVar {
        nombre: "variable".to_owned(),
        tipo: "entero".to_owned(),
        dimensiones: vec![],
        direccion: 1000
      }))
    );
    assert_eq!(
      tabla.buscar_variable("a".to_owned()),
      Err(("Variable no existente", "a".to_owned()))
    );
  }
}

