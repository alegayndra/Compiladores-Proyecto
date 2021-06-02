//! Módulo que se encarga de la semántica de las funciones.

use std::collections::HashMap;
use crate::semantica::tabla_variables::*;
use crate::semantica::cubo_semantico::*;

/// Tipo de funciones.  
/// Sirve para guardar toda la información de una función.  
///
/// # Atributos
///
/// * `nombre` - ID de la función
/// * `tipo` - Tipo de la función
/// * `variables` - Tabla de variables de la función
/// * `direccion` - Dirección de memoria de la función
/// * `num_cuadruplo` - Número de cuadruplo donde empieza la función
/// * `parametros` - Vector de los diferentes parametros en ordén
/// * `era` - Tamaño de la función
///
/// # Ejemplo de creación
///
/// ```ignore
/// let función: TipoFunc = TipoFunc {
///   nombre: "func".to_owned(),
///   tipo: "void".to_owned(),
///   variables: vec![],
///   direccion: -8,
///   num_cuadruplo: 5,
///   parametros: vec![], // Para ver como crear un `TipoVar`, ir al archivo de `tabla_variables.rs`
///   era: vec![
///     // Normales y temporales respectivamente
///     (0, 0), // Variables enteras
///     (0, 0), // Variables flotantes
///     (0, 0), // Variables caracteres
///   ],
/// };
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TipoFunc {
  pub nombre: String,
  pub tipo: String,
  pub variables: TablaVariables,
  pub direccion: i64,
  pub num_cuadruplo: i64,
  pub parametros: Vec<TipoVar>,
  pub era: Vec<(i64, i64)>
}

/// Tabla de funciones.  
/// Sirve para guardar todas las funciones.  
///
/// # Atributos
///
/// * `tabla` - HashMap de funciones
///
/// # Ejemplo de creación
///
/// ```ignore
/// let tabla_funciones: TablaFunciones = TablaFunciones {
///   tabla: HashMap::new()
/// };
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TablaFunciones {
  pub tabla: HashMap<String, TipoFunc>
}

/// Diferentes métodos implementados para el acceso y modificación de una función en particular.
impl TipoFunc {
  /// Función auxiliar para modificar el era de una función.  
  ///
  /// # Parametros
  ///
  /// * `tipo_var` - Tipo de la variable
  /// * `temporal` - Número que marca si es un temporal o no
  ///
  /// # Ejemplo
  ///
  /// ```ignore
  /// let funcion: TipoFunc = TipoFunc { /* Atributos */ };
  /// 
  /// match funcion.modificar_era("entero".to_owned(), 1); // Se está agregando un temporal entero
  /// ```
  pub fn modificar_era(&mut self, tipo_var: String, temporal: i8) {
    let posicion = conseguir_num_tipo(tipo_var.as_str());
    match temporal {
      1 => {
        self.era[posicion as usize].1 += 1;
      },
      _ => {
        self.era[posicion as usize].0 += 1;
      }
    };
  }
}

/// Diferentes métodos implementados para el acceso y modificación de la tabla de funciones.
impl TablaFunciones {
  /// Función auxiliar para agregar una variable a la tabla de variables.  
  /// Regresa un Result, con letreros y la variable que se acaba de agregar.
  ///
  /// # Parametros
  ///
  /// * `nombre_func` - ID de la variable
  /// * `tipo_func` - Tipo de la variable
  /// * `dims` - Arreglo de las dimensiones de la variable
  /// * `dir` - Dirección de memoria de la variable
  ///
  /// # Ejemplo
  ///
  /// ```ignore
  /// let tabla_funciones: TablaFunciones = TablaFunciones { tabla: HashMap::new() };
  /// 
  /// match tabla_funciones.agregar_variable("func".to_owned(), "void".to_owned(), -8, 5) { 
  ///   Ok((_, funcion)) => funcion, // Se agregó la función éxitosamente
  ///   Err(err) => err, // Error al agregar la función
  /// };
  /// ```
  pub fn agregar_funcion(&mut self, nombre_func: String, tipo_func: String, dir: i64, cuad: i64) -> Result<(&str, TipoFunc), (&str, String)> {
    // Busca que no exista la función
    match self.tabla.contains_key(&nombre_func) {
      true => Err(("Nombre de funcion ocupado", nombre_func.clone())),
      false =>  {
        // Crea la función y la inserta en el hashmap
        let func = TipoFunc { 
          nombre: nombre_func.clone(),
          tipo: tipo_func.clone(),
          variables: TablaVariables { tabla: HashMap::new() },
          parametros: vec![],
          direccion: dir,
          num_cuadruplo: cuad,
          era: vec![
            (0, 0),
            (0, 0),
            (0, 0)
          ]
        };
        self.tabla.insert(nombre_func.clone(), func.clone());
        Ok(("Funcion agregada", func.clone()))
      }
    }
  }

