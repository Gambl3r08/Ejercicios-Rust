use std::fs::File;
use std::io::Read;

fn read_a_file() -> std::io::Result<Vec<u8>>{
	let mut file = try!(File::open("example.data"));

	let mut data = Vec::new();
	try!(file.read_a_end(&mut data));

	return Ok(data);
}

fn main() {
	
}