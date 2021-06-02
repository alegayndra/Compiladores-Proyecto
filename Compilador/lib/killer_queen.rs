//! Parser del lenguaje _Killer Queen_
//! 
//! Utilizamos las librerias de _nom_ y _lazy static_, para parsear y crear variables estáticas en ejecución respectivamente.

extern crate nom;
pub mod scanners;
pub mod semantica;
pub mod parser;
