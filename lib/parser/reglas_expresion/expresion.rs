use nom::{
  IResult,
  sequence::{tuple, preceded},
  combinator::opt,
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::exp::*;
use crate::semantica::globales::*;

fn checar_lista_operadores() {
  let mut lista_operadores = PILA_OPERADORS.lock().unwrap();
  match lista_operadores.pop() {
    Some(op) => {
      match op_relacional(&op) {
        Ok(_) => {
          let mut pila_val = PILA_VALORES.lock().unwrap();
          let der = match pila_val.pop() {
            Some(val) => val,
            _ => {
              // println!("Stack de valores vacío en EXPRESION");
              return;
            }
          };
          let izq = match pila_val.pop() {
            Some(val) => val,
            _ => {
              // println!("Stack de valores vacío en EXPRESION");
              return;
            }
          };

          drop(pila_val);

          match CUADRUPLOS.lock().unwrap().agregar_cuadruplo(&op, izq, der) {
            Ok(_res) => { /*println!("{:?}", _res);*/ () },
            Err(_err) => { /*println!("{:?}", _err);*/ () },
          };
        },
        Err(_) => {
          lista_operadores.push(op);
          ()
        }
      }
      ()
    },
    _ => {
      // println!("Stack de operadores vacío en EXPRESION");
      ()
    }
  }
}

fn exp_extra(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  next = match opt(preceded(ws, op_relacional))(next) {
    Ok((next_input, Some(operador))) => {
      PILA_OPERADORS.lock().unwrap().push(operador.to_owned());
      next_input
    }
    Err(err) => return Err(err),
    Ok((next_input, None)) => next_input 
  };

  match opt(preceded(ws, exp))(next) {
    Ok((next_input, _)) => {
      checar_lista_operadores();
      Ok((next_input, "termino"))
    },
    Err(err) => Err(err)
  }
}

fn exp_opcional(input: &str) -> IResult<&str, &str> {
  match opt(exp_extra)(input) {
    Ok((next_input, Some(res))) => Ok((next_input, res)), 
    _ => Ok((input, "exp_opcional"))  
  }
}

pub fn expresion(input: &str) -> IResult<&str, &str> {
  tuple((exp, exp_opcional))(input)
  .map(|(next_input, _res)| {
    (next_input, "expresion")
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  // use nom::{
  //     error::{ErrorKind, VerboseError, VerboseErrorKind},
  //     Err,
  // };

  #[test]
  fn test_expresion() {
    assert_eq!(expresion("termino > termino"),                                        Ok(("", "expresion")));
    assert_eq!(expresion("termino"),                                                  Ok(("", "expresion")));
    assert_eq!(expresion("id + num_entero * id2 - num_entero - termino"),             Ok(("", "expresion")));
    assert_eq!(expresion("id + num_entero * id2 - num_entero - termino > id3"),       Ok(("", "expresion")));
    assert_eq!(expresion("( id + num_entero * id2 - num_entero - termino >= id3 )"),  Ok(("", "expresion")));
  }
}
