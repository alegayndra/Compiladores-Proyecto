use std::collections::HashMap;

use crate::semantica::tabla_variables::*;
use crate::semantica::tabla_funciones::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TipoClase {
  pub nombre: String,
  pub metodos: TablaFunciones,
  pub atributos: TablaVariables,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TablaClases {
  pub tabla: HashMap<String, TipoClase>
}

impl TablaClases {
  pub fn agregar_clase(&mut self, nombre_clase: String) -> &str {
    match self.tabla.contains_key(&nombre_clase) {
      true => "Nombre de clase ocupado",
      false =>  {
        self.tabla.insert(nombre_clase.clone(), TipoClase { 
          nombre: nombre_clase.clone(),
          atributos: TablaVariables { tabla: HashMap::new() } ,
          metodos: TablaFunciones { tabla: HashMap::new() } 
        });
        "Clase agregada"
      }
    }
  }

  pub fn buscar_clase(&self, nombre_clase: String) -> &str {
    match self.tabla.contains_key(&nombre_clase) {
      true => "Clase existente",
      false =>  "Clase no existente"
    }
  }

  pub fn agregar_metodo(&mut self, nombre_clase: String, nombre_func: String, tipo_func: String) -> &str {
    match self.tabla.get_mut(&nombre_clase) {
      Some(clase) => clase.metodos.agregar_funcion(nombre_func, tipo_func),
      None => "Clase no existente"
    }
  }

  pub fn buscar_metodo(&self, nombre_clase: String, nombre_func: String) -> &str {
    match self.tabla.get(&nombre_clase) {
      Some(clase) => clase.metodos.buscar_funcion(nombre_func),
      None => "Clase no existente"
    }
  }

  pub fn agregar_parametro_metodo(&mut self, nombre_clase: String, nombre_func: String, nombre_var: String, tipo_var: String) -> &str {
    match self.tabla.get_mut(&nombre_clase) {
      Some(clase) => match clase.metodos.tabla.get_mut(&nombre_func) {
        Some(metodo) => metodo.parametros.agregar_variable(nombre_var, tipo_var),
        None => "Metodo no existente"
      },
      None => "Clase no existente"
    }
  }

  pub fn buscar_parametro_metodo(&self, nombre_clase: String, nombre_func: String, nombre_var: String) -> &str {
    match self.tabla.get(&nombre_clase) {
      Some(clase) => match clase.metodos.tabla.get(&nombre_func) {
        Some(metodo) => metodo.parametros.buscar_variable(nombre_var),
        None => "Metodo no existente"
      },
      None => "Clase no existente"
    }
  }

  pub fn agregar_atributo(&mut self, nombre_clase: String, nombre_var: String, tipo_var: String) -> &str {
    match self.tabla.get_mut(&nombre_clase) {
      Some(clase) => clase.atributos.agregar_variable(nombre_var, tipo_var),
      None => "Clase no existente"
    }
  }

  pub fn buscar_atributo(&self, nombre_clase: String, nombre_var: String) -> &str {
    match self.tabla.get(&nombre_clase) {
      Some(clase) => clase.atributos.buscar_variable(nombre_var),
      None => "Clase no existente"
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
    assert_eq!(tabla.agregar_clase("Persona".to_string()), "Clase agregada");
    assert_eq!(tabla.agregar_clase("Persona".to_string()), "Nombre de clase ocupado");
    assert_eq!(tabla.buscar_clase("Persona".to_string()), "Clase existente");
    assert_eq!(tabla.buscar_clase("Estudiante".to_string()), "Clase no existente");

    assert_eq!(tabla.agregar_metodo("Persona".to_string(), "func".to_string(), "entero".to_string()), "Funcion agregada");
    assert_eq!(tabla.agregar_metodo("Persona".to_string(), "func".to_string(), "entero".to_string()), "Nombre de funcion ocupado");
    assert_eq!(tabla.buscar_metodo("Persona".to_string(), "func".to_string()), "Funcion existente");
    assert_eq!(tabla.buscar_metodo("Persona".to_string(), "a".to_string()), "Funcion no existente");
    assert_eq!(tabla.buscar_metodo("Estudiante".to_string(), "a".to_string()), "Clase no existente");

    assert_eq!(tabla.agregar_parametro_metodo("Persona".to_string(), "func".to_string(), "variable".to_string(), "entero".to_string()), "Variable agregada");
    assert_eq!(tabla.agregar_parametro_metodo("Persona".to_string(), "func".to_string(), "variable".to_string(), "entero".to_string()), "Nombre de variable ocupado");
    assert_eq!(tabla.agregar_parametro_metodo("Persona".to_string(), "a".to_string(), "variable".to_string(), "entero".to_string()), "Metodo no existente");
    assert_eq!(tabla.agregar_parametro_metodo("Estudiante".to_string(), "a".to_string(), "variable".to_string(), "entero".to_string()), "Clase no existente");
    assert_eq!(tabla.buscar_parametro_metodo("Persona".to_string(), "func".to_string(), "variable".to_string()), "Variable existente");
    assert_eq!(tabla.buscar_parametro_metodo("Persona".to_string(), "func".to_string(), "a".to_string()), "Variable no existente");
    assert_eq!(tabla.buscar_parametro_metodo("Persona".to_string(), "a".to_string(), "variable".to_string()), "Metodo no existente");
    assert_eq!(tabla.buscar_parametro_metodo("Estudiante".to_string(), "a".to_string(), "variable".to_string()), "Clase no existente");

    assert_eq!(tabla.agregar_atributo("Persona".to_string(), "variable".to_string(), "entero".to_string()), "Variable agregada");
    assert_eq!(tabla.agregar_atributo("Persona".to_string(), "variable".to_string(), "entero".to_string()), "Nombre de variable ocupado");
    assert_eq!(tabla.buscar_atributo("Persona".to_string(), "variable".to_string()), "Variable existente");
    assert_eq!(tabla.buscar_atributo("Persona".to_string(), "a".to_string()), "Variable no existente");
    assert_eq!(tabla.buscar_atributo("Estudiante".to_string(), "a".to_string()), "Clase no existente");
  }
}
