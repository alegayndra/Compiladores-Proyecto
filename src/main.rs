extern crate compilador;
extern crate nom;

use compilador::parser::func_esp::*;
use compilador::parser::variables::*;

fn main() {
	println!("{:?}", leer_parser("lee ( id )"));
	println!("{:?}", leer_parser("lee (id)"));
	println!("{:?}", leer_parser("lee(id)"));

	println!("{:?}", variables("entero id;"));
	println!("{:?}", variables("entero id, id, id;"));
	println!("{:?}", variables("id id, id, id;"));
	// println!("{:?}", leer_parser("lee ( id , id , id , id , id )"));
	// println!("{:?}", leer_parser("lee ( id , id , id , id , )"));

	// println!("{:?}", leer_parser("lee(id)"));
	// println!("{:?}", leer_parser("lee(id,id,id,id,id)"));
	// println!("{:?}", leer_parser("lee(id,id,id,id,)"));
}
