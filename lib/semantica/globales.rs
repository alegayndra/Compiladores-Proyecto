use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::semantica::tabla_clases::*;
use crate::semantica::tabla_funciones::*;
use crate::semantica::tabla_variables::*;
use crate::semantica::cuadruplos::*;

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
}

pub static mut DIRECCIONES_GLOBALES: [[[[i64 ; 2] ; 2] ; 3] ; 3] = [
  [ // Globales
    [ // Enteras
      // Val    Inicio
      [    0,     0], // Normales
      [  833,   833]  // Temporales
    ],
    [ // Flotantes
      // Val    Inicio
      [ 1250,   1250], // Normales
      [ 2083,   2083]  // Temporales
    ],
    [ // Caracteres
      // Val    Inicio
      [ 2500,   2500], // Normales
      [ 2833,   2833]  // Temporales
    ]
  ],
  [ // Locales
    [ // Enteras
      // Val    Inicio
      [ 3000,   3000], // Normales
      [ 4665,   4665]  // Temporales
    ],
    [ // Flotantes
      // Val    Inicio
      [ 5500,   5500], // Normales
      [ 7164,   7164]  // Temporales
    ],
    [ // Caracteres
      // Val    Inicio
      [ 8000,   8000], // Normales
      [ 8666,   8666]  // Temporales
    ]
  ],
  [ // Constantes
    [ // Enteras
      // Val    Inicio
      [ 9000,   9000], // Normales
      [   -1,     -1]  // Temporales (no hay en constates)
    ],
    [ // Flotantes
      // Val    Inicio
      [ 9401,   9401], // Normales
      [   -1,     -1]  // Temporales (no hay en constates)
    ],
    [ // Caracteres
        // Val    Inicio
        [ 9801,   9801], // Normales
        [   -1,     -1]  // Temporales (no hay en constates)
    ]
  ]
];
