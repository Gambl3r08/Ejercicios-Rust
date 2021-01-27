use std::net::TcpStream;

fn main() {
	TcpStream::connect("192.168.0.1:3000");
	let direccion = "192.168.0.1:3000".to_string();
	TcpStream::connect(&*direccion);//convirtiendo direccion a &str

}