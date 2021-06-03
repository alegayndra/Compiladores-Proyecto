//! Compilador del lenguaje _Killer Queen_
//! 
//! Utilizamos las librerias de _nom_ y _lazy static_, para parsear y crear variables estáticas en ejecución respectivamente.
//!
//! # Manual de usuario - Killer Queen
//! Manual de usuario sobre el lenguaje de programación _Killer Queen_
//! 
//! Todo el código que se genere tiene que pertenecer a un archivo de terminación `.eo`.
//! 
//! Nota: Cuando se haga alusión a una variable (nombre personalizable) en las explicaciones, se encontrará dicho texto rodeado de `<` `>`, dichos caracteres *NO* forman parte del código  
//! Por ejemplo: 
//! ```
//! entero < variable > ;
//! ```
//! 
//! Ya con la variable seleccionada la misma línea de código se vería así:  
//! ```
//! entero var1;
//! ```
//! 
//! ## Estructura básica
//! 
//! La estructura básica de un código del lenguaje es la siguiente:
//! 
//! ```
//! programa < id > ;
//! 
//! < declaraciones >
//! 
//! principal() {
//!   < estatutos >
//! }
//! ```
//! 
//! ### ID Programa
//! La parte inicial del código sirve para indiciar el nombre del programa.
//! 
//! ### Declaraciones
//! 
//! Dentro de la sección de `declaraciones` se pueden declarar variables globales y/o funciones.
//! 
//! #### Variables
//! 
//! La variables se declaran de la siguiente manera:
//! 
//! ```
//! < tipo > < id > < dimensiones > ;
//! ```
//! 
//! Donde el `tipo` y el `id` son requisitos, mientras que las `dimensiones` son opcionales.
//! 
//! Los tipos de variables existentes en el lenguaje son:
//! - entero
//! - flotante
//! - char
//! 
//! La estructura de las dimensiones es la siguiente
//! 
//! ```
//! [ < num entero > ] [ < num entero > ]
//! ```
//! 
//! También se pueden declarar varias variables en la misma linea de la siguiente manera: 
//! 
//! ```
//! < tipo > < id > < dimensiones > , < id > < dimensiones > , < id > < dimensiones > , ... < id > < dimensiones > ;
//! ```
//! 
//! ##### Ejemplos
//! ```
//! entero num;
//! flotante promedio;
//! char letra;
//! entero a, b, c, d;
//! char nombre[10];
//! ```
//! 
//! #### Funciones
//! 
//! Las funciones se declaran de la siguiente manera:
//! 
//! ```
//! < tipo > funcion < id > ( < parametros > ) { < estatutos > }
//! ```
//! 
//! Los tipos de variables existentes en el lenguaje son:
//! - entero
//! - flotante
//! - char
//! - void
//! 
//! Los parametros son atómicos y siguen la siguiente estructura:
//! 
//! ```
//! < tipo > < id > , < tipo > < id >, < tipo > < id > ... , < tipo > < id >
//! ```
//! 
//! Puede que una función no tenga parámetros.
//! 
//! ##### Ejemplos
//! ```
//! entero funcion suma(entero a, entero b) { < estatutos > }
//! flotante funcion multiplicacion(flotante a, flotante b) { < estatutos > }
//! void funcion imprimir() { < estatutos > }
//! ```
//! ### Expresiones
//! 
//! Cada expresión _< exp >_, _< expresion >_ genera derivaciones para ejecutar operaciones aritméticas, lógicas y relacionales. 
//! 
//! ### Estatutos
//! 
//! Los estatutos son las acciones de código que pertenecen dentro de una función y son las siguientes:
//! - Asignaciones
//! - Lectura
//! - Escritura
//! - Llamada de función
//! - Ciclos
//! - Condicionales
//! - Retornos
//! - Comentarios
//! 
//! Representa la "columna vertebral" del lenguaje, casi todas las acciones por ejecutar derivan de _estatuto_.
//! 
//! #### Asignaciones
//! 
//! Todas las asginaciones tienen la siguiente estructura:
//! ```
//! < id > = < exp >;
//! ```
//! Cada expresión _< exp >_ permite asignar el resultado obtenido a < id >. 
//! 
//! ##### Ejemplos
//! ```
//! num = 10;
//! promedio = 9.7;
//! letra = 'J';
//! a = b - d / c * 3;
//! nombre[1] = 'M';
//! ```
//! #### Lectura
//! La estructura para mostrar un mensaje en la consola es la siguiente.
//! 
//! ```
//!  lee ( < id >, < id >, < id > ... , < id > );
//! ```
//! ##### Ejemplo
//! ```
//! lee(num, promedio);
//! ```
//! #### Escritura
//! La estructura para escribir un mensaje en la consola es la siguiente.
//! 
//! ```
//!  escribe ( < texto >, < texto >, < texto > ... , < texto > );
//! ```
//! También se permiten que se escriba el resultado de una expresión siguiendo la estructura de
//! ```
//!  escribe ( < expresion > );
//! ```
//! 
//! ##### Ejemplos
//! ```
//! escribe(suma(2, 3, 4));
//! escribe("Hola mundo");
//! escribe(num);
//! ```
//! #### Llamada de función
//! La estructura para llamar una función es la siguiente.
//! 
//! ```
//! < id > ( < expresion >, < expresion > );
//! ```
//! Cada expresión va a representar el parámetro a enviar de dicha función.
//! También se permiten que se escriba el resultado de una expresión siguiendo la estructura de
//! 
//! ##### Ejemplos
//! ```
//! suma(2, 3, 4);
//! llenar_arreglo();
//! resta(9 - 4);
//! ```
//! #### Repetición
//! Hay dos estructura para hacer un ciclo, depende de si sigues un formato de _while loop_ o un formato de _for loop_. 
//! 
//! ##### While loop
//! ```
//! mientras ( < expresion > ){ < estatuto > }
//! ```
//! 
//! ##### For loop
//! ```
//! desde < id > = < exp > hasta < exp > { < estatuto > }
//! ```
//! Cada expresión va a representar el parámetro a enviar de dicha función.
//! También se permiten que se escriba el resultado de una expresión siguiendo la estructura de
//! 
//! ##### Ejemplos
//! ```
//! desde i = 10 hasta 20 {
//!     escribe(i);
//! }
//! mientras ( var > 1) {
//!     var = var - 1;
//!     acum = acum * (var);
//!   }
//! ```
//! #### Condicionales
//! La estructura para realizar una decisión en el lenguaje es la siguiente.
//! 
//! ```
//! si ( < expresion >) { 
//!   < estatuto > 
//! }
//! sino { < estatuto > }
//! ```
//! 
//! ##### Ejemplos
//! ```
//! si (var > 0) {
//!     regresa 1;
//! }
//! sino{
//!   regresa i;
//! }
//! ```
//! #### Retornos
//! La estructura para ejecutar un retorno de una función en el lenguaje es la siguiente.
//! ```
//! regresa < exp >;
//! ```
//! 
//! ##### Ejemplo
//! ```
//! regresa i;
//! ```
//! 
//! #### Comentarios
//! La estructura para mostrar un comentario es esta de una función en el lenguaje es la siguiente.
//! ```
//! %% < texto > %%
//! ```
//! Solo se permite hacer comentarios dentro de las funciones.
//! 
//! ##### Ejemplos
//! ```
//! %% entero i; %%
//! ```

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::fs;