  /// Función auxiliar para buscar una función dentro de la tabla de funciones.  
  /// Regresa un Result, con letreros y la variable que se buscó.
  ///
  /// # Parametros
  ///
  /// * `nombre_func` - ID de la función
  ///
  /// # Ejemplo
  ///
  /// ```ignore
  /// let tabla_funciones: TablaFunciones = TablaFunciones { tabla: HashMap::new() };
  /// 
  /// match tabla_funciones.buscar_funcion("func".to_owned()) { 
  ///   Ok((_, funcion)) => funcion, // Se encontró la función
  ///   Err(err) => err, // Error al buscar la función
  /// };
  /// ```
  pub fn buscar_funcion(&self, nombre_func: String) -> Result<(&str, TipoFunc), (&str, String)> {
    match self.tabla.get(&nombre_func) {
      Some(func) => Ok(("Funcion existente", func.clone())),
      None => Err(("Funcion no existente", nombre_func.clone()))
    }
  }

  /// Función auxiliar para agregar una variable una función dentro de la tabla de funciones.  
  /// Regresa un Result, con letreros y la variable que se acaba de agregar.
  ///
  /// # Parametros
  ///
  /// * `nombre_func` - ID de la función
  /// * `nombre_var` - ID de la variable
  /// * `tipo_var` - Tipo de la variable
  /// * `dims` - Arreglo de las dimensiones de la variable
  /// * `dir` - Dirección de memoria de la variable
  ///
  /// # Ejemplo
  ///
  /// ```ignore
  /// let tabla_funciones: TablaFunciones = TablaFunciones { tabla: HashMap::new() };
  /// 
  /// match tabla_funciones.agregar_variable("func".to_owned(), "nombre".to_owned(), "entero".to_owned(), vec![], 100) { 
  ///   Ok((_, _, variable)) => variable, // Se agregó la variable éxitosamente
  ///   Err(err) => err, // Error al agregar la variable
  /// };
  /// ```
  pub fn agregar_variable(&mut self, nombre_func: String, nombre_var: String, tipo_var: String, dims: Vec<i64>, dir: i64, temporal: i8) -> Result<(&str, String, TipoVar), (&str, String, String)> {
    // Busca la función y luego agrega la variable
    match self.tabla.get_mut(&nombre_func) {  
      Some(funcion) => match funcion.variables.agregar_variable(nombre_var.clone(), tipo_var.clone(), dims, dir) {
        Ok((_, var)) => {
          funcion.modificar_era(tipo_var.clone(), temporal);
          Ok(("Variable agregada a funcion", nombre_func.clone(), var))
        },
        Err((_, nombre_var)) => Err(("Nombre de variable ocupado en funcion", nombre_func.clone(), nombre_var))
      },
      None => Err(("Funcion no existente", nombre_func.clone(), "".to_owned()))
    }
  }

  /// Función auxiliar para buscar una variable dentro de una función dentro de la tabla de funciones.  
  /// Regresa un Result, con letreros y la variable que se buscó.
  ///
  /// # Parametros
  ///
  /// * `nombre_func` - ID de la función
  /// * `nombre_var` - ID de la variable
  ///
  /// # Ejemplo
  ///
  /// ```ignore
  /// let tabla_variables: TablaVariables = TablaVariables { tabla: HashMap::new() };
  /// 
  /// match tabla_variables.buscar_variable("func".to_owned(), "nombre".to_owned()) { 
  ///   Ok((_, _,, variable)) => variable, // Se encontró la variable
  ///   Err(err) => err, // Error al buscar la variable
  /// };
  /// ```
  pub fn buscar_variable(&self, nombre_func: String, nombre_var: String) -> Result<(&str, String, TipoVar), (&str, String, String)> {
    // Busca la función y luego la variable
    match self.tabla.get(&nombre_func) {
      Some(funcion) => match funcion.variables.buscar_variable(nombre_var.clone()) {
        Ok((_, var)) => Ok(("Variable existente en funcion", nombre_func.clone(), var.clone())),
        Err((_, nombre_var)) => Err(("Variable no existente en funcion", nombre_func.clone(), nombre_var))
      },
      None => Err(("Funcion no existente", nombre_func.clone(), "".to_owned()))
    }
  }

