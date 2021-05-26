use crate::semantica::tabla_variables::*;
use crate::semantica::cubo_semantico::*;
use crate::semantica::globales::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListaCuadruplos {
  pub lista: Vec<(i64, i64, i64, i64)>
}

static mut NUM_TEMPORAL: i64 = 0;

impl ListaCuadruplos {
  pub fn agregar_cuadruplo<'a>(&mut self, operador: &'a str, izq: TipoVar, der: TipoVar) -> Result<(&'a str, (&'a str, String, String)), (&'a str, (&'a str, String, String))>{
    let op_num = conseguir_num_operador(operador);
    let izq_num = conseguir_num_tipo(izq.tipo.as_str());
    let der_num = conseguir_num_tipo(der.tipo.as_str());

    match checar_cubo_semantico(op_num as usize, izq_num as usize, der_num as usize) {
      3 => Err(("Tipos incompatibles", (operador, izq.tipo, der.tipo))),
      n => {
        // Crear temporal
        let mut tabla_variables = VARIABLES.lock().unwrap();
        let tipo_temporal = conseguir_tipo_num(n);
        unsafe {
          loop {
            let nombre_temporal = format!("temporal{}", NUM_TEMPORAL);
            match tabla_variables.agregar_variable(nombre_temporal.clone(), tipo_temporal.clone(), vec![], 1000) {
              Ok(_) => {
                println!("Temporal agregado: {:?}", nombre_temporal);
                break;
              },
              Err(_) => {
                NUM_TEMPORAL += 1;
                ()
              }
            }
          }
        }
        self.lista.push((op_num, izq.direccion, der.direccion, 0));
        Ok(("Tipos compatibles", (operador, izq.tipo, der.tipo)))
      }
    }
  }

  pub fn agregar_cuadruplo_asignacion<'a>(&mut self, valor: TipoVar, destino: TipoVar) -> Result<(&'a str, (String, String)), (&'a str, (String, String))>{
    let op_num = conseguir_num_operador("=");
    let valor_num = conseguir_num_tipo(valor.tipo.as_str());
    let destino_num = conseguir_num_tipo(destino.tipo.as_str());

    match checar_cubo_semantico(op_num as usize, valor_num as usize, destino_num as usize) {
      3 => Err(("Asignacion incompatible", (valor.tipo, destino.tipo))),
      _ => {
        // Crear temporal
        self.lista.push((op_num, valor.direccion, -1, destino.direccion));
        Ok(("Asignacion compatible", (valor.tipo, destino.tipo)))
      }
    }
  }

  pub fn agregar_cuadruplo_escritura<'a>(&mut self, valor: TipoVar) -> Result<(&'a str, String), (&'a str, String)>{
    let op_num = conseguir_num_operador("ESCRIBE");
    self.lista.push((op_num, -1, -1, valor.direccion));
    Ok(("Print bueno", valor.tipo))
  }

  pub fn agregar_cuadruplo_lectura<'a>(&mut self, valor: TipoVar) -> Result<(&'a str, String), (&'a str, String)>{
    let op_num = conseguir_num_operador("LEE");
    self.lista.push((op_num, -1, -1, valor.direccion));
    Ok(("Read bueno", valor.tipo))
  }

  pub fn agregar_cuadruplo_goto<'a>(&mut self) -> Result<&'a str, &'a str>{
    let op_num = conseguir_num_operador("GOTO");
    self.lista.push((op_num, -1, -1, -1));
    Ok("Goto bueno")
  }

  pub fn modificar_cuadruplo_goto<'a>(&mut self, num_cuadruplo: usize) -> Result<(&'a str, usize, i64), (&'a str, usize, i64)>{
    let direccion_cuadruplo = (self.lista.len()) as i64;
    self.lista[num_cuadruplo].3 = direccion_cuadruplo;
    Ok(("Goto modificado", num_cuadruplo, direccion_cuadruplo))
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
