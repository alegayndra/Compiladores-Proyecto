use crate::semantica::cubo_semantico::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListaCuadruplos {
  pub lista: Vec<(i64, i64, i64, i64)>
}

impl ListaCuadruplos {
  pub fn agregar_cuadruplo<'a>(&mut self, operador: &'a str, izq: &'a str, der: &'a str) -> Result<(&'a str, (&'a str, &'a str, &'a str)), (&'a str, (&'a str, &'a str, &'a str))>{
    let op_num = conseguir_num_operador(operador);
    let izq_num = conseguir_num_tipo(izq);
    let der_num = conseguir_num_tipo(der);

    match checar_cubo_semantico(op_num as usize, izq_num as usize, der_num as usize) {
      3 => Err(("Tipos incompatibles", (operador, izq, der))),
      _ => {
        // Crear temporal
        self.lista.push((op_num, izq_num, der_num, 0));
        Ok(("Tipos compatibles", (operador, izq, der)))
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
    assert_eq!(cuadruplos.agregar_cuadruplo("+", "entero", "entero"),   Ok(("Tipos compatibles", ("+", "entero", "entero"))));
    assert_eq!(cuadruplos.agregar_cuadruplo("+", "entero", "flotante"), Ok(("Tipos compatibles", ("+", "entero", "flotante"))));
    assert_eq!(cuadruplos.agregar_cuadruplo("+", "entero", "char"),     Err(("Tipos incompatibles", ("+", "entero", "char"))));
  }
}