  /// Función auxiliar para agregar un parámetro una función dentro de la tabla de funciones.  
  /// Regresa un Result, con letreros y la variable que se acaba de agregar.
  ///
  /// # Parametros
  ///
  /// * `nombre_func` - ID del función
  /// * `nombre_var` - ID del parámetro
  /// * `tipo_var` - Tipo del parámetro
  /// * `dims` - Arreglo de las dimensiones del parámetro
  /// * `dir` - Dirección de memoria del parámetro
  ///
  /// # Ejemplo
  ///
  /// ```ignore
  /// let tabla_funciones: TablaFunciones = TablaFunciones { tabla: HashMap::new() };
  /// 
  /// match tabla_funciones.agregar_parametro("func".to_owned(), "nombre".to_owned(), "entero".to_owned(), vec![], 100) { 
  ///   Ok((_, _, variable)) => variable, // Se agregó la variable éxitosamente
  ///   Err(err) => err, // Error al agregar la variable
  /// };
  /// ```
  pub fn agregar_parametro(&mut self, nombre_func: String, nombre_var: String, tipo_var: String, dir: i64) -> Result<(&str, String, TipoVar), (&str, String, String)> {
    // Busca la función y luego agrega el parámetro
    match self.tabla.get_mut(&nombre_func) {
      Some(funcion) => {
        match funcion.variables.agregar_variable(nombre_var.clone(), tipo_var.clone(), vec![], dir) {
          Ok((_, var)) => {
            // Agrega el parámetro a la lista de parametros al igual que a la tabla de variables
            funcion.parametros.push(var.clone());
            funcion.modificar_era(tipo_var.clone(), 0);
            Ok(("Parametro agregado a funcion", nombre_func.clone(), var))
          },
          Err((_, nom_var)) => Err(("Nombre de variable ocupado en funcion", nombre_func.clone(), nom_var))
        }
      },
      None => Err(("Funcion no existente", nombre_func.clone(), "".to_owned()))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tabla_funciones() {
    let mut tabla : TablaFunciones = TablaFunciones { tabla: HashMap::new() };
    let dims = vec![];

    let dir_func = 14000;
    let cuad_func = 3;
    let func_entera = TipoFunc {
      nombre: "func".to_owned(),
      tipo: "entero".to_owned(),
      variables: TablaVariables { tabla: HashMap::new() },
      parametros: vec![],
      direccion: 14000,
      num_cuadruplo: 3,
      era: vec![
        (0, 0),
        (0, 0),
        (0, 0)
      ]
    };

    let dir_var = 1000;
    let var_entera = TipoVar {
      nombre: "variable".to_owned(),
      tipo: "entero".to_owned(),
      dimensiones: vec![],
      direccion: 1000
    };

    assert_eq!(
      tabla.agregar_funcion("func".to_owned(), "entero".to_owned(), dir_func, cuad_func), 
      Ok(("Funcion agregada", func_entera.clone()))
    );
    assert_eq!(
      tabla.agregar_funcion("func".to_owned(), "entero".to_owned(), dir_func, cuad_func),
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
      tabla.agregar_variable("func".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var, 0), 
      Ok(("Variable agregada a funcion", "func".to_owned(), var_entera.clone()))
    );
    assert_eq!(
      tabla.agregar_variable("func".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var, 0),
      Err(("Nombre de variable ocupado en funcion", "func".to_owned(), "variable".to_owned()))
    );
    assert_eq!(
      tabla.agregar_variable("a".to_owned(), "variable".to_owned(), "entero".to_owned(), dims.clone(), dir_var, 0),
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
