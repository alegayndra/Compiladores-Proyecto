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
  pub fn agregar_clase(&mut self, nombre_clase: String, nombre_padre: String) -> Result<(&str, TipoClase), (&str, String)> {
    match self.tabla.contains_key(&nombre_clase) {
      true => Err(("Nombre de clase ocupado", nombre_clase.clone())),
      false =>  {
        if nombre_padre.clone() != "".to_owned() {
          match self.tabla.contains_key(&nombre_padre) {
            false => return Err(("Padre no existe", nombre_padre.clone())),
            true => (),
          }
        }
        let clase = TipoClase { 
          nombre: nombre_clase.clone(),
          padre: nombre_padre.clone(),
          atributos: TablaVariables { tabla: HashMap::new() },
          metodos: TablaFunciones { tabla: HashMap::new() } 
        };
        self.tabla.insert(nombre_clase.clone(), clase.clone());
        Ok(("Clase agregada", clase.clone()))
      }
    }
  }

  pub fn buscar_clase(&self, nombre_clase: String) -> Result<(&str, TipoClase), (&str, String)> {
    match self.tabla.get(&nombre_clase) {
      Some(clase) => Ok(("Clase existente", clase.clone())),
      None => Err(("Clase no existente", nombre_clase.clone()))
    }
  }

  pub fn agregar_metodo(&mut self, nombre_clase: String, nombre_func: String, tipo_func: String, dir: i64) -> Result<(&str, String, TipoFunc), (&str, String, String)> {
    match self.tabla.get_mut(&nombre_clase) {
      Some(clase) => match clase.metodos.agregar_funcion(nombre_func, tipo_func, dir) {
        Ok((_, func)) => Ok(("Metodo agregado a clase", nombre_clase.clone(), func)),
        Err((_, nom_func)) => Err(("Nombre de metodo ocupado en clase", nombre_clase.clone(), nom_func))
      },
      None => Err(("Clase no existente", nombre_clase.clone(), "".to_owned()))
    }
  }

  pub fn buscar_metodo(&self, nombre_clase: String, nombre_func: String) -> Result<(&str, String, TipoFunc), (&str, String, String)> {
    match self.tabla.get(&nombre_clase) {
      Some(clase) => match clase.metodos.buscar_funcion(nombre_func) {
        Ok((_, func)) => Ok(("Metodo existente en clase", nombre_clase.clone(), func)),
        Err((_, nom_func)) => Err(("Metodo no existente en clase", nombre_clase.clone(), nom_func))
      },
      None => Err(("Clase no existente", nombre_clase.clone(), "".to_owned()))
    }
  }

  pub fn agregar_variable_metodo(
    &mut self,
    nombre_clase: String,
    nombre_func: String,
    nombre_var: String,
    tipo_var: String,
    dims: Vec<String>,
    dir: i64
  ) -> Result<(&str, String, String, TipoVar), (&str, String, String, String)> {
    match self.tabla.get_mut(&nombre_clase) {
      Some(clase) => match clase.metodos.tabla.get_mut(&nombre_func) {
        Some(metodo) => match metodo.variables.agregar_variable(nombre_var.clone(), tipo_var.clone(), dims.clone(), dir) {
          Ok((_, var)) => Ok(("Variable agregada a metodo", nombre_clase.clone(), nombre_func.clone(), var)),
          Err((_, nom_var)) => Err(("Nombre de variable ocupado en metodo", nombre_clase.clone(), nombre_func.clone(), nom_var))
        },
        None => Err(("Metodo no existente en clase", nombre_clase.clone(), nombre_func.clone(), "".to_owned()))
      },
      None => Err(("Clase no existente", nombre_clase.clone(), "".to_owned(), "".to_owned()))
    }
  }

  pub fn buscar_variable_metodo(
    &self,
    nombre_clase: String,
    nombre_func: String,
    nombre_var: String
  ) -> Result<(&str, String, String, TipoVar), (&str, String, String, String)> {
    match self.tabla.get(&nombre_clase) {
      Some(clase) => match clase.metodos.tabla.get(&nombre_func) {
        Some(metodo) => match metodo.variables.buscar_variable(nombre_var) {
          Ok((_, var)) => Ok(("Variable existente en metodo", nombre_clase.clone(), nombre_func.clone(), var)),
          Err((_, nombre_var)) => Err(("Variable no existente en metodo", nombre_clase.clone(), nombre_func.clone(), nombre_var))
        },
        None => Err(("Metodo no existente en clase", nombre_clase.clone(), nombre_func.clone(), "".to_owned()))
      },
      None => Err(("Clase no existente", nombre_clase.clone(), "".to_owned(), "".to_owned()))
    }
  }

  pub fn agregar_parametro_metodo(
    &mut self,
    nombre_clase: String,
    nombre_func: String,
    nombre_var: String,
    tipo_var: String,
    dims: Vec<String>,
    dir: i64
  ) -> Result<(&str, String, String, TipoVar), (&str, String, String, String)> {
    match self.tabla.get_mut(&nombre_clase) {
      Some(clase) => match clase.metodos.tabla.get_mut(&nombre_func) {
        Some(metodo) => match metodo.variables.agregar_variable(nombre_var.clone(), tipo_var.clone(), dims.clone(), dir) {
          Ok((_, var)) => {
            metodo.parametros.push(var.clone());
            Ok(("Parametro agregado a metodo", nombre_clase.clone(), nombre_func.clone(), var))
          },
          Err((_, nom_var)) => Err(("Nombre de variable ocupado en metodo", nombre_clase.clone(), nombre_func.clone(), nom_var))
        },
        None => Err(("Metodo no existente en clase", nombre_clase.clone(), nombre_func.clone(), "".to_owned()))
      },
      None => Err(("Clase no existente", nombre_clase.clone(), "".to_owned(), "".to_owned()))
    }
  }

  // pub fn buscar_parametro_metodo(&self, nombre_clase: String, nombre_func: String, nombre_var: String) -> Result<(&str, String), (&str, String)> {
  //   match self.tabla.get(&nombre_clase) {
  //     Some(clase) => match clase.metodos.tabla.get(&nombre_func) {
  //       Some(metodo) => metodo.parametros.hash.buscar_variable(nombre_var),
  //       None => Err(("Metodo no existente", nombre_func.clone()))
  //     },
  //     None => Err(("Clase no existente", nombre_clase.clone()))
  //   }
  // }

  pub fn agregar_atributo(
    &mut self,
    nombre_clase: String,
    nombre_var: String,
    tipo_var: String,
    dims: Vec<String>,
    dir: i64
  ) -> Result<(&str, String, TipoVar), (&str, String, String)> {
    match self.tabla.get_mut(&nombre_clase) {
      Some(clase) => match clase.atributos.agregar_variable(nombre_var, tipo_var, dims, dir) {
        Ok((_, var)) => Ok(("Atributo agregado a clase", nombre_clase.clone(), var)),
        Err((_, nom_var)) => Err(("Nombre de atributo ocupado en clase", nombre_clase.clone(), nom_var))
      },
      None => Err(("Clase no existente", nombre_clase.clone(), "".to_owned()))
    }
  }

  pub fn buscar_atributo(&self, nombre_clase: String, nombre_var: String) -> Result<(&str, String, TipoVar), (&str, String, String)> {
    match self.tabla.get(&nombre_clase) {
      Some(clase) => match clase.atributos.buscar_variable(nombre_var) {
        Ok((_, var)) => Ok(("Atributo existente en clase", nombre_clase.clone(), var)),
        Err((_, nom_var)) => Err(("Atributo no existente en clase", nombre_clase.clone(), nom_var))
      },
      None => Err(("Clase no existente", nombre_clase.clone(), "".to_owned()))
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
    let dims : Vec<String> = vec![];

    // let dir_clase = 20000;
    let clase = TipoClase {
      nombre: "Persona".to_owned(),
      padre: "".to_owned(),
      atributos: TablaVariables { tabla: HashMap::new() },
      metodos: TablaFunciones { tabla: HashMap::new() } 
    };

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
      tabla.agregar_clase("Persona".to_owned(), "".to_owned()),
      Ok(("Clase agregada", clase.clone()))
    );
    assert_eq!(
      tabla.agregar_clase("Persona".to_owned(), "".to_owned()),
      Err(("Nombre de clase ocupado", "Persona".to_owned()))
    );
    assert_eq!(
      tabla.buscar_clase("Persona".to_owned()),
      Ok(("Clase existente", clase.clone()))
    );
    assert_eq!(
      tabla.buscar_clase("Estudiante".to_owned()),
      Err(("Clase no existente", "Estudiante".to_owned()))
    );

    assert_eq!(
      tabla.agregar_metodo("Persona".to_owned(), "func".to_owned(), "entero".to_owned(), dir_func),
      Ok(("Metodo agregado a clase", "Persona".to_owned(), func_entera.clone()))
    );
    assert_eq!(
      tabla.agregar_metodo("Persona".to_owned(), "func".to_owned(), "entero".to_owned(), dir_func),
      Err(("Nombre de metodo ocupado en clase", "Persona".to_owned(), "func".to_owned()))
    );
    assert_eq!(
      tabla.buscar_metodo("Persona".to_owned(), "func".to_owned()),
      Ok(("Metodo existente en clase", "Persona".to_owned(), func_entera.clone()))
    );
    assert_eq!(
      tabla.buscar_metodo("Persona".to_owned(), "a".to_owned()),
      Err(("Metodo no existente en clase", "Persona".to_owned(), "a".to_owned()))
    );
    assert_eq!(
      tabla.buscar_metodo("Estudiante".to_owned(), "a".to_owned()),
      Err(("Clase no existente", "Estudiante".to_owned(), "".to_owned()))
    );

    assert_eq!(
      tabla.agregar_parametro_metodo("Persona".to_owned(), "func".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var),
      Ok(("Parametro agregado a metodo", "Persona".to_owned(), "func".to_owned(), var_entera.clone()))
    );
    assert_eq!(
      tabla.agregar_parametro_metodo("Persona".to_owned(), "func".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var),
      Err(("Nombre de variable ocupado en metodo", "Persona".to_owned(), "func".to_owned(), "variable".to_owned()))
    );
    assert_eq!(
      tabla.agregar_parametro_metodo("Persona".to_owned(), "a".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var),
      Err(("Metodo no existente en clase", "Persona".to_owned(), "a".to_owned(), "".to_owned()))
    );
    assert_eq!(
      tabla.agregar_parametro_metodo("Estudiante".to_owned(), "a".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var),
      Err(("Clase no existente", "Estudiante".to_owned(), "".to_owned(), "".to_owned()))
    );
    // assert_eq!(
    //   tabla.buscar_parametro_metodo("Persona".to_owned(), "func".to_owned(), "variable".to_owned()),
    //   Ok(("Variable existente", "variable".to_owned()))
    // );
    // assert_eq!(
    //   tabla.buscar_parametro_metodo("Persona".to_owned(), "func".to_owned(), "a".to_owned()),
    //   Err(("Variable no existente", "a".to_owned()))
    // );
    // assert_eq!(
    //   tabla.buscar_parametro_metodo("Persona".to_owned(), "a".to_owned(), "variable".to_owned()),
    //    Err(("Metodo no existente", "a".to_owned()))
    //   );
    // assert_eq!(
    //   tabla.buscar_parametro_metodo("Estudiante".to_owned(), "a".to_owned(), "variable".to_owned()),
    //   Err(("Clase no existente", "Estudiante".to_owned()))
    // );

    assert_eq!(
      tabla.agregar_atributo("Persona".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var),
      Ok(("Atributo agregado a clase", "Persona".to_owned(), var_entera.clone()))
    );
    assert_eq!(
      tabla.agregar_atributo("Persona".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var),
      Err(("Nombre de atributo ocupado en clase", "Persona".to_owned(), "variable".to_owned()))
    );
    assert_eq!(
      tabla.buscar_atributo("Persona".to_owned(), "variable".to_owned()),
      Ok(("Atributo existente en clase", "Persona".to_owned(), var_entera.clone()))
    );
    assert_eq!(
      tabla.buscar_atributo("Persona".to_owned(), "a".to_owned()),
      Err(("Atributo no existente en clase", "Persona".to_owned(), "a".to_owned()))
    );
    assert_eq!(
      tabla.buscar_atributo("Estudiante".to_owned(), "a".to_owned()),
      Err(("Clase no existente", "Estudiante".to_owned(), "".to_owned()))
    );
  }
}
