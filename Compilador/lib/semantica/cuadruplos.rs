//! Módulo que se encarga de la generación y modificación de cuadruplo.

use crate::semantica::tabla_variables::*;
use crate::semantica::cubo_semantico::*;
use crate::semantica::globales::*;

/// Lista de cuadruplos.  
///
/// # Atributos
///
/// * `lista` - Lista de cuadruplos
///
/// # Ejemplo de creación
/// ```
/// let cuadruplos: ListaCuadruplos = ListaCuadruplos {
///   lista: vec![]
/// };
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListaCuadruplos {
  pub lista: Vec<(i64, i64, i64, i64)>
}

/// Número que indica el siguiente cuadruplo a crear
static mut NUM_TEMPORAL: i64 = 0;

/// Función auxiliar para agregar un temporal a las tablas de variables.  
///
/// # Parametros
///
/// * `var` - ID del temporal
/// * `tipo_var` - Tipo del temporal
/// * `dir` - Dirección de memoria del temporal
///
/// # Ejemplo
///
/// ```
/// agregar_temporal_a_tabla("temporal1".to_owned(), "entero".to_owned(), 5); // Suma entre enteros
/// ```
pub fn agregar_temporal_a_tabla(var: String, tipo_var: String, dir: i64) {
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

/// Diferentes métodos implementados para el acceso y modificación de cuadruplos.
impl ListaCuadruplos {
  /// Función para generar un cuadruplo de operaciones.  
  ///
  /// # Parametros
  ///
  /// * `operador` - Operador
  /// * `izq` - Tipo perando izquierda
  /// * `der` - Tipo operando derecho
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo("+", "entero", "entero"); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo<'a>(&mut self, operador: &'a str, izq: TipoVar, der: TipoVar) -> Result<(&'a str, (&'a str, String, String)), (&'a str, (&'a str, String, String))>{
    // Convierte los parametros a número para checar el cubo semántico
    let op_num = conseguir_num_operador(operador);
    let izq_num = conseguir_num_tipo(izq.tipo.as_str());
    let der_num = conseguir_num_tipo(der.tipo.as_str());

    match checar_cubo_semantico(op_num as usize, izq_num as usize, der_num as usize) {
      3 => Err(("Tipos incompatibles", (operador, izq.tipo, der.tipo))),
      n => {
        // Crear temporal
        let mut tabla_variables = VARIABLES.lock().unwrap();
        let tipo_temporal = conseguir_tipo_num(n);
        let dir = match conseguir_direccion(tipo_temporal.as_str(), "variable", 1, vec![]) {
          Ok(num) => num,
          Err(_err) => {
            // println!("{:?}", _err);
            return Err(("Error al conseguir direccion de variable temporal", ("", "".to_owned(), "".to_owned())));
          }
        };
        // Itera hasta conseguir un nombre válido de temporal
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
        // Crea cuádruplo
        self.lista.push((op_num, izq.direccion, der.direccion, dir));
        Ok(("Tipos compatibles", (operador, izq.tipo, der.tipo)))
      }
    }
  }

  /// Función para generar un cuadruplo de modificación del valor de un `desde`.  
  ///
  /// # Parametros
  ///
  /// * `objetivo` - Dirección objetivo
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_for(100); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_for<'a>(&mut self, objetivo: i64) -> Result<(&'a str, i64), (&'a str, i64)>{
    let op_num = conseguir_num_operador("+");
    self.lista.push((op_num, CONSTANTES.lock().unwrap().agregar_constante("1".to_owned(), "entero".to_owned()).direccion, objetivo, objetivo));
    Ok(("Incremento de for creado", objetivo))
  }

  /// Función para generar un cuadruplo de asignación.  
  ///
  /// # Parametros
  ///
  /// * `valor` - Valor a asignar
  /// * `destino` - Variable a la que se le asignara el valor
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_asignacion(TipoVar { /* atributos */ }, TipoVar { /* atributos */ }); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_asignacion<'a>(&mut self, valor: TipoVar, destino: TipoVar) -> Result<(&'a str, (String, String)), (&'a str, (String, String))>{
    // Convierte los parametros a número para checar el cubo semántico
    let op_num = conseguir_num_operador("=");
    let valor_num = conseguir_num_tipo(valor.tipo.as_str());
    let destino_num = conseguir_num_tipo(destino.tipo.as_str());

    match checar_cubo_semantico(op_num as usize, valor_num as usize, destino_num as usize) {
      3 => Err(("Asignacion incompatible", (valor.tipo, destino.tipo))),
      _ => {
        // Crea cuádruplo
        self.lista.push((op_num, valor.direccion, -1, destino.direccion));
        Ok(("Asignacion compatible", (valor.tipo, destino.tipo)))
      }
    }
  }

  /// Función para generar un cuadruplo de asignación, donde el destino es la posición dentro de un arreglo.  
  /// El operador ASG sirve para que el destino se maneje como un apuntador dentro de la máquina virtual.  
  ///
  /// # Parametros
  ///
  /// * `valor` - Valor a asignar
  /// * `destino` - Variable a la que se le asignara el valor
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_asignacion_arreglo(TipoVar { /* atributos */ }, TipoVar { /* atributos */ }); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_asignacion_arreglo<'a>(&mut self, valor: TipoVar, destino: TipoVar) -> Result<(&'a str, (String, String)), (&'a str, (String, String))>{
    // Convierte los parametros a número para checar el cubo semántico
    let op_num = conseguir_num_operador("ASG");
    let asg_num = conseguir_num_operador("=");
    let valor_num = conseguir_num_tipo(valor.tipo.as_str());
    let destino_num = conseguir_num_tipo(destino.tipo.as_str());

    // Checa cubo semántico con el operador de '=', ya que 'ASG' sigue siendo una asignación
    match checar_cubo_semantico(asg_num as usize, valor_num as usize, destino_num as usize) {
      3 => Err(("Asignacion de arreglo incompatible", (valor.tipo, destino.tipo))),
      _ => {
        // Crea cuádruplo
        self.lista.push((op_num, valor.direccion, -1, destino.direccion));
        Ok(("Asignacion de arreglo compatible", (valor.tipo, destino.tipo)))
      }
    }
  }

  /// Función para generar un cuadruplo de escritura.  
  ///
  /// # Parametros
  ///
  /// * `valor` - Valor a escribir
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_escritura(TipoVar { /* atributos */ }); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_escritura<'a>(&mut self, valor: TipoVar) -> Result<(&'a str, String), (&'a str, String)>{
    let op_num = conseguir_num_operador("ESCRIBE");
    self.lista.push((op_num, -1, -1, valor.direccion));
    Ok(("Print bueno", valor.tipo))
  }

  /// Función para generar un cuadruplo de lectura.  
  ///
  /// # Parametros
  ///
  /// * `valor` - Valor a leer
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_lectura(TipoVar { /* atributos */ }); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_lectura<'a>(&mut self, valor: TipoVar) -> Result<(&'a str, String), (&'a str, String)>{
    let op_num = conseguir_num_operador("LEE");
    self.lista.push((op_num, -1, -1, valor.direccion));
    Ok(("Read bueno", valor.tipo))
  }

  /// Función para generar un cuadruplo de goto.  
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_goto(); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_goto<'a>(&mut self) -> Result<&'a str, &'a str>{
    let op_num = conseguir_num_operador("GOTO");
    self.lista.push((op_num, -1, -1, -1));
    Ok("Goto bueno")
  }

  /// Función para modificar un cuadruplo de salto, ya sea goto o gotof.  
  ///
  /// # Parametros
  ///
  /// * `num_cuadruplo` - Cuadruplo a modificar
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_lectura(5); // Suma entre enteros
  /// ```
  pub fn modificar_cuadruplo_goto<'a>(&mut self, num_cuadruplo: usize) -> Result<(&'a str, usize, i64), (&'a str, usize, i64)> {
    // Consigue el cuadruplo al cual se tiene que hacer el salto
    let direccion_cuadruplo = (self.lista.len()) as i64;
    self.lista[num_cuadruplo].3 = direccion_cuadruplo;
    Ok(("Goto modificado", num_cuadruplo, direccion_cuadruplo))
  }

  /// Función para modificar un cuadruplo de salto, ya sea goto o gotof.  
  /// En este caso se modifica el salto dentro de un `sino`, dónde requerimos una posición extra.  
  ///
  /// # Parametros
  ///
  /// * `num_cuadruplo` - Cuadruplo a modificar
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.modificar_cuadruplo_goto_sino(1); // Suma entre enteros
  /// ```
  pub fn modificar_cuadruplo_goto_sino<'a>(&mut self, num_cuadruplo: usize) -> Result<(&'a str, usize, i64), (&'a str, usize, i64)>{
    // Consigue el cuadruplo al cual se tiene que hacer el salto
    let direccion_cuadruplo = (self.lista.len() + 1) as i64;
    self.lista[num_cuadruplo].3 = direccion_cuadruplo;
    Ok(("Goto modificado", num_cuadruplo, direccion_cuadruplo))
  }

  /// Función para generar un cuadruplo de gotof.  
  ///
  /// # Parametros
  ///
  /// * `resultado` - Valor que se va a evaluar
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_gotof(TipoVar { /* atributos */ }); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_gotof<'a>(&mut self, resultado: TipoVar) -> Result<&'a str, &'a str>{
    // Se checa que sea un tipo válido
    let op_num = conseguir_num_operador("GOTOF");
    match conseguir_num_tipo(resultado.tipo.as_str()) {
      0 => 0,
      1 => 1,
      _ => {
        println!("Tipo incompatible en GOTOF: {:?}", resultado);
        return Err("GOTOF incompatible");
      }
    };

    // Crea el cuádruplo
    self.lista.push((op_num, resultado.direccion, -1, -1));
    Ok("GOTOF bueno")
  }

  /// Función para generar un cuadruplo de gotof dentro de un `desde`.  
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_gotof_desde(); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_gotof_desde<'a>(&mut self) -> Result<(&'a str, i64), (&'a str, i64)>{
    let op_num = conseguir_num_operador("GOTOF");
    // Consigue la posición del último cuadruplo
    let pos_arr = self.lista.len() - 1;

    // Checa que tipo de operación está en el último cuadruplo
    // Si es una asignación, saca la dirección del destino
    let dir = match self.lista[pos_arr].0 {
      12 => self.lista[pos_arr].3,
      25 => {
        // En el caso que sea una asignación a un elemento no atómico, crea un cuadruplo de acceso al valor de un elemento no atómico
        match self.agregar_cuadruplo_acceder_desde(self.lista[pos_arr].3) {
          Ok((_, _, dir_temp)) => dir_temp,
          Err(_) => return Err(("Error al crear GOTOF desde", -100))
        }
      },
      _ => return Err(("Cuadruplo inválido en GOTOF desde", -9)) // Cuadruplo inválida
    };

    // Crea cuadruplo
    self.lista.push((op_num, dir, -1, -1));
    Ok(("GOTOF desde bueno", dir))
  }

  /// Función para generar un cuadruplo de acceso a un apuntador dentro de un `desde`.  
  ///
  /// # Parametros
  ///
  /// * `apuntador` - Dirección de apuntador
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_acceder_desde(10); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_acceder_desde<'a>(&mut self, apuntador: i64) -> Result<(&'a str, i64, i64), (&'a str, i64, i64)>{
    let op_num = conseguir_num_operador("ACC");
    let mut tabla_variables = VARIABLES.lock().unwrap();
    // Consigue una dirección de memoria para el temporal
    let dir = match conseguir_direccion("entero", "variable", 1, vec![]) {
      Ok(num) => num,
      Err(_err) => {
        return Err(("Error al conseguir direccion de variable temporal", 0, 0));
      }
    };
    // Crea un temporal
    unsafe {
      loop {
        let nombre_temporal = format!("temporal{}", NUM_TEMPORAL);
        match tabla_variables.agregar_variable(nombre_temporal.clone(), "entero".to_owned(), vec![], dir) {
          Ok((_, var)) => {
            PILA_VALORES.lock().unwrap().push(var);
            agregar_temporal_a_tabla(nombre_temporal.clone(), "entero".to_owned(), dir);
            break;
          },
          Err(_) => {
            NUM_TEMPORAL += 1;
          }
        }
      }
    }

    // Crea cuádruplo
    self.lista.push((op_num, apuntador, -1, dir));
    Ok(("ACC desde generado", apuntador, dir))
  }

  /// Función para generar un cuadruplo de `endfunc`.  
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_endfunc(); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_endfunc<'a>(&mut self) -> Result<&'a str, &'a str>{
    let op_num = conseguir_num_operador("ENDFUNC");
    self.lista.push((op_num, -1, -1, -1));
    Ok("ENDFUNC generado")
  }

  /// Función para generar un cuadruplo de `return`.  
  ///
  /// # Parametros
  ///
  /// * `valor` - Valor a regresa
  /// * `dir_func` - Dirección de la función
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_return(TipoVar { /* atributos */ }, 10); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_return<'a>(&mut self, valor: TipoVar, dir_func: i64) -> Result<&'a str, &'a str>{
    let op_num = conseguir_num_operador("RETURN");
    self.lista.push((op_num, valor.direccion , -1, dir_func));
    Ok("RETURN generado")
  }

  /// Función para generar un cuadruplo de `era`.  
  ///
  /// # Parametros
  ///
  /// * `num_cuadruplo` - Número de cuadruplo de la función
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_era(10); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_era<'a>(&mut self, num_cuadruplo: i64) -> Result<(&'a str, i64), (&'a str, i64)>{
    let op_num = conseguir_num_operador("ERA");
    self.lista.push((op_num, -1, -1, num_cuadruplo));
    Ok(("ERA generado", num_cuadruplo))
  }

  /// Función para generar un cuadruplo de asignación a un parámetro.  
  ///
  /// # Parametros
  ///
  /// * `valor` - Valor a asignar
  /// * `destino` - Variable a la que se le asignara el valor
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_param(TipoVar { /* atributos */ }, TipoVar { /* atributos */ }); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_param<'a>(&mut self, valor: TipoVar, destino: TipoVar) -> Result<(&'a str, (String, String)), (&'a str, (String, String))>{
    // Convierte los parametros a número para checar el cubo semántico
    let op_num = conseguir_num_operador("PARAM");
    let as_num = conseguir_num_operador("=");
    let valor_num = conseguir_num_tipo(valor.tipo.as_str());
    let destino_num = conseguir_num_tipo(destino.tipo.as_str());

    match checar_cubo_semantico(as_num as usize, valor_num as usize, destino_num as usize) {
      3 => Err(("Asignacion de parametro incompatible", (valor.tipo, destino.tipo))),
      _ => {
        // Crea cuádruplo
        self.lista.push((op_num, valor.direccion, -1, destino.direccion));
        Ok(("Asignacion de parametro compatible", (valor.tipo, destino.tipo)))
      }
    }
  }

  /// Función para generar un cuadruplo de `gosub`.  
  ///
  /// # Parametros
  ///
  /// * `num_cuadruplo` - Número de cuadruplo de la función
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_gosub(10); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_gosub<'a>(&mut self, num_cuadruplo: i64) -> Result<(&'a str, i64), (&'a str, i64)>{
    let op_num = conseguir_num_operador("GOSUB");
    self.lista.push((op_num, -1, -1, num_cuadruplo));
    Ok(("GOSUB generado", num_cuadruplo))
  }

  /// Función para generar un cuadruplo de verificar que un valor este dentro de las dimensiones de un elemento no atómico.  
  ///
  /// # Parametros
  ///
  /// * `direccion` - Número de cuadruplo de la función
  /// * `dimension` - Número de cuadruplo de la función
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_verificar(10, 20); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_verificar<'a>(&mut self, direccion: i64, dimension: i64) -> Result<(&'a str, i64, i64), (&'a str, i64, i64)>{
    let op_num = conseguir_num_operador("VER");
    let mut constantes = CONSTANTES.lock().unwrap();

    // Convierte las dimensiones en constantes
    let cero = constantes.agregar_constante("0".to_owned(), "entero".to_owned());
    let dim = constantes.agregar_constante(dimension.to_string(), "entero".to_owned());
    self.lista.push((op_num, direccion, cero.direccion, dim.direccion - 1));
    Ok(("VER generado", direccion, dimension))
  }

  /// Función para generar un cuadruplo para acceder a la posición dentro de un arreglo.  
  /// Simula el acceso a un apuntador al guardar el valor dentro de la dirección que guarda el apuntador en un temporal.  
  ///
  /// # Parametros
  ///
  /// * `apuntador` - Apuntador de arreglo
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_acceder(TipoVar { /* atributos */ }); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_acceder<'a>(&mut self, apuntador: TipoVar) -> Result<(&'a str, i64, i64), (&'a str, i64, i64)>{
    let op_num = conseguir_num_operador("ACC");
    let mut tabla_variables = VARIABLES.lock().unwrap();
    // Consigue dirección para temporal
    let dir = match conseguir_direccion(&apuntador.tipo, "variable", 1, vec![]) {
      Ok(num) => num,
      Err(_err) => {
        return Err(("Error al conseguir direccion de variable temporal", 0, 0));
      }
    };
    // Crea temporal
    unsafe {
      loop {
        let nombre_temporal = format!("temporal{}", NUM_TEMPORAL);
        match tabla_variables.agregar_variable(nombre_temporal.clone(), apuntador.tipo.clone(), vec![], dir) {
          Ok((_, var)) => {
            PILA_VALORES.lock().unwrap().push(var);
            agregar_temporal_a_tabla(nombre_temporal.clone(), apuntador.tipo.clone(), dir);
            break;
          },
          Err(_) => {
            NUM_TEMPORAL += 1;
          }
        }
      }
    }

    // Crea cuadruplo
    self.lista.push((op_num, apuntador.direccion, -1, dir));
    Ok(("ACC generado", apuntador.direccion, dir))
  }

  /// Función para generar un cuadruplo para guardar el valor de retorno de una función dentro de un temporal.  
  ///
  /// # Parametros
  ///
  /// * `dir_funcion` - Dirección de la función
  /// * `tipo_func` - Tipo de la función
  ///
  /// # Ejemplo
  ///
  /// ```
  /// let cuadruplos: ListaCuadruplos = ListaCuadruplos { lista: vec![] };
  /// 
  /// cuadruplos.agregar_cuadruplo_asignacion_valor_funcion(100, "entero".to_owned()); // Suma entre enteros
  /// ```
  pub fn agregar_cuadruplo_asignacion_valor_funcion<'a>(&mut self, dir_funcion: i64, tipo_func: String) -> Result<(&'a str, i64), (&'a str, i64)>{
    let op_num = conseguir_num_operador("=");
    let mut tabla_variables = VARIABLES.lock().unwrap();
    // Consigue dirección para temporal
    let dir = match conseguir_direccion(&tipo_func, "variable", 1, vec![]) {
      Ok(num) => num,
      Err(_err) => {
        // println!("{:?}", _err);
        return Err(("Error al conseguir direccion de variable temporal", 0));
      }
    };

    // Crea temporal
    unsafe {
      loop {
        let nombre_temporal = format!("temporal{}", NUM_TEMPORAL);
        match tabla_variables.agregar_variable(nombre_temporal.clone(), tipo_func.clone(), vec![], dir) {
          Ok((_, var)) => {
            PILA_VALORES.lock().unwrap().push(var);
            agregar_temporal_a_tabla(nombre_temporal.clone(), tipo_func.clone(), dir);
            break;
          },
          Err(_) => {
            NUM_TEMPORAL += 1;
          }
        }
      }
    }

    // Crea cuádruplo
    self.lista.push((op_num, dir_funcion, -1, dir));
    Ok(("Asignacion funcion generado", dir_funcion))
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
