extern crate compilador;
extern crate nom;

use compilador::parser::func_esp::*;

fn main() {
	println!("{:?}", leer_parser("lee ( id )"));
	println!("{:?}", leer_parser("lee (id)"));
	println!("{:?}", leer_parser("lee(id)"));
	// println!("{:?}", leer_parser("lee ( id , id , id , id , id )"));
	// println!("{:?}", leer_parser("lee ( id , id , id , id , )"));

	// println!("{:?}", leer_parser("lee(id)"));
	// println!("{:?}", leer_parser("lee(id,id,id,id,id)"));
	// println!("{:?}", leer_parser("lee(id,id,id,id,)"));

}