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

// pub struct CuboSemantico {
//   pub lista: Vec<Vec<Vec<i64>>>
// }

// static CUBO_SEMANTICO: Vec<Vec<Vec<i64>>> = vec![
static CUBO_SEMANTICO: [[[i64; 13]; 6]; 6] = [
  [ // Entero
    // +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
    [  0, 0, 0, 0, 0, 0,  0,  0,  0,  0, 0, 0, 0], // Entero
    [  1, 1, 1, 1, 0, 0,  0,  0,  0,  0, 0, 0, 0], // Flotante
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Char
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Texto
  ],
  [ // Flotante
    // +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
    [  1, 1, 1, 1, 0, 0,  0,  0,  0,  0, 0, 0, 1], // Entero
    [  1, 1, 1, 1, 0, 0,  0,  0,  0,  0, 0, 0, 1], // Flotante
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Char
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Texto
  ],
  [ // Char
    // +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Entero
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Flotante
    [  3, 3, 3, 3, 0, 0,  0,  0,  0,  0, 0, 0, 2], // Char
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Texto
  ],
  [ // Error
    // +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Entero
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Flotante
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Char
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Texto
  ],
  [ // Objeto
    // +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Entero
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Flotante
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Char
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Texto
  ],
  [ // Texto
    // +  -  *  /  >  <  >=  <=  ==  !=  &  |  =
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Entero
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Flotante
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Char
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Error
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Objeto
    [  3, 3, 3, 3, 3, 3,  3,  3,  3,  3, 3, 3, 3], // Texto
  ],
];

pub fn checar_cubo_semantico(op: usize, izq: usize, der: usize) -> i64{
  CUBO_SEMANTICO[izq][der][op]
}

pub fn conseguir_num_operador(operador: &str) -> i64 {
  match operador {
    "+"       => 0,
    "-"       => 1,
    "*"       => 2,
    "/"       => 3,
    ">"       => 4,
    "<"       => 5,
    ">="      => 6,
    "<="      => 7,
    "=="      => 8,
    "!="      => 9,
    "&"       => 10,
    "|"       => 11,
    "="       => 12,
    "ESCRIBE" => 13,
    "LEE"     => 14,
    "GOTO"    => 15,
    "GOTOT"   => 16,
    "GOTOF"   => 17,
    "GOSUB"   => 18,
    "ERA"     => 19,
    "RETURN"  => 20,
    "PARAM"   => 21,
    _         => -1,
  }
}

pub fn conseguir_num_tipo(tipo: &str) -> i64 {
  match tipo {
    "entero"    => 0,
    "flotante"  => 1,
    "char"      => 2,
    "error"     => 3,
    "texto"     => 5,
    _           => 4, // objeto
  }
}

pub fn conseguir_tipo_num(tipo: i64) -> String {
  match tipo {
    0 => "entero",
    1 => "flotante",
    2 => "char",
    3 => "error",
    5 => "texto",
    _ => "clase",
  }.to_owned()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_checar() {
    assert_eq!(checar_cubo_semantico(0, 0, 0), 0);
    assert_eq!(checar_cubo_semantico(0, 1, 1), 1);
    assert_eq!(checar_cubo_semantico(3, 1, 4), 3);
    assert_eq!(checar_cubo_semantico(0, 2, 0), 3);
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
