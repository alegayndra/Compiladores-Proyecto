use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::semantica::tabla_clases::*;
use crate::semantica::tabla_funciones::*;
use crate::semantica::tabla_variables::*;
use crate::semantica::cuadruplos::*;
use crate::semantica::cubo_semantico::*;

lazy_static! {
  pub static ref CLASES: Mutex<TablaClases> = {
    let map = Mutex::new(TablaClases { tabla: HashMap::new() });
    map
  };

  pub static ref FUNCIONES: Mutex<TablaFunciones> = {
    let map = Mutex::new(TablaFunciones { tabla: HashMap::new() });
    map
  };

  pub static ref VARIABLES: Mutex<TablaVariables> = {
    let map = Mutex::new(TablaVariables { tabla: HashMap::new() });
    map
  };

  pub static ref CONSTANTES: Mutex<TablaVariables> = {
    let map = Mutex::new(TablaVariables { tabla: HashMap::new() });
    map
  };

  pub static ref CONTEXTO_FUNCION: Mutex<String> = {
    let string = Mutex::new("".to_owned());
    string
  };

  pub static ref ID_PROGRAMA: Mutex<String> = {
    let string = Mutex::new("".to_owned());
    string
  };

  pub static ref CONTEXTO_CLASE: Mutex<String> = {
    let string = Mutex::new("".to_owned());
    string
  };

  pub static ref CUADRUPLOS: Mutex<ListaCuadruplos> = {
    let lista = Mutex::new(ListaCuadruplos { lista: vec![] });
    lista
  };

  pub static ref PILA_OPERADORS: Mutex<Vec<String>> = {
    let operadores = Mutex::new(vec![]);
    operadores
  };

  pub static ref PILA_VALORES: Mutex<Vec<TipoVar>> = {
    let operadores = Mutex::new(vec![]);
    operadores
  };

  pub static ref PILA_SALTOS: Mutex<Vec<i64>> = {
    let saltos = Mutex::new(vec![]);
    saltos
  };
}

pub static mut ERA_CONSTANTES: (i64, i64, i64, i64) = (0, 0, 0, 0);

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

fn conseguir_contexto(contexto_num: usize) -> String {
  match contexto_num {
    0 => "global",
    1 => "local",
    2 => "constante",
    _ => ""
  }.to_owned()
}

pub fn conseguir_direccion(tipo_var: &str, contexto: &str, temporal: usize) -> Result<i64, String> {
  let contexto_num: usize;
  let tipo_num: usize;
  if contexto == "constante" {
    contexto_num = 2;
  } else if ID_PROGRAMA.lock().unwrap().to_string() == CONTEXTO_FUNCION.lock().unwrap().to_string() { // contexto global
    contexto_num = 0;
  } else {
    contexto_num = 1;
  }

  tipo_num = match conseguir_num_tipo(tipo_var) {
    3 => return Err("Variable de tipo error".to_owned()),
    4 => return Err("Variable de tipo objeto".to_owned()),
    5 => 2 as usize,
    n => n as usize
  };

  unsafe {
    let dir_nueva = DIRECCIONES[contexto_num][tipo_num][temporal][0];
    if dir_nueva + 1 >= DIRECCIONES[contexto_num][tipo_num][temporal][2] {
      return Err(format!("Limite de tipo {} alcanzado en el contexto {}", tipo_var, conseguir_contexto(contexto_num)))
    }

    DIRECCIONES[contexto_num][tipo_num][temporal][0] = dir_nueva + 1;
    return Ok(dir_nueva)
  }
}