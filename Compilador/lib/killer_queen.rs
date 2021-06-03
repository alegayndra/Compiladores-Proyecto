//! Compilador del lenguaje _Killer Queen_
//! 
//! Utilizamos las librerias de _nom_ y _lazy static_, para parsear y crear variables estáticas en ejecución respectivamente.
//!
//! # Manual de usuario
//! Instrucciones sobre la creación de datos para el lenguaje de compilación _Killer Queen_
//! 
//! Todo el código que se genere tiene que pertenecer a un archivo de terminación `.eo`.
//! 
//! Cuando se haga alusión a una variable (nombre personalizable) mientras se explica, se encontrará dicho texto rodeado de `<` `>`, dichos caracteres *NO* forman parte del código  
//! Por ejemplo: 
//! ```
//! entero < variable > ;
//! ```
//! 
//! 
//! Ya con la variable seleccionada la misma línea de código se vería así:  
//! ```
//! entero var1;
//! ```
//! ## Instrucciones
//! 
//! ### Estructura
//! 
//! La primera línea de código debe ser
//! ```
//! programa < nombre > ;
//! ```
//! Para indicar el nombre del programa.
//! 
//! Si se quieren declarar variables globales eso es lo siguiente a declarar, primero se indica el tipo y luego se indica el nombre.  
//! Los tipos de datos existentes en el lenguaje son:
//! - entero
//! - flotante
//! - char
//! 
//! Y se pueden crear arreglos de estos también.
//! 
//! Un ejemplo de como se verían la declaración de variables es
//! ```
//! entero < variable >;
//! entero < variable >[< tamaño_arreglo >];
//! flotante < variable >;
//! char < variable >;
//! ```
//! 
//! Si se quiere crear funciones además de la función _principal()_ eso es lo siguiente.  
//! Primero se indica el tipo de retorno de la función si es que tiene usando las palabras reservadas _entero_, _flotante_, _char_, _void_.  
//! Luego se coloca la palabra reservada _funcion_, después un < nombre > y por último los _()_.
//! 
//! Lo anterior visto en código daría un ejemplo como este:
//! ```
//! void funcion < nombre >()
//! ```
//! 
//! Si se desean agregar parámetros a la función se tiene que indicar el tipo de dato y luego el nombre de la misma.  
//! Un ejemplo se puede ver así:
//! ```
//! char funcion < nombre >(flotante < variable_1 >, entero < variable_2 >)
//! ```
//! 
//! Por último todo el segmento de código de una función se tiene que encontrar rodeando entre **{ }**
//! 
//! Un ejemplo con todas las especificaciones de funciones se vería así:
//! ```
//! void funcion buscar(entero var){}
//! ```
//! 
//! Para correr el compilador y la máquina virtual, se necesita tener instalado el ambiente de desarrollo rust y de python. Una vez instalados, se siguen las siguientes instrucciones dentro de la carpeta principal.
//! 
//! Para correr el compilador se corre el siguiente comando:
//! 
//! ```bash
//! $ cargo run nombre_archivo
//! ```
//! 
//! Donde `nombre_archivo` es el nombre del archivo con el código a compilar sin la terminación de `.eo`. Por ejemplo, para compilar el acrhivo `sumas_y_restas.eo`, se debe correr `cargo run sumas_y_restas`.
//! 
//! También se tiene que crear un directorio llamado `cuadruplos` para que se pueda generar el archivo de sálida.
//! 
//! Una vez generado el archivo de sálida con el código intermedio, para poder ejectutarlo, se corre el siguiente comando para correr la máquina virtual:
//! 
//! ```bash
//! linux: 
//! $ python3 Maquina_Virtual/main.py
//! 
//! windows:
//! $ python Maquina_Virtual/main.py
//! ```
//! 
//! ### Pruebas
//! 
//! #### Pruebas unitarias
//! Para correr las pruebas unitarias dentro de Rust, se corre el siguiente comando:
//! 
//! ```bash
//! cargo test -- --test-threads=1
//! ```
//! 
//! #### Pruebas de compilador
//! Para correr los diferentes archivos de prueba, se corre `cargo run Pruebas/archivo`, donde `archivo` es el nombre del archivo deseado a correr. Luego, para correr la maquina virtual, se corre `python3 Maquina_Virtual/main.py`.
//! 
//! ### Generar documentación
//! 
//! Para generar y abrir la documentación corre `cargo doc --lib --open`.

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
