//! Módulo que se encarga de las variables globales de la semántica.
//! 
//! # _Mutex_
//! Para acceder a cualquiera de los `Mutex` encontrados en este archivo se hace de la siguiente manera:
//!
//! ```ignore
//! MUTEX.lock().unwrap();
//! 
//! // Por ejemplo
//! 
//! CLASES.lock().unwrap();
//! ```
//! 
//! # _Statics_
//! Para acceder a cualquiera de las `statics` encontrados en este archivo se hace de la siguiente manera:
//!
//! ```ignore
//! unsafe {
//!   VARIABLE;
//! }
//! // Por ejemplo
//! 
//! unsafe {
//!   DIRECCION_CONTEXTO_FUNCION = 10;
//! }
//! ```
//! 

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::semantica::tabla_clases::*;
use crate::semantica::tabla_funciones::*;
use crate::semantica::tabla_variables::*;
use crate::semantica::cuadruplos::*;
use crate::semantica::cubo_semantico::*;

// Variables globales que se necesitan crear en ejecución
lazy_static! {
  /// Tabla de clases globales
  pub static ref CLASES: Mutex<TablaClases> = {
    let map = Mutex::new(TablaClases { tabla: HashMap::new() });
    map
  };

  /// Tabla de funciones globales
  pub static ref FUNCIONES: Mutex<TablaFunciones> = {
    let map = Mutex::new(TablaFunciones { tabla: HashMap::new() });
    map
  };

  /// Tabla de variables 
  pub static ref VARIABLES: Mutex<TablaVariables> = {
    let map = Mutex::new(TablaVariables { tabla: HashMap::new() });
    map
  };

  /// Tabla de constantes
  pub static ref CONSTANTES: Mutex<TablaVariables> = {
    let map = Mutex::new(TablaVariables { tabla: HashMap::new() });
    map
  };

  /// Contexto de la función actual
  pub static ref CONTEXTO_FUNCION: Mutex<String> = {
    let string = Mutex::new("".to_owned());
    string
  };

  /// ID de del programa
  pub static ref ID_PROGRAMA: Mutex<String> = {
    let string = Mutex::new("".to_owned());
    string
  };

  /// Contexto de la clase actual
  pub static ref CONTEXTO_CLASE: Mutex<String> = {
    let string = Mutex::new("".to_owned());
    string
  };

  /// Lista de cuadruplos
  pub static ref CUADRUPLOS: Mutex<ListaCuadruplos> = {
    let lista = Mutex::new(ListaCuadruplos { lista: vec![] });
    lista
  };

  /// Pila de operadores
  pub static ref PILA_OPERADORS: Mutex<Vec<String>> = {
    let operadores = Mutex::new(vec![]);
    operadores
  };

  /// Pila de valores
  pub static ref PILA_VALORES: Mutex<Vec<TipoVar>> = {
    let operadores = Mutex::new(vec![]);
    operadores
  };

  /// Pila de saltos
  pub static ref PILA_SALTOS: Mutex<Vec<i64>> = {
    let saltos = Mutex::new(vec![]);
    saltos
  };

  /// Pila de dimensiones
  pub static ref PILA_DIMENSIONES: Mutex<Vec<(TipoVar, i64)>> = {
    let saltos = Mutex::new(vec![]);
    saltos
  };
}

/// Booleano que marca si hay un `return` o no en la función actual.
pub static mut RETURN_EXISTENTE: bool = false;

/// Entero que guarda la dirección de memoria de la variable relacionada con la función actual.
pub static mut DIRECCION_CONTEXTO_FUNCION: i64 = -10;

/// Tupla que marca el era de las constantes.
pub static mut ERA_CONSTANTES: (i64, i64, i64) = (0, 0, 0);

/// Mapa de memoria de los diferentes contextos y tipos de variables.
pub static mut DIRECCIONES: [[[[i64 ; 3] ; 2] ; 3] ; 3] = [
  [ // Globales
    [ // Enteras
      // Val    Inicio  Limite
      [    0,     0,    833], // Normales
      [  833,   833,    1250]  // Temporales
    ],
    [ // Flotantes
      // Val    Inicio  Limite
      [ 1250,   1250,    2083], // Normales
      [ 2083,   2083,    2500]  // Temporales
    ],
    [ // Caracteres
      // Val    Inicio  Limite
      [ 2500,   2500,    2833], // Normales
      [ 2833,   2833,    3000]  // Temporales
    ]
  ],
  [ // Locales
    [ // Enteras
      // Val    Inicio  Limite
      [ 3000,   3000,    4665], // Normales
      [ 4665,   4665,    5500]  // Temporales
    ],
    [ // Flotantes
      // Val    Inicio  Limite
      [ 5500,   5500,    7164], // Normales
      [ 7164,   7164,    8000]  // Temporales
    ],
    [ // Caracteres
      // Val    Inicio  Limite
      [ 8000,   8000,    8666], // Normales
      [ 8666,   8666,    9000]  // Temporales
    ]
  ],
  [ // Constantes
    [ // Enteras
      // Val    Inicio  Limite
      [ 9000,   9000,    9401], // Normales
      [   -1,     -1,      -1]  // Temporales (no hay en constates)
    ],
    [ // Flotantes
      // Val    Inicio  Limite
      [ 9401,   9401,    9801], // Normales
      [   -1,     -1,      -1]  // Temporales (no hay en constates)
    ],
    [ // Caracteres
      // Val    Inicio  Limite
      [ 9801,   9801,   10000], // Normales
      [   -1,     -1,      -1]  // Temporales (no hay en constates)
    ]
  ]
];

