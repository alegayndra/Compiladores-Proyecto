use crate::semantica::tabla_variables::*;
use crate::semantica::cubo_semantico::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListaCuadruplos {
  pub lista: Vec<(i64, i64, i64, i64)>
}

impl ListaCuadruplos {
  pub fn agregar_cuadruplo<'a>(&mut self, operador: &'a str, izq: TipoVar, der: TipoVar) -> Result<(&'a str, (&'a str, String, String)), (&'a str, (&'a str, String, String))>{
    let op_num = conseguir_num_operador(operador);
    let izq_num = conseguir_num_tipo(izq.tipo.as_str());
    let der_num = conseguir_num_tipo(der.tipo.as_str());

    match checar_cubo_semantico(op_num as usize, izq_num as usize, der_num as usize) {
      3 => Err(("Tipos incompatibles", (operador, izq.tipo, der.tipo))),
      _ => {
        // Crear temporal
        self.lista.push((op_num, izq.direccion, der.direccion, 0));
        Ok(("Tipos compatibles", (operador, izq.tipo, der.tipo)))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_checar() {
    let mut cuadruplos = ListaCuadruplos { lista: vec![] };
    let var_entera = TipoVar {
      nombre: "a".to_owned(),
      direccion: 1000,
      tipo: "entero".to_owned(),
      dimensiones: vec![]
    };

    let var_flotante = TipoVar {
      nombre: "a".to_owned(),
      direccion: 2000,
      tipo: "flotante".to_owned(),
      dimensiones: vec![]
    };

    let var_char = TipoVar {
      nombre: "a".to_owned(),
      direccion: 3000,
      tipo: "char".to_owned(),
      dimensiones: vec![]
    };

    assert_eq!(cuadruplos.agregar_cuadruplo("+", var_entera.clone(), var_entera.clone()),   Ok(("Tipos compatibles", ("+", "entero".to_owned(), "entero".to_owned()))));
    assert_eq!(cuadruplos.agregar_cuadruplo("+", var_entera.clone(), var_flotante.clone()), Ok(("Tipos compatibles", ("+", "entero".to_owned(), "flotante".to_owned()))));
    assert_eq!(cuadruplos.agregar_cuadruplo("+", var_entera.clone(), var_char.clone()),     Err(("Tipos incompatibles", ("+", "entero".to_owned(), "char".to_owned()))));
  }
}
