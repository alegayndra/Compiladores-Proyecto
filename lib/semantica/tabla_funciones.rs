use std::collections::HashMap;

use crate::semantica::tabla_variables::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TipoFunc {
  pub nombre: String,
  pub tipo: String,
  pub variables: TablaVariables,
  pub direccion: i64,
  pub parametros: Vec<TipoVar>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TablaFunciones {
  pub tabla: HashMap<String, TipoFunc>
}

impl TablaFunciones {
  pub fn agregar_funcion(&mut self, nombre_func: String, tipo_func: String, dir: i64) -> Result<(&str, TipoFunc), (&str, String)> {
    match self.tabla.contains_key(&nombre_func) {
      true => Err(("Nombre de funcion ocupado", nombre_func.clone())),
      false =>  {
        let func = TipoFunc { 
          nombre: nombre_func.clone(),
          tipo: tipo_func.clone(),
          variables: TablaVariables { tabla: HashMap::new() },
          parametros: vec![],
          direccion: dir
        };
        self.tabla.insert(nombre_func.clone(), func.clone());
        Ok(("Funcion agregada", func.clone()))
      }
    }
  }

  pub fn buscar_funcion(&self, nombre_func: String) -> Result<(&str, TipoFunc), (&str, String)> {
    match self.tabla.get(&nombre_func) {
      Some(func) => Ok(("Funcion existente", func.clone())),
      None => Err(("Funcion no existente", nombre_func.clone()))
    }
  }

  pub fn agregar_variable(&mut self, nombre_func: String, nombre_var: String, tipo_var: String, dims: Vec<String>, dir: i64) -> Result<(&str, String, TipoVar), (&str, String, String)> {
    match self.tabla.get_mut(&nombre_func) {  
      Some(funcion) => match funcion.variables.agregar_variable(nombre_var.clone(), tipo_var.clone(), dims, dir) {
        Ok((_, var)) => Ok(("Variable agregada a funcion", nombre_func.clone(), var)),
        Err((_, nombre_var)) => Err(("Nombre de variable ocupado en funcion", nombre_func.clone(), nombre_var))
      },
      None => Err(("Funcion no existente", nombre_func.clone(), "".to_owned()))
    }
  }

  pub fn buscar_variable(&self, nombre_func: String, nombre_var: String) -> Result<(&str, String, TipoVar), (&str, String, String)> {
    match self.tabla.get(&nombre_func) {
      Some(funcion) => match funcion.variables.buscar_variable(nombre_var.clone()) {
        Ok((_, var)) => Ok(("Variable existente en funcion", nombre_func.clone(), var.clone())),
        Err((_, nombre_var)) => Err(("Variable no existente en funcion", nombre_func.clone(), nombre_var))
      },
      None => Err(("Funcion no existente", nombre_func.clone(), "".to_owned()))
    }
  }

  pub fn agregar_parametro(&mut self, nombre_func: String, nombre_var: String, tipo_var: String, dims: Vec<String>, dir: i64) -> Result<(&str, String, TipoVar), (&str, String, String)> {
    match self.tabla.get_mut(&nombre_func) {
      Some(funcion) => {
        match funcion.variables.agregar_variable(nombre_var.clone(), tipo_var.clone(), dims.clone(), dir) {
          Ok((_, var)) => {
            funcion.parametros.push(var.clone());
            Ok(("Parametro agregado a funcion", nombre_func.clone(), var))
          },
          Err((_, nom_var)) => Err(("Nombre de variable ocupado en funcion", nombre_func.clone(), nom_var))
        }
      },
      None => Err(("Funcion no existente", nombre_func.clone(), "".to_owned()))
    }
  }

  // pub fn buscar_parametro(&mut self, nombre_func: String, nombre_var: String) -> Result<(&str, String), (&str, String)> {
  //   match self.tabla.get(&nombre_func) {
  //     Some(funcion) => funcion.parametros.hash.buscar_variable(nombre_var),
  //     None => Err(("Funcion no existente", nombre_func.clone()))
  //   }
  // }
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
    let dims = vec![];

    let dir_func = 14000;
    let func_entera = TipoFunc {
      nombre: "func".to_owned(),
      tipo: "entero".to_owned(),
      variables: TablaVariables { tabla: HashMap::new() },
      parametros: vec![],
      direccion: 14000
    };

    let dir_var = 1000;
    let var_entera = TipoVar {
      nombre: "variable".to_owned(),
      tipo: "entero".to_owned(),
      dimensiones: vec![],
      direccion: 1000
    };

    assert_eq!(
      tabla.agregar_funcion("func".to_owned(), "entero".to_owned(), dir_func), 
      Ok(("Funcion agregada", func_entera.clone()))
    );
    assert_eq!(
      tabla.agregar_funcion("func".to_owned(), "entero".to_owned(), dir_func),
      Err(("Nombre de funcion ocupado", "func".to_owned()))
    );

    assert_eq!(
      tabla.buscar_funcion("func".to_owned()),
      Ok(("Funcion existente", func_entera.clone()))
    );
    assert_eq!(
      tabla.buscar_funcion("a".to_owned()),
      Err(("Funcion no existente", "a".to_owned()))
    );

    assert_eq!(
      tabla.agregar_variable("func".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var), 
      Ok(("Variable agregada a funcion", "func".to_owned(), var_entera.clone()))
    );
    assert_eq!(
      tabla.agregar_variable("func".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var),
      Err(("Nombre de variable ocupado en funcion", "func".to_owned(), "variable".to_owned()))
    );
    assert_eq!(
      tabla.agregar_variable("a".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var),
      Err(("Funcion no existente", "a".to_owned(), "".to_owned()))
    );

    assert_eq!(
      tabla.buscar_variable("func".to_owned(), "variable".to_owned()), 
      Ok(("Variable existente en funcion", "func".to_owned(), var_entera.clone()))
    );
    assert_eq!(
      tabla.buscar_variable("func".to_owned(), "a".to_owned()), 
      Err(("Variable no existente en funcion", "func".to_owned(), "a".to_owned()))
    );
    assert_eq!(
      tabla.buscar_variable("a".to_owned(), "a".to_owned()), 
      Err(("Funcion no existente", "a".to_owned(), "".to_owned()))
    );
  }
}
