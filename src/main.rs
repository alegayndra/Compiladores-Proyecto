extern crate compilador;
extern crate nom;

use compilador::parser::programa::*;

fn main() {
	println!("{:?}", programa("a"));
	println!("{:?}", programa("
	programa idPrograma;
	void funcion func (entero var): {
		estatuto;
		regresa expresion;
	}
	entero num;
	clase Estudiante <Persona> {
		char nombre[10], apellido[10];
	};
	principal() {
		lee(var);
		escribe(var);
		id();
		id(param);
		id.metodo();
		mientras ( id > 10 ) {
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
	}"));
}
