/*
  Tipos de datos:
  - Entero : 0
  - Flotante : 1
  - Char : 2
  - Error : 3
  - Objeto : 4

  Operadores:
  - sum_sub
    - + : 0
    - - : 1
  - mult_div
    - * : 2
    - / : 3
  - relacionales
    - > : 4
    - < : 5
    - >= : 6
    - <= : 7
    - == : 8
    - != : 9
  - logica
    - & : 10
    - | : 11
  - Asignacion
    - = : 12
*/

pub struct CuboSemantico {
  pub lista: Vec<Vec<Vec<i64>>>
}

impl CuboSemantico {
  pub fn new() -> CuboSemantico {

    CuboSemantico {
      lista: vec![
        vec![ // Entero
          //    +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
          vec![ 0, 0, 0, 0, 0, 0,  0,  0,  0,  0, 0, 0, 0], // Entero
          vec![ 1, 1, 1, 1, 0, 0,  0,  0,  0,  0, 0, 0, 0], // Flotante
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Char
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
        ],
        vec![ // Flotante
          //    +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
          vec![ 1, 1, 1, 1, 0, 0,  0,  0,  0,  0, 0, 0, 1], // Entero
          vec![ 1, 1, 1, 1, 0, 0,  0,  0,  0,  0, 0, 0, 1], // Flotante
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Char
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
        ],
        vec![ // Char
          //    +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Entero
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Flotante
          vec![ 3, 3, 3, 3, 0, 0,  0,  0,  0,  0, 0, 0, 2], // Char
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
        ],
        vec![ // Error
          //    +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Entero
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Flotante
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Char
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
        ],
        vec![ // Objeto
          //    +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Entero
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Flotante
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Char
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
          vec![ 3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
        ],
      ]
    }
  }

  pub fn checar(&self, izq: usize, der: usize, op: usize) -> i64{
    self.lista[izq][der][op]
  }
}

pub fn conseguir_num_operador(operador: &str) -> i64 {
  match operador {
    "+"   => 0,
    "-"   => 1,
    "*"   => 2,
    "/"   => 3,
    ">"   => 4,
    "<"   => 5,
    ">="  => 6,
    "<="  => 7,
    "=="  => 8,
    "!="  => 9,
    "&"   => 10,
    "|"   => 11,
    "="   => 12,
    _     => -1,
  }
}

pub fn conseguir_num_tipo(tipo: &str) -> i64 {
  match tipo {
    "entero"    => 0,
    "flotante"  => 1,
    "char"      => 2,
    "error"     => 3,
    _           => 4,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_checar() {
    let cubo : CuboSemantico = CuboSemantico::new();
    assert_eq!(cubo.checar(0, 0, 0), 0);
    assert_eq!(cubo.checar(0, 1, 1), 1);
    assert_eq!(cubo.checar(3, 1, 5), 3);
  }

  #[test]
  fn test_conseguir_num_operador() {
    assert_eq!(conseguir_num_operador("+"),  0);
    assert_eq!(conseguir_num_operador("-"),  1);
    assert_eq!(conseguir_num_operador("*"),  2);
    assert_eq!(conseguir_num_operador("/"),  3);
    assert_eq!(conseguir_num_operador(">"),  4);
    assert_eq!(conseguir_num_operador("<"),  5);
    assert_eq!(conseguir_num_operador(">="), 6);
    assert_eq!(conseguir_num_operador("<="), 7);
    assert_eq!(conseguir_num_operador("=="), 8);
    assert_eq!(conseguir_num_operador("!="), 9);
    assert_eq!(conseguir_num_operador("&"),  10);
    assert_eq!(conseguir_num_operador("|"),  11);
    assert_eq!(conseguir_num_operador("="),  12);
  }
  #[test]
  fn test_conseguir_num_tipo() {
    assert_eq!(conseguir_num_tipo("entero"),    0);
    assert_eq!(conseguir_num_tipo("flotante"),  1);
    assert_eq!(conseguir_num_tipo("char"),      2);
    assert_eq!(conseguir_num_tipo("error"),     3);
    assert_eq!(conseguir_num_tipo("a"),         4);
  }
}
