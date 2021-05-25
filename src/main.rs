extern crate compilador;
extern crate nom;

use compilador::parser::programa::*;

fn main() {
	println!("{:?}", programa("
		programa idPrograma;

		void funcion func (entero var) {
			entero i;
			i = 10;
			char j;
			lee(j);
			regresa 10 + i;
		}

		entero num;
		entero i;
		char id;
		flotante promedio;

		clase Estudiante {
			char nombre[10], apellido[10];
		};

		principal() {
			num = 10 * 2;
			promedio = 10.1;
			%% id = \"a\"; %%
			%% comentario %%
			lee(i);
			escribe(10);
			escribe(\"aaa\");
		}"
	));
}
