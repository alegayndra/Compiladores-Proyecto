use std::collections::HashMap;

use crate::semantica::tabla_variables::*;
use crate::semantica::tabla_funciones::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TipoClase {
  pub nombre: String,
  pub padre: String,
  pub metodos: TablaFunciones,
  pub atributos: TablaVariables,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TablaClases {
  pub tabla: HashMap<String, TipoClase>
}

impl TablaClases {
  pub fn agregar_clase(&mut self, nombre_clase: String, nombre_padre: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.contains_key(&nombre_clase) {
      true => Err(("Nombre de clase ocupado", nombre_clase.clone())),
      false =>  {
        self.tabla.insert(nombre_clase.clone(), TipoClase { 
          nombre: nombre_clase.clone(),
          padre: nombre_padre.clone(),
          atributos: TablaVariables { tabla: HashMap::new() } ,
          metodos: TablaFunciones { tabla: HashMap::new() } 
        });
        Ok(("Clase agregada", nombre_clase.clone()))
      }
    }
  }

  pub fn buscar_clase(&self, nombre_clase: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.contains_key(&nombre_clase) {
      true => Ok(("Clase existente", nombre_clase.clone())),
      false =>  Err(("Clase no existente", nombre_clase.clone()))
    }
  }

  pub fn agregar_metodo(&mut self, nombre_clase: String, nombre_func: String, tipo_func: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.get_mut(&nombre_clase) {
      Some(clase) => clase.metodos.agregar_funcion(nombre_func, tipo_func),
      None => Err(("Clase no existente", nombre_clase.clone()))
    }
  }

  pub fn buscar_metodo(&self, nombre_clase: String, nombre_func: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.get(&nombre_clase) {
      Some(clase) => clase.metodos.buscar_funcion(nombre_func),
      None => Err(("Clase no existente", nombre_clase.clone()))
    }
  }

  pub fn agregar_parametro_metodo(&mut self, nombre_clase: String, nombre_func: String, nombre_var: String, tipo_var: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.get_mut(&nombre_clase) {
      Some(clase) => match clase.metodos.tabla.get_mut(&nombre_func) {
        // Some(metodo) => match metodo.parametros_hash.buscar_variable(nombre_var.clone()) {
        //   Ok(_) => {
        //     metodo.parametros_vec.push(TipoVar {
        //       nombre: nombre_var.clone(),
        //       tipo: tipo_var.clone()
        //     });
        //     metodo.parametros_hash.agregar_variable(nombre_var.clone(), tipo_var.clone())
        //   },
        //   Err(err) => Err(err)
        // },
        Some(metodo) => metodo.parametros_hash.agregar_variable(nombre_var.clone(), tipo_var.clone()),
        None => Err(("Metodo no existente", nombre_func.clone()))
      },
      None => Err(("Clase no existente", nombre_clase.clone()))
    }
  }

  pub fn buscar_parametro_metodo(&self, nombre_clase: String, nombre_func: String, nombre_var: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.get(&nombre_clase) {
      Some(clase) => match clase.metodos.tabla.get(&nombre_func) {
        Some(metodo) => metodo.parametros_hash.buscar_variable(nombre_var),
        None => Err(("Metodo no existente", nombre_func.clone()))
      },
      None => Err(("Clase no existente", nombre_clase.clone()))
    }
  }

  pub fn agregar_atributo(&mut self, nombre_clase: String, nombre_var: String, tipo_var: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.get_mut(&nombre_clase) {
      Some(clase) => clase.atributos.agregar_variable(nombre_var, tipo_var),
      None => Err(("Clase no existente", nombre_clase.clone()))
    }
  }

  pub fn buscar_atributo(&self, nombre_clase: String, nombre_var: String) -> Result<(&str, String), (&str, String)> {
    match self.tabla.get(&nombre_clase) {
      Some(clase) => clase.atributos.buscar_variable(nombre_var),
      None => Err(("Clase no existente", nombre_clase.clone()))
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
  fn test_tabla_clases() {
    let mut tabla : TablaClases = TablaClases { tabla: HashMap::new() };
    assert_eq!(
      tabla.agregar_clase("Persona".to_owned(), "".to_owned()),
      Ok(("Clase agregada", "Persona".to_owned()))
    );
    assert_eq!(
      tabla.agregar_clase("Persona".to_owned(), "".to_owned()),
      Err(("Nombre de clase ocupado", "Persona".to_owned()))
    );
    assert_eq!(
      tabla.buscar_clase("Persona".to_owned()),
      Ok(("Clase existente", "Persona".to_owned()))
    );
    assert_eq!(
      tabla.buscar_clase("Estudiante".to_owned()),
      Err(("Clase no existente", "Estudiante".to_owned()))
    );

    assert_eq!(
      tabla.agregar_metodo("Persona".to_owned(), "func".to_owned(), "entero".to_owned()),
      Ok(("Funcion agregada", "func".to_owned()))
    );
    assert_eq!(
      tabla.agregar_metodo("Persona".to_owned(), "func".to_owned(), "entero".to_owned()),
      Err(("Nombre de funcion ocupado", "func".to_owned()))
    );
    assert_eq!(
      tabla.buscar_metodo("Persona".to_owned(), "func".to_owned()),
      Ok(("Funcion existente", "func".to_owned()))
    );
    assert_eq!(
      tabla.buscar_metodo("Persona".to_owned(), "a".to_owned()),
      Err(("Funcion no existente", "a".to_owned()))
    );
    assert_eq!(
      tabla.buscar_metodo("Estudiante".to_owned(), "a".to_owned()),
      Err(("Clase no existente", "Estudiante".to_owned()))
    );

    assert_eq!(
      tabla.agregar_parametro_metodo("Persona".to_owned(), "func".to_owned(), "variable".to_owned(), "entero".to_owned()),
      Ok(("Variable agregada", "variable".to_owned()))
    );
    assert_eq!(
      tabla.agregar_parametro_metodo("Persona".to_owned(), "func".to_owned(), "variable".to_owned(), "entero".to_owned()),
      Err(("Nombre de variable ocupado", "variable".to_owned()))
    );
    assert_eq!(
      tabla.agregar_parametro_metodo("Persona".to_owned(), "a".to_owned(), "variable".to_owned(), "entero".to_owned()),
      Err(("Metodo no existente", "a".to_owned()))
    );
    assert_eq!(
      tabla.agregar_parametro_metodo("Estudiante".to_owned(), "a".to_owned(), "variable".to_owned(), "entero".to_owned()),
      Err(("Clase no existente", "Estudiante".to_owned()))
    );
    assert_eq!(
      tabla.buscar_parametro_metodo("Persona".to_owned(), "func".to_owned(), "variable".to_owned()),
      Ok(("Variable existente", "variable".to_owned()))
    );
    assert_eq!(
      tabla.buscar_parametro_metodo("Persona".to_owned(), "func".to_owned(), "a".to_owned()),
      Err(("Variable no existente", "a".to_owned()))
    );
    assert_eq!(
      tabla.buscar_parametro_metodo("Persona".to_owned(), "a".to_owned(), "variable".to_owned()),
       Err(("Metodo no existente", "a".to_owned()))
      );
    assert_eq!(
      tabla.buscar_parametro_metodo("Estudiante".to_owned(), "a".to_owned(), "variable".to_owned()),
      Err(("Clase no existente", "Estudiante".to_owned()))
    );

    assert_eq!(
      tabla.agregar_atributo("Persona".to_owned(), "variable".to_owned(), "entero".to_owned()),
      Ok(("Variable agregada", "variable".to_owned()))
    );
    assert_eq!(
      tabla.agregar_atributo("Persona".to_owned(), "variable".to_owned(), "entero".to_owned()),
      Err(("Nombre de variable ocupado", "variable".to_owned()))
    );
    assert_eq!(
      tabla.buscar_atributo("Persona".to_owned(), "variable".to_owned()),
      Ok(("Variable existente", "variable".to_owned()))
    );
    assert_eq!(
      tabla.buscar_atributo("Persona".to_owned(), "a".to_owned()),
      Err(("Variable no existente", "a".to_owned()))
    );
    assert_eq!(
      tabla.buscar_atributo("Estudiante".to_owned(), "a".to_owned()),
      Err(("Clase no existente", "Estudiante".to_owned()))
    );
  }
}
