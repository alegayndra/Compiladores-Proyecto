//! Módulo que se encarga de la semántica de las variables.

use std::collections::HashMap;
use crate::semantica::globales::*;
use crate::semantica::cubo_semantico::*;

/// Tipo de variables.  
/// Sirve para guardar toda la información de una variable.  
///
/// # Atributos
///
/// * `nombre` - ID de la variable
/// * `direccion` - Dirección de memoria de la variable
/// * `tipo` - Tipo de la variable
/// * `dimensiones` - Arreglo de las dimensiones de la variable
///
/// # Ejemplo de creación
/// ```
/// let variable: TipoVar = TipoVar {
///   nombre: "nombre".to_owned(),
///   direccion: 200,
///   tipo: "entero".to_owned(),
///   dimensiones: vec![],
/// };
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TipoVar {
  pub nombre: String,
  pub direccion: i64,
  pub tipo: String,
  pub dimensiones: Vec<i64>
}

/// Tabla de variables.  
/// Sirve para guardar todas las variables.  
///
/// # Atributos
///
/// * `tabla` - HashMap de variables
///
/// # Ejemplo de creación
/// ```
/// let tabla_variables: TablaVariables = TablaVariables {
///   tabla: HashMap::new()
/// };
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TablaVariables {
  pub tabla: HashMap<String, TipoVar>
}

/// Diferentes métodos implementados para el acceso y modificación de la tabla de variables.
impl TablaVariables {
  /// Función auxiliar para agregar una variable a la tabla de variables.  
  /// Regresa un Result, con letreros y la variable que se acaba de agregar.
  ///
  /// # Parametros
  ///
  /// * `nombre_var` - ID de la variable
  /// * `tipo_var` - Tipo de la variable
  /// * `dims` - Arreglo de las dimensiones de la variable
  /// * `dir` - Dirección de memoria de la variable
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let tabla_variables: TablaVariables = TablaVariables { tabla: HashMap::new() };
  /// 
  /// match tabla_variables.agregar_variable("nombre".to_owned(), "entero".to_owned(), vec![], 100) { 
  ///   Ok((_, variable)) => variable, // Se agregó la variable éxitosamente
  ///   Err(err) => err, // Error al agregar la variable
  /// };
  /// ```
  pub fn agregar_variable(&mut self, nombre_var: String, tipo_var: String, dims: Vec<i64>, dir: i64) -> Result<(&str, TipoVar), (&str, String)> {
    // Busca que no exista la variable
    match self.tabla.contains_key(&nombre_var) {
      true => Err(("Nombre de variable ocupado", nombre_var.clone())),
      false => {
        // Crea la variable y la inserta en el hashmap
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

  /// Función auxiliar para buscar una variable dentro de la tabla de variables.  
  /// Regresa un Result, con letreros y la variable que se buscó.
  ///
  /// # Parametros
  ///
  /// * `nombre_var` - ID de la variable
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let tabla_variables: TablaVariables = TablaVariables { tabla: HashMap::new() };
  /// 
  /// match tabla_variables.buscar_variable("nombre".to_owned()) { 
  ///   Ok((_, variable)) => variable, // Se encontró la variable
  ///   Err(err) => err, // Error al buscar la variable
  /// };
  /// ```
  pub fn buscar_variable(&self, nombre_var: String) -> Result<(&str, TipoVar), (&str, String)> {
    match self.tabla.get(&nombre_var) {
      Some(var) => Ok(("Variable existente", var.clone())),
      None => Err(("Variable no existente", nombre_var.clone()))
    }
  }
  
  /// Función auxiliar para agregar una constantes.  
  /// Regresa la constante.
  ///
  /// # Parametros
  ///
  /// * `nombre_var` - ID de la consante
  /// * `tipo_var` - Tipo de la consante
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let tabla_variables: TablaVariables = TablaVariables { tabla: HashMap::new() };
  /// 
  /// let constante = tabla_variables.agregar_constante("10".to_owned(), "entero".to_owned());
  /// ```
  pub fn agregar_constante(&mut self, nombre_var: String, tipo_var: String) -> TipoVar {
    // Busca que la constante que se quiere agregar ya exista
    // En ese caso, se regresa la constante
    match self.tabla.get(&nombre_var) {
      Some(var) => var.clone(),
      None => {
        // Consigue la dirección de memoria de la constante
        let dir = match conseguir_direccion(tipo_var.clone().as_str(), "constante", 0, vec![]) {
          Ok(num) => num,
          Err(err) => { println!("{:?}", err); -1}
        };

        // Crea la constante
        let var = TipoVar {
          nombre: nombre_var.clone(),
          tipo: tipo_var.clone(),
          dimensiones: vec![],
          direccion: dir
        };

        // Modifica el era de las constantes
        unsafe {
          match conseguir_num_tipo(tipo_var.as_str()) {
            0 => ERA_CONSTANTES.0 += 1,
            1 => ERA_CONSTANTES.1 += 1,
            2 => ERA_CONSTANTES.2 += 1,
            5 => ERA_CONSTANTES.2 += 1,
            _ => (),
          }
        }

        // Inserta la constante en el hashmap
        self.tabla.insert(nombre_var.clone(), var.clone());
        var
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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

