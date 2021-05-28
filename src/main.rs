extern crate compilador;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use compilador::parser::programa::*;
use compilador::semantica::globales::*;

fn escribir_archivo() {
	let path = Path::new("cuadruplos/killer_queen.txt");
	let display = path.display();

	// Open a file in write-only mode, returns `io::Result<File>`
	let mut file = match File::create(&path) {
		Err(why) => panic!("couldn't create {}: {}", display, why),
		Ok(file) => file,
	};

	let mut texto_archivo: String = "".to_owned();

	// Escritura constantes
	let constantes = CONSTANTES.lock().unwrap();
	let mut texto_constantes: String = "".to_owned();

	unsafe {
		let era_constantes = format!("({}, {}, {}, {})", ERA_CONSTANTES.0, ERA_CONSTANTES.1, ERA_CONSTANTES.2, ERA_CONSTANTES.3);
		texto_constantes = format!("{}{}\n", texto_constantes, era_constantes);
	}

	for (_key, val) in constantes.tabla.iter() {
		let const_string: String = format!("({}, {}, {})", val.nombre, val.direccion, val.tipo);
		texto_constantes = format!("{}{}\n", texto_constantes, const_string);
    // println!("key: {} val: {}", key, val);
	}

	texto_archivo = format!("{}CONSTANTES\n{}FIN_CONSTANTES\n", texto_archivo, texto_constantes);

	// Escritura globales
	let tabla_funciones = FUNCIONES.lock().unwrap();
	let id_programa = ID_PROGRAMA.lock().unwrap();
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

	// Escritura funciones
	let mut texto_funciones: String = "".to_owned();

	for (key, val) in tabla_funciones.tabla.iter() {
		if key.to_owned() != id_programa.to_string() {
			let funcion_string: String = format!("({}, {}, {})", val.nombre, val.direccion, val.tipo); // Faltan dimensiones
			texto_funciones = format!("{}{}\n", texto_funciones, funcion_string);
			let mut lista_parametros: String = "".to_owned();
			for param in val.parametros.iter() {
        let param_string: String = format!("({}, {}, {})", param.nombre, param.direccion, param.tipo);
				lista_parametros = format!("{}{}\n", lista_parametros, param_string);
    	}
			texto_funciones = format!("{}PARAMS\n{}FIN_PARAMS\n", texto_funciones, lista_parametros);
		}
	}

	texto_archivo = format!("{}FUNCIONES\n{}FIN_FUNCIONES\n", texto_archivo, texto_funciones);

	// Escritura clases
	let tabla_clases = CLASES.lock().unwrap();
	let mut texto_clases: String = "".to_owned();

	for (_key, val) in tabla_clases.tabla.iter() {
		let clase_string: String = format!("({}, {})", val.nombre, val.padre);
		texto_clases = format!("{}{}\n", texto_clases, clase_string);
		let mut texto_metodos: String = "".to_owned();
		for (_key_m, metodo) in val.metodos.tabla.iter() {
			let metodo_string: String = format!("({}, {}, {})", metodo.nombre, metodo.direccion, metodo.tipo);
			texto_metodos = format!("{}{}\n", texto_metodos, metodo_string);
			let mut lista_parametros: String = "".to_owned();
			for param in metodo.parametros.iter() {
				let param_string: String = format!("({}, {}, {})", param.nombre, param.direccion, param.tipo);
				lista_parametros = format!("{}{}\n", lista_parametros, param_string);
			}
			texto_metodos = format!("{}PARAMS\n{}FIN_PARAMS\n", texto_metodos, metodo_string);
		}
		texto_clases = format!("{}METODOS\n{}FIN_METODOS\n", texto_clases, texto_metodos);
	}

	texto_archivo = format!("{}CLASES\n{}FIN_CLASES\n", texto_archivo, texto_clases);

	// Escritura cuadruplos
	let cuadruplos = CUADRUPLOS.lock().unwrap();
	let mut lista_cuadruplos: String = "".to_owned();

	for cuad in cuadruplos.lista.iter() {
		let cuad_string: String = format!("({}, {}, {}, {})", cuad.0, cuad.1, cuad.2, cuad.3);
		lista_cuadruplos = format!("{}{}\n", lista_cuadruplos, cuad_string);
	}

	texto_archivo = format!("{}CUADRUPLOS\n{}FIN_CUADRUPLOS\n", texto_archivo, lista_cuadruplos);

	// Guardado de archivo
	match file.write_all(texto_archivo.as_bytes()) {
		Err(why) => panic!("couldn't write to {}: {}", display, why),
		Ok(_) => println!("successfully wrote to {}", display),
	}
}

fn main() {
	println!("{:?}", programa("
		programa idPrograma;

		entero num;
		entero i;
		char id;
		flotante promedio;

		principal() {
			num = 10 * 2;
			promedio = 10.1;
			%% id = \"a\"; %%
			%% comentario %%
			lee(i);
			lee(id);
			escribe(10);
			escribe(\"aaa\");
			si (10 & 10) {
				i = 11;
			}
			si (10 > 10) {
				i = 11;
			} sino {
				i = 12;
			}

			desde i = 10 hasta 20 {
				escribe(i);
			}
		}"
	));

	escribir_archivo();
}
