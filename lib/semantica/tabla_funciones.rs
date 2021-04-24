use crate::semantica::tabla_variables::*;

#[derive(Debug)]
pub struct TipoFunc {
  nombre: String,
  tipo: String,
  variables: TablaVariables,
}

#[derive(Debug)]
pub struct TablaFunciones {
  pub tabla: Vec<TipoFunc>
}

impl TablaFunciones {
  pub fn agregar_funcion(&mut self, nombre_func: String, tipo_func: String) -> &str {
    let mut var_encontrada: bool = false;

    if self.tabla.len() > 0 {
      for indice in 0..=self.tabla.len() {
        if !var_encontrada && self.tabla[indice].nombre == nombre_func.clone() {
          var_encontrada = true;
        }
      }
    }
  
  
    let mensaje: &str; 
  
    if var_encontrada {
      mensaje = "Nombre de variable ocupado";
    } else {
      self.tabla.push(TipoFunc { 
        nombre: nombre_func.clone(),
        tipo: tipo_func.clone(),
        variables: TablaVariables { tabla: vec![] } 
      });
      mensaje = "Variable agregada";
    }

    return mensaje;
  }

  pub fn modificar_funcion(&mut self, nombre_func: String, tipo_func: String) -> &str {
    let mut var_encontrada: bool = false;
  
    for indice in 0..=self.tabla.len() {
      if self.tabla[indice].nombre == nombre_func.clone() {
        self.tabla[indice] = TipoFunc { 
          nombre: nombre_func.clone(),
          tipo: tipo_func.clone(),
          variables: TablaVariables { tabla: vec![] }
        };
        var_encontrada = true;
      }
    }
  
    let mensaje: &str; 
  
    if var_encontrada {
      mensaje = "Variable no encontrada";
    } else {
      mensaje = "Variable modificada";
    }

    return mensaje;
  }

  pub fn agregar_variable(&mut self, nombre_func: String, nombre_var: String, tipo_var: String, valor_var: String) -> &str {
    for indice in 0..=self.tabla.len() {
      if self.tabla[indice].nombre == nombre_func.clone() {
        return self.tabla[indice].variables.agregar_variable(nombre_var, tipo_var, valor_var);
      }
    }
    ""
  }

  pub fn buscar_variable(&mut self, nombre_func: String, nombre_var: String) -> &str {
    for indice in 0..=self.tabla.len() {
      if self.tabla[indice].nombre == nombre_func.clone() {
        return self.tabla[indice].variables.buscar_variable(nombre_var);
      }
    }
    ""
  }
}

// let mut tabla_variables: Vec<TipoVar>;

// pub fn agregar_variable_a_tabla(nombre: String, tipo: String, valor: String, contexto: String) -> &str {
//   bool var_encontrada = false;

//   for var in tabla_variables {
//     if (!var_encontrada && var.nombre == nombre) {
//       var_encontrada = true;
//     }
//   }

//   let mut mensaje: &str; 

//   if (var_encontrada) {
//     mensaje = "Nombre de variable ocupado";
//   } else {
//     tabla_variables.push(TipoVar { nombre, tipo, valor, contexto});
//     mensaje = "Variable agregada";
//   }
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   // use nom::{
//   //     error::{ErrorKind, VerboseError, VerboseErrorKind},
//   //     Err,
//   // };

//   #[test]
//   fn test_agregar_variable_a_tabla() {
//     assert_eq!(agregar_variable_a_tabla("var", "entero", "1", "global" ), Ok(("", "1")));
//   }
// }
