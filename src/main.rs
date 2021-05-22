extern crate compilador;
extern crate nom;

use compilador::parser::programa::*;

fn main() {
	// println!("{:?}", programa("programa idPrograma; principal(){}"));
	// println!("{:?}", programa("a"));
	// println!("{:?}", programa("
	// 	programa idPrograma;
		
	// 	clase Estudiante {
	// 		entero num;
	// 		void funcion agregar(entero n, entero m) {
	// 			entero o;
	// 		}
	// 	};

	// 	entero a;
	// 	principal() {
	// 	}"
	// ));
	println!("{:?}", programa("
		programa idPrograma;

		void funcion func (entero var) {
			entero i;
			id = 10;
			regresa expresion;
		}

		entero num;
		entero num;
		entero i;

		clase Persona {};

		clase Estudiante {
			char nombre[10], apellido[10];
		};

		principal() {
			lee(var);
			escribe(var);
			id();
			id(param);
			id.metodo();
			mientras ( id > 10 + 20 ) {
				escribe(id);
			}

			desde arr[10] = 10 hasta 20 {
				escribe(id);
			}
			%% comentario %%
			si (id > 2) {
				escribe(id);
			}
			si (id > 2) {
				escribe(id);
			} sino {
				escribe(id);
			}
		}"
	));
}
