extern crate compilador;
// #[macro_use]
extern crate nom;

// use nom::*;
use compilador::lexer::*;

fn main() {
	// println!("{:?}", hello_parser("hello"));
	// println!("{:?}", hello_parser("hello world"));
	// println!("{:?}", hello_parser("goodbye hello again"));
	// println!("{:?}", sumsub_parser("goodbye hello again"));
	println!("{:?}", arit("+*"));
	println!("{:?}", arit("-/"));
	println!("{:?}", arit("aa"));
	println!("{:?}", arit("aasda"));
	println!("{:?}", arit("+-"));
	println!("{:?}", arit("+aaaaaaaa"));
	println!("{:?}", arit("+/"));
	println!("{:?}", arit("-/"));
	println!("{:?}", arit("-+"));
	println!("{:?}", arit("+/ada"));
	println!("{:?}", op_relacional_parser("<=<"));
	println!("{:?}", op_relacional_parser(">=<"));
	println!("{:?}", op_relacional_parser("=!==")); //Expected to fail
	println!("{:?}", op_relacional_parser("==!="));
	println!("{:?}", op_logica_parser("&|"));
	println!("{:?}", op_logica_parser("|&"));
	println!("{:?}", op_logica_parser("|&"));
	println!("{:?}", leer_parser("lee(id)"));
	println!("{:?}", leer_parser("lee(id,id,id,id,id)"));
	println!("{:?}", leer_parser("lee(id,id,id,id,)"));

}