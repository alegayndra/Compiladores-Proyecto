use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::semantica::tabla_clases::*;
use crate::semantica::tabla_funciones::*;
use crate::semantica::tabla_variables::*;

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
}
