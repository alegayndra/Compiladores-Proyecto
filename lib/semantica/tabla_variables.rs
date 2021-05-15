use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TipoVar {
  pub nombre: String,
  pub tipo: String
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TablaVariables {
  pub tabla: HashMap<String, TipoVar>
}

impl TablaVariables {
  pub fn agregar_variable(&mut self, nombre_var: String, tipo_var: String) -> &str {
    match self.tabla.contains_key(&nombre_var) {
      true => "Nombre de variable ocupado",
      false => {
        self.tabla.insert(nombre_var.clone(), TipoVar {
          nombre: nombre_var.clone(),
          tipo: tipo_var.clone()
        });
        "Variable agregada"
      }
    }
  }

  pub fn buscar_variable(&mut self, nombre_var: String) -> &str {
    match self.tabla.contains_key(&nombre_var) {
      true => "Variable existente",
      false => "Variable no existe"
    }
  }
}

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
