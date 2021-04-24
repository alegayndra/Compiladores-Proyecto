#[derive(Debug)]
pub struct TipoVar {
  nombre: String,
  tipo: String,
  valor: String
}

#[derive(Debug)]
pub struct TablaVariables {
  tabla: Vec<TipoVar>
}

impl TablaVariables {
  pub fn agregar_variable(&mut self, nombre_var: String, tipo_var: String, valor_var: String, contexto_var: String) -> &str {
    let mut var_encontrada: bool = false;
  
    for indice in 0..=self.tabla.len() {
      if !var_encontrada && self.tabla[indice].nombre == nombre_var.clone() {
        var_encontrada = true;
      }
    }
  
    let mensaje: &str; 
  
    if var_encontrada {
      mensaje = "Nombre de variable ocupado";
    } else {
      self.tabla.push(TipoVar { 
        nombre: nombre_var.clone(),
        tipo: tipo_var.clone(),
        valor: valor_var.clone(),
      });
      mensaje = "Variable agregada";
    }

    return mensaje;
  }

  pub fn modificar_variable(&mut self, nombre_var: String, tipo_var: String, valor_var: String) -> &str {
    let mut var_encontrada: bool = false;
  
    for indice in 0..=self.tabla.len() {
      if self.tabla[indice].nombre == nombre_var.clone() {
        self.tabla[indice] = TipoVar { 
          nombre: nombre_var.clone(),
          tipo: tipo_var.clone(),
          valor: valor_var.clone(),
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

  pub fn buscar_variable(&mut self, nombre_var: String) -> &str {
    for indice in 0..=self.tabla.len() {
      if self.tabla[indice].nombre == nombre_var.clone() {
        return "Var encontrada";
      }
    }

    return "Var no encontrada";
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
