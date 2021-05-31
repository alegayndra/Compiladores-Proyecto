use nom::{
  branch::alt,
  bytes::complete::tag,
  IResult,
  sequence::{delimited, preceded}
};
  
use crate::scanners::ws::*;
use crate::scanners::operadores::*;
use crate::parser::reglas_expresion::valor::*;
use crate::parser::reglas_expresion::exp_logica::*;
use crate::semantica::globales::*;

fn checar_pila_operadores() {
  let mut lista_operadores = PILA_OPERADORS.lock().unwrap();
  match lista_operadores.pop() {
    Some(op) => {
      match op.as_str() {
        "(" => (),
        _ => {
          // println!("No se encontró ( al final del stack de operadores en FACTOR");
          lista_operadores.push(op);
          ()
        }
      }
    },
    None => { /*println!("Stack de operadores vacío en EXP_LOGICA");*/ () }
  };
  drop(lista_operadores);
}

fn retorna_expresion(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;

  next = match tag("(")(next) {
    Ok((next_input, _)) => {
      let mut lista_operadores = PILA_OPERADORS.lock().unwrap();
      lista_operadores.push("(".to_owned());
      drop(lista_operadores);
      
      next_input
    },
    Err(err) => return Err(err)
  };

  next = match delimited(ws, exp_logica, ws)(next) {
    Ok((next_input, _)) => next_input,
    Err(err) => return Err(err)
  };

  match tag(")")(next) {
    Ok((next_input, _)) => {
      checar_pila_operadores();
      Ok((next_input, "retorna_expresion"))
    },
    Err(err) => Err(err)
  }
}

fn op_vacio(input: &str) -> IResult<&str, &str> {
  Ok((input, ""))
}

fn checar_lista_operadores(op_valor: &str) {
  match op_valor {
    "-" => {
      let mut pila_val = PILA_VALORES.lock().unwrap();
      let valor = match pila_val.pop() {
        Some(val) => val,
        _ => {
          println!("Stack de valores vacío en VALOR_FACTOR");
          return
        }
      };

      drop(pila_val);

      match CUADRUPLOS.lock().unwrap().agregar_cuadruplo(op_valor, valor.clone(), valor.clone()) {
        Ok(res) => {
          println!("{:?}", res);
          ()
        },
        Err(err) => {
          println!("{:?}", err);
          ()
        }
      };
      ()
    }
    _ => ()
  };
}

fn valor_factor(input: &str) -> IResult<&str, &str> {
  let mut next : &str = input;
  let op_valor: &str;

  next = match alt((op_sumsub, op_vacio))(next) {
    Ok((next_input, op)) => {
      op_valor = op;
      next_input
    },
    Err(err) => return Err(err)
  };

  match preceded(ws, valor)(next) {
    Ok((next_input, _valor)) => {
      checar_lista_operadores(op_valor);
      Ok((next_input, "valor_factor")) 
    },
    Err(err) => Err(err),
  }
}

pub fn factor(input: &str) -> IResult<&str, &str> {
  alt((retorna_expresion, valor_factor))(input)
  .map(|(next_input, _)| {
    (next_input, "factor")
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
  fn test_valor_factor() {
    assert_eq!(valor_factor("10"),                   Ok(("", "valor_factor")));
    assert_eq!(valor_factor("- 10"),                 Ok(("", "valor_factor")));
    assert_eq!(valor_factor("+ \"s\""),              Ok(("", "valor_factor")));
    assert_eq!(valor_factor("+ Nombre.metodo()"),    Ok(("", "valor_factor")));
    assert_eq!(valor_factor("+ Nombre . metodo()"),  Ok(("", "valor_factor")));
  }

  #[test]
  fn test_factor() {
    assert_eq!(factor("- num_entero"),        Ok(("", "factor")));
    assert_eq!(factor("+ \"s\""),             Ok(("", "factor")));
    assert_eq!(factor("+ Nombre . metodo()"), Ok(("", "factor")));
    assert_eq!(factor("( expresion )"),       Ok(("", "factor")));
    assert_eq!(factor("( 10 )"),              Ok(("", "factor")));
    assert_eq!(factor("( 10 * id )"),         Ok(("", "factor")));
    assert_eq!(factor("( 11 & id )"),         Ok(("", "factor")));
    assert_eq!(factor("( 1 | 0 )"),           Ok(("", "factor")));
  }
}
