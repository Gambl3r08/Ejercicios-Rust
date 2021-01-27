use std::env;
use std::fs::File;
use std::io::Write;

fn main() {

	//Crear un archivo temporal
	let temp_directory = env::temp_dir();
	let temp_file = temp_directory.join("file");


	//Abrir un archivo en modo escritura 
	let mut file = File::create(temp_file).unwrap();
	// Escribir una referencia &str en el archivo
	writeln!(&mut file, "Hello world!").unwrap();

	// Escribir un byte string.
	file.write(b"Bytes\n").unwrap();
}