/// Función auxiliar para conseguir el contexto actual.  
/// Regresa un String.
///
/// # Parametros
///
/// * `contexto_num` - Número represantivo del contexto
///
/// # Ejemplo
///
/// ```ignore
/// conseguir_contexto(1);
/// ```
fn conseguir_contexto(contexto_num: usize) -> String {
  match contexto_num {
    0 => "global",
    1 => "local",
    2 => "constante",
    _ => ""
  }.to_owned()
}

/// Función auxiliar para separar y conseguir una dirección de memoria.  
/// Regresa un Result con la dirección de memoria.
///
/// # Parametros
///
/// * `tipo_var` - Tipo de la variable
/// * `contexto` - String que determina si la variable es constante o no
/// * `temporal` - Número que marca si la variables es un temporal
/// * `dims` - Arreglo de las dimensiones de la variable
///
/// # Ejemplo
///
/// ```ignore
/// match conseguir_direccion("entero", "constante", 0, vec![]) { 
///   Ok(direccion) => direccion, // Se separó la memoria éxitosamente
///   Err(err) => err, // Error al agregar la variable
/// };
/// ```
pub fn conseguir_direccion(tipo_var: &str, contexto: &str, temporal: usize, dims: Vec<i64>) -> Result<i64, String> {
  println!("conseguir_direccion {}", tipo_var);
  let contexto_num: usize;
  let tipo_num: usize;
  // Busca si el contexto es constante, local o global
  if contexto == "constante" {
    contexto_num = 2;
  } else if ID_PROGRAMA.lock().unwrap().to_string() == CONTEXTO_FUNCION.lock().unwrap().to_string() { // contexto global
    contexto_num = 0;
  } else {
    contexto_num = 1;
  }

  // Calcula cuantas direcciones de memoria se van a separar dependiendo de las dimensiones de la variable
  let cant_direcciones: i64 = match dims.len() {
    2 => dims[0] * dims[1],
    1 => dims[0],
    _ => 1
  };

  // Consigue el tipo de la variable en número
  tipo_num = match conseguir_num_tipo(tipo_var) {
    3 => return Err("Variable de tipo error".to_owned()),
    4 => return Err("Variable de tipo objeto".to_owned()),
    6 => return Err("Variable de tipo void".to_owned()),
    5 => 2 as usize,
    n => n as usize
  };

  unsafe {
    let dir_nueva = DIRECCIONES[contexto_num][tipo_num][temporal][0];
    // Checa que no se exceda el tamaño de la memoria
    if dir_nueva + cant_direcciones >= DIRECCIONES[contexto_num][tipo_num][temporal][2] {
      return Err(format!("Limite de tipo {} alcanzado en el contexto {}", tipo_var, conseguir_contexto(contexto_num)))
    }

    // Separa las direcciones de memoria
    DIRECCIONES[contexto_num][tipo_num][temporal][0] = dir_nueva + cant_direcciones;
    Ok(dir_nueva)
  }
}

/// Función auxiliar para _resetear_ las direcciones locales.  
///
/// # Ejemplo
///
/// ```ignore
/// resetear_direcciones_locales();
/// ```
pub fn resetear_direcciones_locales()  {
  unsafe {
    DIRECCIONES[1][0][0][0] = DIRECCIONES[1][0][0][1];
    DIRECCIONES[1][0][1][0] = DIRECCIONES[1][0][1][1];
    DIRECCIONES[1][1][0][0] = DIRECCIONES[1][1][0][1];
    DIRECCIONES[1][1][1][0] = DIRECCIONES[1][1][1][1];
    DIRECCIONES[1][2][0][0] = DIRECCIONES[1][2][0][1];
    DIRECCIONES[1][2][1][0] = DIRECCIONES[1][2][1][1];
  }
}
