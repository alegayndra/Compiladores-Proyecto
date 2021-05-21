use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::semantica::cuadruplos::*;

lazy_static! {
  pub static ref CUADRUPLOS: Mutex<ListaCuadruplos> = {
    let lista = Mutex::new(ListaCuadruplos { lista: vec![] });
    lista
  };

  pub static ref PILA_OPERADORS: Mutex<Vec<&str>> = {
    let operadores = Mutex::new(vec![]);
    operadores
  };

  pub static ref PILA_VALORES: Mutex<Vec<&str>> = {
    let operadores = Mutex::new(vec![]);
    operadores
  };
}