pub mod scanners;
pub mod semantica;
pub mod parser;

use crate::parser::programa::*;
use crate::semantica::globales::*;

/// Escribe el archivo de salida.
///
/// # Ejemplo
///
/// ```ignore
/// escribir_archivo();
/// ```
fn escribir_archivo() {
	let arch = "Compilador/cuadruplos/killer_queen.txt";
	let path = Path::new(arch);

	let display = path.display();

	// Open a file in write-only mode, returns `io::Result<File>`
	let mut file = match File::create(&path) {
		Err(why) => panic!("couldn't create {}: {}", display, why),
		Ok(file) => file,
	};

	// Variables globales de semántica
	let tabla_funciones = FUNCIONES.lock().unwrap();
	let id_programa = ID_PROGRAMA.lock().unwrap();
	let constantes = CONSTANTES.lock().unwrap();
	let cuadruplos = CUADRUPLOS.lock().unwrap();
	// let tabla_clases = CLASES.lock().unwrap();

	let mut texto_archivo: String = "".to_owned();

	// Escritura constantes
	{
		let mut texto_constantes: String = "".to_owned();
	
		unsafe {
			let era_constantes = format!("({}, {}, {})", ERA_CONSTANTES.0, ERA_CONSTANTES.1, ERA_CONSTANTES.2);
			texto_constantes = format!("{}{}\n", texto_constantes, era_constantes);
		}
	
		for (_key, val) in constantes.tabla.iter() {
			let tipo_var = match val.tipo.as_str() {
				"texto" => "char",
				tipo => tipo
			}.to_owned();
			let const_string: String = format!("({}, {}, {})", val.nombre, val.direccion, tipo_var);
			texto_constantes = format!("{}{}\n", texto_constantes, const_string);
			// println!("key: {} val: {}", key, val);
		}
	
		texto_archivo = format!("{}CONSTANTES\n{}FIN_CONSTANTES\n", texto_archivo, texto_constantes);
	}

	// Escritura globales
	{
		let mut texto_globales: String = "".to_owned();
	
		match tabla_funciones.tabla.get(&id_programa.to_string()) {
			Some(vars) => {
				let mut globales_string: String = "".to_owned(); // Faltan dimensiones
				for tam in vars.era.iter() {
					let tam_string: String = format!("({}, {})", tam.0, tam.1);
					globales_string = format!("{}{}\n", globales_string, tam_string);
				}
				texto_globales = format!("{}{}", texto_globales, globales_string);
				()
			},
			None => ()
		}
	
		texto_archivo = format!("{}GLOBALES\n{}FIN_GLOBALES\n", texto_archivo, texto_globales);
	}

	// Escritura funciones
	{
		let mut texto_funciones: String = "".to_owned();
	
		for (key, val) in tabla_funciones.tabla.iter() {
			if key.to_owned() != id_programa.to_string() {
				let funcion_string: String = format!("({}, {}, {})", val.nombre, val.direccion, val.num_cuadruplo); // Faltan dimensiones
				texto_funciones = format!("{}{}\n", texto_funciones, funcion_string);
				let mut tamanio_string: String = "".to_owned(); // Faltan dimensiones
				for tam in val.era.iter() {
					let tam_string: String = format!("({}, {})", tam.0, tam.1);
					tamanio_string = format!("{}{}\n", tamanio_string, tam_string);
				}
				texto_funciones = format!("{}{}", texto_funciones, tamanio_string);
				let mut lista_parametros: String = "".to_owned();
				for param in val.parametros.iter() {
					let param_string: String = format!("({}, {})", param.direccion, param.tipo);
					lista_parametros = format!("{}{}\n", lista_parametros, param_string);
				}
				texto_funciones = format!("{}PARAMS\n{}FIN_PARAMS\n", texto_funciones, lista_parametros);
			}
		}
	
		texto_archivo = format!("{}FUNCIONES\n{}FIN_FUNCIONES\n", texto_archivo, texto_funciones);
	}

	// Escritura cuadruplos
	{
		let mut lista_cuadruplos: String = "".to_owned();
	
		for cuad in cuadruplos.lista.iter() {
			let cuad_string: String = format!("({}, {}, {}, {})", cuad.0, cuad.1, cuad.2, cuad.3);
			lista_cuadruplos = format!("{}{}\n", lista_cuadruplos, cuad_string);
		}
	
		texto_archivo = format!("{}CUADRUPLOS\n{}FIN_CUADRUPLOS\n", texto_archivo, lista_cuadruplos);
	}

	// Guardado de archivo
	match file.write_all(texto_archivo.as_bytes()) {
		Err(why) => panic!("couldn't write to {}: {}", display, why),
		Ok(_) => println!("successfully wrote to {}", display),
	}
}

/// Inicia todo el proceso de compilación.  
/// Lee el archivo de entrada, empieza el análisis del lenguaje y escribe el archivo de salida.  
///
/// # Ejemplo
///
/// ```ignore
/// iniciar_compilador();
/// ```
pub fn iniciar_compilador() {
  // Consigue las variables de ambiente
  let args: Vec<String> = env::args().collect();
  let nombre_archivo = &args[1];

  // Agrega al nombre del archivo la terminación .eo
  let arch = format!("{}.eo", nombre_archivo);

  // Lee archivo
  println!("Leyendo archivo {}", arch.clone());
  let contents = fs::read_to_string(&arch).expect("Something went wrong reading the file");
  println!("Archivo leído correctamente");

  // Analiza el código fuente dado y genera el archivo de sálida en caso de que sea un éxito
  match programa(&contents) {
    Ok(_) => {
      escribir_archivo();
    },
    Err(err) => {
      println!("{:?}", err);
    }
  };
}
