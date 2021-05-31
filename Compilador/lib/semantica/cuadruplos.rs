use crate::semantica::tabla_variables::*;
use crate::semantica::cubo_semantico::*;
use crate::semantica::globales::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListaCuadruplos {
  pub lista: Vec<(i64, i64, i64, i64)>
}

static mut NUM_TEMPORAL: i64 = 0;

fn agregar_temporal_a_tabla(var: String, tipo_var: String, dir: i64) {
  let contexto_clase = CONTEXTO_CLASE.lock().unwrap();
  let contexto_funcion = CONTEXTO_FUNCION.lock().unwrap();

  if contexto_clase.clone() != "".to_owned() {
    if contexto_funcion.clone() != "".to_owned() {
      match CLASES.lock().unwrap().agregar_atributo(contexto_clase.to_string(), var, tipo_var, vec![], dir) {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        }
      }
    } else {
      match CLASES.lock().unwrap().agregar_variable_metodo(contexto_clase.to_string(), contexto_funcion.to_string(), var, tipo_var, vec![], dir, 1) {
        Ok(_) => (),
        Err(err) => {
          println!("{:?}", err);
        }
      }
    }
  } else {
    match FUNCIONES.lock().unwrap().agregar_variable(contexto_funcion.to_string(), var, tipo_var, vec![], dir, 1) {
      Ok(_) => (),
      Err(err) => {
        println!("{:?}", err);
      }
    }
  }
}

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
        let dir = match conseguir_direccion(tipo_temporal.as_str(), "variable", 1) {
          Ok(num) => num,
          Err(_err) => {
            // println!("{:?}", _err);
            return Err(("Error al conseguir direccion de variable temporal", ("", "".to_owned(), "".to_owned())));
          }
        };
        unsafe {
          loop {
            let nombre_temporal = format!("temporal{}", NUM_TEMPORAL);
            match tabla_variables.agregar_variable(nombre_temporal.clone(), tipo_temporal.clone(), vec![], dir) {
              Ok((_, var)) => {
                PILA_VALORES.lock().unwrap().push(var);
                agregar_temporal_a_tabla(nombre_temporal.clone(), tipo_temporal.clone(), dir);
                break;
              },
              Err(_) => {
                NUM_TEMPORAL += 1;
              }
            }
          }
        }
        self.lista.push((op_num, izq.direccion, der.direccion, dir));
        Ok(("Tipos compatibles", (operador, izq.tipo, der.tipo)))
      }
    }
  }

  pub fn agregar_cuadruplo_for<'a>(&mut self, objetivo: TipoVar) -> Result<(&'a str, String), (&'a str, String, String)>{
    let op_num = conseguir_num_operador("+");
    let obj_num = conseguir_num_tipo(objetivo.tipo.as_str());

    match checar_cubo_semantico(op_num as usize, 0 as usize, obj_num as usize) {
      2 => Err(("Variable objetivo no es un número", objetivo.nombre.clone(), objetivo.tipo.clone())),
      3 => Err(("Variable objetivo no es un número", objetivo.nombre.clone(), objetivo.tipo.clone())),
      4 => Err(("Variable objetivo no es un número", objetivo.nombre.clone(), objetivo.tipo.clone())),
      5 => Err(("Variable objetivo no es un número", objetivo.nombre.clone(), objetivo.tipo.clone())),
      _ => {
        self.lista.push((op_num, CONSTANTES.lock().unwrap().agregar_constante("1".to_owned(), "entero".to_owned()).direccion, objetivo.direccion, objetivo.direccion));
        Ok(("Incremento de for creado", objetivo.tipo.clone()))
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

  pub fn agregar_cuadruplo_gotof<'a>(&mut self, resultado: TipoVar) -> Result<&'a str, &'a str>{
    let op_num = conseguir_num_operador("GOTOF");
    match conseguir_num_tipo(resultado.tipo.as_str()) {
      0 => 0,
      1 => 1,
      _ => {
        println!("Tipo incompatible en GOTOF: {:?}", resultado);
        return Err("GOTOF incompatible");
      }
    };
    self.lista.push((op_num, resultado.direccion, -1, -1));
    Ok("GOTOF bueno")
  }

  pub fn agregar_cuadruplo_endfunc<'a>(&mut self) -> Result<&'a str, &'a str>{
    let op_num = conseguir_num_operador("ENDFUNC");
    self.lista.push((op_num, -1, -1, -1));
    Ok("ENDFUNC generado")
  }

  pub fn agregar_cuadruplo_return<'a>(&mut self, valor: TipoVar, dir_func: i64) -> Result<&'a str, &'a str>{
    let op_num = conseguir_num_operador("RETURN");
    self.lista.push((op_num, valor.direccion , -1, dir_func));
    Ok("RETURN generado")
  }

  pub fn agregar_cuadruplo_era<'a>(&mut self, dir_func: i64) -> Result<(&'a str, i64), (&'a str, i64)>{
    let op_num = conseguir_num_operador("ERA");
    self.lista.push((op_num, -1, -1, dir_func));
    Ok(("ERA generado", dir_func))
  }

  pub fn agregar_cuadruplo_param<'a>(&mut self, valor: TipoVar, destino: TipoVar) -> Result<(&'a str, (String, String)), (&'a str, (String, String))>{
    let op_num = conseguir_num_operador("PARAM");
    let as_num = conseguir_num_operador("=");
    let valor_num = conseguir_num_tipo(valor.tipo.as_str());
    let destino_num = conseguir_num_tipo(destino.tipo.as_str());

    match checar_cubo_semantico(as_num as usize, valor_num as usize, destino_num as usize) {
      3 => Err(("Asignacion de parametro incompatible", (valor.tipo, destino.tipo))),
      _ => {
        self.lista.push((op_num, valor.direccion, -1, destino.direccion));
        Ok(("Asignacion de parametro compatible", (valor.tipo, destino.tipo)))
      }
    }
  }

  pub fn agregar_cuadruplo_gosub<'a>(&mut self, cuadruplo: i64) -> Result<(&'a str, i64), (&'a str, i64)>{
    let op_num = conseguir_num_operador("GOSUB");
    self.lista.push((op_num, -1, -1, cuadruplo));
    Ok(("GOSUB generado", cuadruplo))
  }

  // pub fn modificar_cuadruplo_gotof<'a>(&mut self, num_cuadruplo: usize) -> Result<(&'a str, usize, i64), (&'a str, usize, i64)>{
  //   let direccion_cuadruplo = (self.lista.len()) as i64;
  //   self.lista[num_cuadruplo].3 = direccion_cuadruplo;
  //   Ok(("Goto modificado", num_cuadruplo, direccion_cuadruplo))
  // }
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
